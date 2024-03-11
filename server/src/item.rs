use super::lua_value::{table_remove, Table, Value};
use flexstr::{local_fmt, LocalStr};
use hex::{FromHex, ToHex};
use std::{cmp::min, rc::Rc};

#[derive(PartialEq, Eq, Hash)]
pub struct Item {
    pub name: LocalStr,
    pub nbt_hash: Option<[u8; 16]>,
    #[cfg(feature = "plethora")]
    pub damage: i16,
}

const NBT_HASH_KEY: &'static str = if cfg!(feature = "plethora") { "nbtHash" } else { "nbt" };

fn remove_nbt_hash(table: &mut Table) -> Result<Option<[u8; 16]>, LocalStr> {
    table
        .remove(&NBT_HASH_KEY.into())
        .map(LocalStr::try_from)
        .transpose()?
        .map(|x| <_>::from_hex(&*x))
        .transpose()
        .map_err(|e| local_fmt!("invalid nbt-hash: {}", e))
}

impl Item {
    pub fn parse_part(table: &mut Table) -> Result<Rc<Self>, LocalStr> {
        Ok(Rc::new(Self {
            name: table_remove(table, "name")?,
            nbt_hash: remove_nbt_hash(table)?,
            #[cfg(feature = "plethora")]
            damage: table_remove(table, "damage")?,
        }))
    }

    pub fn encode(&self, table: &mut Table) {
        table.insert("name".into(), self.name.clone().into());
        if let Some(nbt_hash) = self.nbt_hash {
            table.insert(NBT_HASH_KEY.into(), nbt_hash.encode_hex::<LocalStr>().into());
        }
        #[cfg(feature = "plethora")]
        table.insert("damage".into(), self.damage.into());
    }
}

pub struct ItemStack {
    pub item: Rc<Item>,
    pub size: i32,
}

impl ItemStack {
    fn parse_part(table: &mut Table) -> Result<Self, LocalStr> {
        Ok(Self { item: Item::parse_part(table)?, size: table_remove(table, "count")? })
    }

    pub fn parse(value: Value) -> Result<Self, LocalStr> {
        let mut table = Table::try_from(value)?;
        let result = Self::parse_part(&mut table)?;
        if table.is_empty() {
            Ok(result)
        } else {
            Err(local_fmt!("unexpected fields: {:?}", table))
        }
    }

    pub fn with_detail(self, detail: Rc<Detail>) -> DetailStack {
        DetailStack { item: self.item, size: self.size, detail }
    }
}

pub struct Detail {
    pub label: LocalStr,
    pub max_size: i32,
    pub others: Table,
}

impl Detail {
    pub fn parse(mut table: Table) -> Result<Rc<Self>, LocalStr> {
        let label = table_remove(&mut table, "displayName")?;
        let max_size = table_remove(&mut table, "maxCount")?;
        Ok(Rc::new(Self { label, max_size, others: table }))
    }

    pub fn encode(&self) -> Table {
        let mut table = self.others.clone();
        table.insert("displayName".into(), self.label.clone().into());
        table.insert("maxCount".into(), self.max_size.into());
        table
    }
}

#[derive(Clone)]
pub struct DetailStack {
    pub item: Rc<Item>,
    pub detail: Rc<Detail>,
    pub size: i32,
}

impl DetailStack {
    pub fn parse(mut table: Table) -> Result<Self, LocalStr> {
        let stack = ItemStack::parse_part(&mut table)?;
        Ok(stack.with_detail(Detail::parse(table)?))
    }
}

#[derive(Clone)]
pub enum Filter {
    Label(LocalStr),
    Name(LocalStr),
    Both { label: LocalStr, name: LocalStr },
    Custom { desc: LocalStr, func: Rc<dyn Fn(&Item, &Detail) -> bool> },
}

impl Filter {
    pub fn apply(&self, item: &Item, detail: &Detail) -> bool {
        match self {
            Filter::Label(label) => detail.label == *label,
            Filter::Name(name) => item.name == *name,
            Filter::Both { label, name } => detail.label == *label && item.name == *name,
            Filter::Custom { func, .. } => func(item, detail),
        }
    }
}

pub fn jammer() -> DetailStack {
    thread_local!(static STACK: DetailStack = DetailStack {
        size: 1,
        item: Rc::new(Item { name: <_>::default(), nbt_hash: None, #[cfg(feature = "plethora")] damage: 0 }),
        detail: Rc::new(Detail { label: <_>::default(), max_size: 1, others: Table::new() })
    });
    STACK.with(|stack| stack.clone())
}

pub struct InsertPlan {
    pub n_inserted: i32,
    pub insertions: Vec<(usize, i32)>,
}

pub fn insert_into_inventory(
    inventory: &mut Vec<Option<DetailStack>>,
    item: &Rc<Item>,
    detail: &Rc<Detail>,
    to_insert: i32,
) -> InsertPlan {
    let mut result = InsertPlan { n_inserted: 0, insertions: Vec::new() };
    let mut remaining = min(to_insert, detail.max_size);
    let mut first_empty_slot = None;
    for (slot, stack) in inventory.iter_mut().enumerate() {
        if remaining <= 0 {
            return result;
        }
        if let Some(stack) = stack {
            if stack.item == *item {
                let to_insert = min(remaining, detail.max_size - stack.size);
                if to_insert > 0 {
                    stack.size += to_insert;
                    result.n_inserted += to_insert;
                    result.insertions.push((slot, to_insert));
                    remaining -= to_insert
                }
            }
        } else if first_empty_slot.is_none() {
            first_empty_slot = Some(slot)
        }
    }
    if remaining > 0 {
        if let Some(slot) = first_empty_slot {
            inventory[slot] = Some(DetailStack { item: item.clone(), detail: detail.clone(), size: remaining });
            result.n_inserted += remaining;
            result.insertions.push((slot, remaining))
        }
    }
    result
}
