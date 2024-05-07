use super::access::{GetAddr, GetClient};
use super::action::{ActionFuture, Call};
use super::detail_cache::{DetailCache, DetailResult};
use super::item::{DetailStack, ItemStack};
use super::lua_value::{call_result, table_to_vec, Value};
use super::server::Server;
use super::util::{alive, join_pair, join_tasks, spawn};
use flexstr::LocalStr;
use std::{
    cell::RefCell,
    future::Future,
    rc::{Rc, Weak},
};

pub trait Inventory: 'static {
    type Access: GetClient + GetAddr;
    fn get_weak(&self) -> &Weak<RefCell<Self>>;
    fn get_server(&self) -> &Rc<RefCell<Server>>;
    fn get_detail_cache(&self) -> &Rc<RefCell<DetailCache>>;
    fn get_accesses(&self) -> &Vec<Self::Access>;
    fn get_size(&self) -> &Option<usize>;
    fn set_size(&mut self, size: usize);
}

macro_rules! impl_inventory {
    ($i:ident, $a:ident) => {
        impl Inventory for $i {
            type Access = $a;
            fn get_weak(&self) -> &Weak<RefCell<Self>> { &self.weak }
            fn get_server(&self) -> &Rc<RefCell<Server>> { &self.server }
            fn get_detail_cache(&self) -> &Rc<RefCell<DetailCache>> { &self.detail_cache }
            fn get_accesses(&self) -> &Vec<Self::Access> { &self.config.accesses }
            fn get_size(&self) -> &Option<usize> { &self.size }
            fn set_size(&mut self, size: usize) { self.size = Some(size) }
        }
    };
}

fn fetch_detail<T: Inventory>(this: &T, slot: usize) -> impl Future<Output = Result<DetailStack, LocalStr>> {
    let server = this.get_server().borrow();
    let access = server.load_balance(this.get_accesses());
    let action = ActionFuture::from(Call {
        addr: access.get_addr().clone(),
        args: vec![if cfg!(feature = "plethora") { "getItemMeta" } else { "getItemDetail" }.into(), (slot + 1).into()],
    });
    server.enqueue_request_group(access.get_client(), vec![action.clone().into()]);
    async move { DetailStack::parse(call_result(action.await?)?) }
}

fn fetch_detail_list<T: Inventory>(this: &T) -> impl Future<Output = Result<Vec<Option<DetailStack>>, LocalStr>> {
    let server = this.get_server().borrow();
    let access = server.load_balance(this.get_accesses());
    let action = ActionFuture::from(Call { addr: access.get_addr().clone(), args: vec!["list".into()] });
    server.enqueue_request_group(access.get_client(), vec![action.clone().into()]);
    let weak = this.get_weak().clone();
    async move {
        let stacks = table_to_vec(call_result(action.await?)?)?;
        let mut details = Vec::new();
        details.resize_with(stacks.len(), || None);
        let details = Rc::new(RefCell::new(details));
        let mut tasks = Vec::new();
        {
            alive!(weak, this);
            let mut details_ref = details.borrow_mut();
            let mut detail_cache = this.get_detail_cache().borrow_mut();
            for (slot, stack) in stacks.into_iter().enumerate() {
                if let Value::N = stack {
                    continue;
                }
                let stack = ItemStack::parse(stack)?;
                match detail_cache.query(&stack.item) {
                    DetailResult::Resolved(x) => details_ref[slot] = Some(stack.with_detail(x.clone())),
                    DetailResult::Resolving { sender, receiver } => {
                        let details = details.clone();
                        tasks.push(spawn(async move {
                            details.borrow_mut()[slot] = Some(stack.with_detail(receiver.await?));
                            Ok(())
                        }));
                        if let Some(sender) = sender {
                            let detail = fetch_detail(this, slot);
                            tasks.push(spawn(async move {
                                sender.send(detail.await.map(|stack| (stack.item, stack.detail)));
                                Ok(())
                            }))
                        }
                    }
                }
            }
        }
        join_tasks(tasks).await?;
        Ok(Rc::into_inner(details).unwrap().into_inner())
    }
}

fn fetch_size<T: Inventory>(this: &T) -> impl Future<Output = Result<usize, LocalStr>> {
    let server = this.get_server().borrow();
    let access = server.load_balance(this.get_accesses());
    let action = ActionFuture::from(Call { addr: access.get_addr().clone(), args: vec!["size".into()] });
    server.enqueue_request_group(access.get_client(), vec![action.clone().into()]);
    let weak = this.get_weak().clone();
    async move {
        let size = call_result(action.await?)?;
        alive(&weak)?.borrow_mut().set_size(size);
        Ok(size)
    }
}

pub fn list_inventory<T: Inventory>(this: &T) -> impl Future<Output = Result<Vec<Option<DetailStack>>, LocalStr>> {
    let stacks = fetch_detail_list(this);
    let size = this.get_size().ok_or_else(|| fetch_size(this));
    async move {
        let (mut stacks, size) = match size {
            Ok(size) => (stacks.await?, size),
            Err(size) => join_pair(stacks, size).await?,
        };
        stacks.resize_with(size, || None);
        Ok(stacks)
    }
}
