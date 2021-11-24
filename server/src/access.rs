pub trait GetClient {
    fn get_client(&self) -> &str;
}

macro_rules! impl_get_client {
    ($a:ident) => {
        impl GetClient for $a {
            fn get_client(&self) -> &str { self.client }
        }
    };
}

pub trait GetAddr {
    fn get_addr(&self) -> &'static str;
}

impl_get_client!(BasicAccess);
pub struct BasicAccess {
    pub client: &'static str,
    pub addr: &'static str,
}

impl GetAddr for BasicAccess {
    fn get_addr(&self) -> &'static str { self.addr }
}

impl_get_client!(BusAccess);
pub struct BusAccess {
    pub client: &'static str,
    pub inv_addr: &'static str,
    pub bus_addr: &'static str,
}

impl GetAddr for BusAccess {
    fn get_addr(&self) -> &'static str { self.inv_addr }
}

impl_get_client!(RedstoneAccess);
pub struct RedstoneAccess {
    pub client: &'static str,
    pub addr: Option<&'static str>,
    pub side: &'static str,
    pub bit: Option<u8>, // for bundled cable only
}

pub const BOTTOM: &str = "bottom";
pub const TOP: &str = "top";
pub const BACK: &str = "back";
pub const FRONT: &str = "front";
pub const RIGHT: &str = "right";
pub const LEFT: &str = "left";
pub const NORTH: &str = "north";
pub const SOUTH: &str = "south";
pub const WEST: &str = "west";
pub const EAST: &str = "east";

impl_get_client!(CraftyAccess);
pub struct CraftyAccess {
    pub client: &'static str,
    pub non_consumable_addr: &'static str,
    pub turtle_addr: &'static str,
    pub bus_addr: &'static str,
}
