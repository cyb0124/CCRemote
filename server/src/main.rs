#[macro_use]
pub mod util;
#[macro_use]
pub mod inventory;
#[macro_use]
pub mod recipe;
#[macro_use]
pub mod config_util;
pub mod access;
pub mod action;
pub mod config;
pub mod detail_cache;
pub mod factory;
pub mod item;
pub mod lua_value;
pub mod process;
pub mod server;
pub mod storage;
pub mod turtle_rc;

use config::build_factory;
use crossterm::{
    event::{Event, EventStream},
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
    ExecutableCommand,
};
use futures_util::StreamExt;
use ratatui::{
    backend::CrosstermBackend,
    layout::{Constraint, Layout},
    style::Color,
    widgets::{List, ListItem},
    Frame, Terminal,
};
use std::{cell::RefCell, collections::VecDeque, io::stdout, rc::Rc};
use tokio::{select, sync::Notify, task::LocalSet};
use tui_textarea::{CursorMove, Input, Key, TextArea};

#[derive(Default)]
pub struct Tui {
    on_redraw: Notify,
    on_input: Notify,
    logs: RefCell<VecDeque<ListItem<'static>>>,
    inputs: RefCell<Vec<String>>,
    text_area: RefCell<TextArea<'static>>,
}

impl Tui {
    fn request_redraw(&self) { self.on_redraw.notify_one() }
    fn log(&self, msg: String, color: u8) {
        let color = match color {
            0 => Color::Reset,
            1 => Color::LightYellow,
            3 => Color::LightBlue,
            6 => Color::LightRed,
            10 => Color::LightMagenta,
            13 => Color::Green,
            14 => Color::Red,
            _ => unreachable!(),
        };
        self.logs.borrow_mut().push_back(ListItem::new(msg).style(color));
        self.request_redraw()
    }

    fn frame(&self, frame: &mut Frame) {
        let layout = Layout::vertical([Constraint::Fill(1), Constraint::Length(1)]).split(frame.size());
        frame.render_widget(self.text_area.borrow().widget(), layout[1]);

        let size = layout[0];
        let mut log_buffer = self.logs.borrow_mut();
        while log_buffer.len() > size.height as _ {
            log_buffer.pop_front();
        }
        frame.render_widget(List::new(log_buffer.iter().cloned()), size)
    }
}

#[tokio::main(flavor = "current_thread")]
async fn main() {
    let tasks = LocalSet::new();
    tasks.spawn_local(async {
        enable_raw_mode().unwrap();
        stdout().execute(EnterAlternateScreen).unwrap();
        let mut evts = EventStream::new();
        let mut term = Terminal::new(CrosstermBackend::new(std::io::stderr())).unwrap();
        let tui = Rc::<Tui>::default();
        // To run turtle_rc, replace with:
        // let _factory = turtle_rc::run(server::Server::new(tui.clone(), 1848));
        let _factory = build_factory(tui.clone());
        loop {
            term.draw(|frame| tui.frame(frame)).unwrap();
            let evt = select! {
                () = tui.on_redraw.notified() => None,
                evt = evts.next() => if let Some(Ok(x)) = evt { Some(x) } else { break }
            };
            if let Some(Event::Key(evt)) = evt {
                let evt = Input::from(evt);
                if evt.ctrl && (evt.key == Key::Char('c') || evt.key == Key::Char('d')) {
                    break;
                } else if evt.ctrl && evt.key == Key::Char('l') {
                    tui.logs.borrow_mut().clear()
                } else if evt.ctrl && evt.key == Key::Char('m') || evt.key == Key::Enter {
                    let mut text_area = tui.text_area.borrow_mut();
                    tui.inputs.borrow_mut().extend(text_area.lines().get(text_area.cursor().0).cloned());
                    text_area.move_cursor(CursorMove::End);
                    text_area.insert_newline()
                } else {
                    tui.text_area.borrow_mut().input(evt);
                }
                tui.on_input.notify_waiters()
            }
        }
        disable_raw_mode().unwrap();
        stdout().execute(LeaveAlternateScreen).unwrap();
    });
    tasks.await
}
