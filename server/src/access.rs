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

pub struct BasicAccess {
    pub client: &'static str,
    pub addr: &'static str,
}

impl_get_client!(BasicAccess);
impl GetAddr for BasicAccess {
    fn get_addr(&self) -> &'static str { self.addr }
}

pub struct BusAccess {
    pub client: &'static str,
    pub inv_addr: &'static str,
    pub bus_addr: &'static str,
}

impl_get_client!(BusAccess);
impl GetAddr for BusAccess {
    fn get_addr(&self) -> &'static str { self.inv_addr }
}
