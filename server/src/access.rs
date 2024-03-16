use flexstr::LocalStr;

pub trait GetClient {
    fn get_client(&self) -> &str;
}

macro_rules! impl_get_client {
    ($a:ident) => {
        impl GetClient for $a {
            fn get_client(&self) -> &str { &*self.client }
        }
    };
}

pub trait GetAddr {
    fn get_addr(&self) -> &LocalStr;
}

impl_get_client!(BasicAccess);
pub struct BasicAccess {
    pub client: LocalStr,
    pub addr: LocalStr,
}

impl GetAddr for BasicAccess {
    fn get_addr(&self) -> &LocalStr { &self.addr }
}

impl_get_client!(BusAccess);
pub struct BusAccess {
    pub client: LocalStr,
    pub inv_addr: LocalStr,
    pub bus_addr: LocalStr,
}

impl GetAddr for BusAccess {
    fn get_addr(&self) -> &LocalStr { &self.inv_addr }
}

impl_get_client!(RedstoneAccess);
pub struct RedstoneAccess {
    pub client: LocalStr,
    pub addr: Option<LocalStr>,
    pub side: LocalStr,
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
    pub client: LocalStr,
    pub non_consumable_addr: LocalStr,
    pub turtle_addr: LocalStr,
    pub bus_addr: LocalStr,
}

impl_get_client!(MultiInvAccess);
pub struct MultiInvAccess {
    pub client: LocalStr,
    pub inv_addrs: Vec<LocalStr>,
    pub bus_addr: LocalStr,
}

impl_get_client!(FluidAccess);
pub struct FluidAccess {
    pub client: LocalStr,
    pub fluid_bus_addrs: Vec<LocalStr>,
}

impl_get_client!(TankAccess);
pub struct TankAccess {
    pub client: LocalStr,
    pub tank_addr: LocalStr,
    pub fluid_bus_addrs: Vec<LocalStr>,
}

impl_get_client!(InvTankAccess);
pub struct InvTankAccess {
    pub client: LocalStr,
    pub inv_addrs: Vec<LocalStr>,
    pub tank_addrs: Vec<LocalStr>,
    pub bus_addr: LocalStr,
    pub fluid_bus_addrs: Vec<LocalStr>,
}
