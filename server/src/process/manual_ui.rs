use super::{scattering_insert, IntoProcess, Inventory, Process};
use crate::inventory::list_inventory;
use crate::item::{insert_into_inventory, InsertPlan};
use crate::util::{alive, join_tasks, spawn};
use crate::{access::BusAccess, detail_cache::DetailCache, factory::Factory, item::DetailStack, server::Server, Tui};
use abort_on_drop::ChildTask;
use flexstr::LocalStr;
use futures_util::future::OptionFuture;
use ratatui::style::{Color, Modifier, Style};
use ratatui::text::{Line, Span};
use regex::Regex;
use std::{
    cell::RefCell,
    rc::{Rc, Weak},
};

pub struct ManualUiConfig {
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

fn make_pred(needle: &str) -> Box<dyn Fn(&DetailStack) -> bool> {
    if needle.is_empty() {
        Box::new(|_| true)
    } else if needle.starts_with("=") {
        let Ok(regex) = Regex::new(&needle[1..]) else { return Box::new(|_| false) };
        Box::new(move |x| regex.is_match(&x.item.name))
    } else {
        let Ok(regex) = Regex::new(needle) else { return Box::new(|_| false) };
        Box::new(move |x| regex.is_match(&x.detail.label))
    }
}

impl ManualUiProcess {
    fn update_view(&self, tui: &Tui) {
        let text_area = tui.text_area.borrow();
        let mut needle = text_area.lines().get(text_area.cursor().0).map(|x| x.as_str()).unwrap_or("");
        if let Some(pos) = needle.rfind('*') {
            needle = &needle[..pos]
        }
        let pred = make_pred(needle);
        tui.set_main_list(
            (self.latest_view.iter().filter(|x| pred(x)))
                .map(|x| {
                    Line::from(vec![
                        Span::raw(format!("{} * ", x.size)),
                        Span::styled(format!("{} ", x.detail.label), Color::LightGreen),
                        Span::styled(x.item.name.to_std_string(), Style::from(Color::Gray).add_modifier(Modifier::DIM)),
                    ])
                })
                .collect(),
        );
        tui.request_redraw()
    }
}

impl Process for ManualUiProcess {
    fn run(&self, _: &Factory) -> ChildTask<Result<(), LocalStr>> {
        let stacks = (!self.config.accesses.is_empty()).then(|| list_inventory(self));
        let weak = self.weak.clone();
        spawn(async move {
            let mut stacks = OptionFuture::from(stacks).await.transpose()?.unwrap_or_default();
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
                    let pred = make_pred(&request[..pos]);
                    let Some(stack) = this.latest_view.iter().find(|x| pred(x)) else { continue };
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
                        let reservation = factory.reserve_item("manual", &stack.item, n_inserted);
                        tasks.push(scattering_insert(this, factory, reservation, insertions));
                        size -= n_inserted
                    }
                }
            }
            join_tasks(tasks).await
        })
    }
}
