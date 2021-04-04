pub trait Access {
    fn get_client(&self) -> &str;
    fn get_addr(&self) -> &'static str;
}

pub struct BasicAccess {
    pub client: &'static str,
    pub addr: &'static str,
}

impl Access for BasicAccess {
    fn get_client(&self) -> &str { self.client }
    fn get_addr(&self) -> &'static str { self.addr }
}
