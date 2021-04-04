use super::lua_value::{Table, Value};
use hex::FromHex;
use std::{
    convert::{TryFrom, TryInto},
    rc::Rc,
};

#[derive(PartialEq, Eq, Hash)]
pub struct Item {
    pub name: String,
    pub nbt_hash: Option<[u8; 16]>,
}

pub struct ItemStack {
    pub item: Rc<Item>,
    pub size: i32,
}

fn get<T: TryFrom<Value, Error = String>>(table: &mut Table, key: &str) -> Result<T, String> {
    table.remove(&key.into()).ok_or_else(|| format!("key not found: {}", key))?.try_into()
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
        let name = get(table, "name")?;
        let size = get(table, "count")?;
        let nbt_hash = get_nbt_hash(table)?;
        Ok(Self { item: Rc::new(Item { name, nbt_hash }), size })
    }

    pub fn parse(value: Value) -> Result<Self, String> {
        let mut table = value.try_into()?;
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
        let label = get(&mut table, "displayName")?;
        let max_size = get(&mut table, "maxCount")?;
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
