use super::lua_value::{table_remove, Table, Value};
use hex::FromHex;
use std::{cmp::min, convert::TryFrom, rc::Rc};

#[derive(PartialEq, Eq, Hash)]
pub struct Item {
    pub name: String,
    pub nbt_hash: Option<[u8; 16]>,
}

pub struct ItemStack {
    pub item: Rc<Item>,
    pub size: i32,
}

fn get_nbt_hash(table: &mut Table) -> Result<Option<[u8; 16]>, String> {
    table
        .remove(&"nbt".into())
        .map(String::try_from)
        .transpose()?
        .as_ref()
        .map(<_>::from_hex)
        .transpose()
        .map_err(|e| format!("invalid nbt-hash: {}", e))
}

impl ItemStack {
    fn parse_ref(table: &mut Table) -> Result<Self, String> {
        let name = table_remove(table, "name")?;
        let size = table_remove(table, "count")?;
        let nbt_hash = get_nbt_hash(table)?;
        Ok(Self { item: Rc::new(Item { name, nbt_hash }), size })
    }

    pub fn parse(value: Value) -> Result<Self, String> {
        let mut table = Table::try_from(value)?;
        let result = Self::parse_ref(&mut table)?;
        if table.is_empty() {
            Ok(result)
        } else {
            Err(format!("unexpected fields: {:?}", table))
        }
    }

    pub fn with_detail(self, detail: Rc<Detail>) -> DetailStack {
        DetailStack { item: self.item, size: self.size, detail }
    }
}

pub struct Detail {
    pub label: String,
    pub max_size: i32,
    pub others: Table,
}

#[derive(Clone)]
pub struct DetailStack {
    pub item: Rc<Item>,
    pub detail: Rc<Detail>,
    pub size: i32,
}

impl DetailStack {
    pub fn parse(mut table: Table) -> Result<Self, String> {
        let stack = ItemStack::parse_ref(&mut table)?;
        let label = table_remove(&mut table, "displayName")?;
        let max_size = table_remove(&mut table, "maxCount")?;
        Ok(stack.with_detail(Rc::new(Detail { label, max_size, others: table })))
    }
}

pub enum Filter {
    Label(&'static str),
    Name(&'static str),
    Both { label: &'static str, name: &'static str },
    Fn(Box<dyn Fn(&Item, &Detail) -> bool>),
}

impl Filter {
    pub fn apply(&self, item: &Item, detail: &Detail) -> bool {
        match self {
            Filter::Label(label) => detail.label == *label,
            Filter::Name(name) => item.name == *name,
            Filter::Both { label, name } => detail.label == *label && item.name == *name,
            Filter::Fn(filter) => filter(item, detail),
        }
    }
}

pub fn jammer() -> DetailStack {
    thread_local!(static STACK: DetailStack = DetailStack {
        size: 1,
        item: Rc::new(Item { name: String::new(), nbt_hash: None }),
        detail: Rc::new(Detail { label: String::new(), max_size: 1, others: Table::new() })
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
