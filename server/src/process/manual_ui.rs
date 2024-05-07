use super::{scattering_insert, IntoProcess, Inventory, Process};
use crate::inventory::list_inventory;
use crate::item::{insert_into_inventory, InsertPlan};
use crate::util::{alive, join_tasks, spawn};
use crate::{access::BusAccess, detail_cache::DetailCache, factory::Factory, item::DetailStack, server::Server, Tui};
use abort_on_drop::ChildTask;
use flexstr::LocalStr;
use ratatui::widgets::ListItem;
use std::{
    cell::RefCell,
    rc::{Rc, Weak},
};

pub struct ManualUiConfig {
    pub name: LocalStr,
    pub accesses: Vec<BusAccess>,
}

pub struct ManualUiProcess {
    weak: Weak<RefCell<ManualUiProcess>>,
    config: ManualUiConfig,
    detail_cache: Rc<RefCell<DetailCache>>,
    factory: Weak<RefCell<Factory>>,
    server: Rc<RefCell<Server>>,
    size: Option<usize>,
    latest_view: Vec<DetailStack>,
    _input_handler: ChildTask<()>,
}

impl_inventory!(ManualUiProcess, BusAccess);

impl IntoProcess for ManualUiConfig {
    type Output = ManualUiProcess;
    fn into_process(self, factory: &Factory) -> Rc<RefCell<Self::Output>> {
        Rc::new_cyclic(|weak| {
            let tui = factory.config.tui.clone();
            let weak = weak.clone();
            RefCell::new(Self::Output {
                weak: weak.clone(),
                config: self,
                detail_cache: factory.get_detail_cache().clone(),
                factory: factory.get_weak().clone(),
                server: factory.get_server().clone(),
                size: None,
                latest_view: Vec::new(),
                _input_handler: spawn(async move { input_handler(tui, weak).await }),
            })
        })
    }
}

async fn input_handler(tui: Rc<Tui>, weak: Weak<RefCell<ManualUiProcess>>) {
    loop {
        tui.on_input.notified().await;
        let Some(this) = weak.upgrade() else { break };
        let this = this.borrow();
        this.update_view(&this.factory.upgrade().unwrap().borrow().config.tui);
    }
}

fn search_pred(needle: &str, stack: &DetailStack) -> bool { needle.is_empty() || stack.detail.label.contains(needle) }

impl ManualUiProcess {
    fn update_view(&self, tui: &Tui) {
        let text_area = tui.text_area.borrow();
        let mut needle = text_area.lines().get(text_area.cursor().0).map(|x| x.as_str()).unwrap_or("");
        if let Some(pos) = needle.rfind('*') {
            needle = &needle[..pos]
        }
        *tui.main_list.borrow_mut() = (self.latest_view.iter())
            .filter(|x| search_pred(needle, x))
            .map(|x| ListItem::new(format!("{} * {}", x.size, x.detail.label)))
            .collect();
        tui.request_redraw()
    }
}

impl Process for ManualUiProcess {
    fn run(&self, _: &Factory) -> ChildTask<Result<(), LocalStr>> {
        let stacks = list_inventory(self);
        let weak = self.weak.clone();
        spawn(async move {
            let mut stacks = stacks.await?;
            let mut tasks = Vec::new();
            {
                alive_mut!(weak, this);
                upgrade_mut!(this.factory, factory);
                this.latest_view = Vec::from_iter(factory.items.iter().map(|(item, info)| {
                    let info = info.borrow();
                    DetailStack { item: item.clone(), detail: info.detail.clone(), size: info.n_stored }
                }));
                this.latest_view.sort_by_key(|x| -x.size);
                let tui = factory.config.tui.clone();
                this.update_view(&tui);
                for request in tui.input_queue.borrow_mut().drain(..) {
                    let Some(pos) = request.rfind('*') else { continue };
                    let needle = &request[..pos];
                    let Some(stack) = this.latest_view.iter().find(|x| search_pred(needle, x)) else { continue };
                    let Ok(mut size) = request[pos + 1..].parse() else { continue };
                    size = stack.size.min(size);
                    loop {
                        let InsertPlan { n_inserted, insertions } = insert_into_inventory(
                            &mut stacks,
                            &stack.item,
                            &stack.detail,
                            size.min(stack.detail.max_size),
                        );
                        if n_inserted <= 0 {
                            break;
                        };
                        let reservation = factory.reserve_item(&this.config.name, &stack.item, n_inserted);
                        tasks.push(scattering_insert(this, factory, reservation, insertions));
                        size -= n_inserted
                    }
                }
            }
            join_tasks(tasks).await
        })
    }
}
