use super::access::GetClient;
use super::action::ActionRequest;
use super::lua_value::{serialize, vec_to_table, Parser, Value};
use super::util::{spawn, AbortOnDrop};
use fnv::FnvHashMap;
use futures_util::{
    sink::SinkExt,
    stream::{SplitSink, SplitStream, StreamExt},
};
use socket2::{Domain, SockAddr, Socket, Type};
use std::{
    cell::RefCell,
    collections::VecDeque,
    fmt::Display,
    mem::replace,
    net::{Ipv6Addr, SocketAddr},
    rc::{Rc, Weak},
    time::Duration,
};
use tokio::{
    net::{TcpListener, TcpStream},
    time::sleep,
};
use tokio_tungstenite::{accept_async, tungstenite::Message, WebSocketStream};

pub struct Server {
    clients: Option<Rc<RefCell<Client>>>,
    logins: FnvHashMap<String, Weak<RefCell<Client>>>,
    _acceptor: AbortOnDrop<()>,
}

impl Drop for Server {
    fn drop(&mut self) {
        while let Some(client) = self.clients.take() {
            self.clients = Rc::try_unwrap(client)
                .map_err(|_| "client should be exclusively owned by server")
                .unwrap()
                .into_inner()
                .next
                .take()
        }
    }
}

enum WriterState {
    NotWriting(SplitSink<WebSocketStream<TcpStream>, Message>),
    Writing(AbortOnDrop<()>),
    Invalid,
}

struct Client {
    weak: Weak<RefCell<Client>>,
    name: String,
    next: Option<Rc<RefCell<Client>>>,
    prev: Option<Weak<RefCell<Client>>>,
    server: Weak<RefCell<Server>>,
    login: Option<String>,
    _reader: AbortOnDrop<()>,
    request_queue: VecDeque<Vec<Rc<RefCell<dyn ActionRequest>>>>,
    request_queue_size: usize,
    response_queue: VecDeque<Rc<RefCell<dyn ActionRequest>>>,
    writer: WriterState,
    timeout: Option<AbortOnDrop<()>>,
}

impl Drop for Client {
    fn drop(&mut self) {
        self.log("disconnected");
        self.name += " disconnected";
        for x in &self.request_queue {
            for x in x {
                x.borrow_mut().on_fail(self.name.clone())
            }
        }
        for x in &self.response_queue {
            x.borrow_mut().on_fail(self.name.clone())
        }
    }
}

impl Client {
    fn log(&self, x: &(impl Display + ?Sized)) { println!("{}: {}", self.name, x) }

    fn disconnect_by_server(&mut self, server: &mut Server) {
        if let Some(login) = &self.login {
            server.logins.remove(login);
        }
        if let Some(next) = self.next.as_ref() {
            next.borrow_mut().prev = self.prev.clone()
        }
        if let Some(prev) = self.prev.as_ref() {
            prev.upgrade().unwrap().borrow_mut().next = self.next.take()
        } else {
            server.clients = self.next.take()
        }
    }

    fn disconnect(&mut self) { self.disconnect_by_server(&mut self.server.upgrade().unwrap().borrow_mut()); }

    fn on_packet(&mut self, value: Value) -> Result<(), String> {
        if let Some(_) = &self.login {
            if let Some(x) = self.response_queue.pop_front() {
                self.update_timeout(true);
                x.borrow_mut().on_response(value)
            } else {
                Err(format!("unexpected packet: {:?}", value))
            }
        } else if let Value::S(login) = value {
            upgrade_mut!(self.server, server);
            self.name += &format!("[{}]", login);
            self.log("logged in");
            server.login(login.clone(), self.weak.clone());
            self.login = Some(login);
            Ok(())
        } else {
            Err(format!("invalid login packet: {:?}", value))
        }
    }

    fn enqueue_request_group(&mut self, group: Vec<Rc<RefCell<dyn ActionRequest>>>) {
        self.request_queue_size += group.len();
        self.request_queue.push_back(group);
        let writer = replace(&mut self.writer, WriterState::Invalid);
        if let WriterState::NotWriting(stream) = writer {
            self.update_timeout(false);
            self.writer = WriterState::Writing(spawn(writer_main(self.weak.clone(), stream)))
        } else {
            self.writer = writer
        }
    }

    fn update_timeout(&mut self, restart: bool) {
        if self.request_queue_size == 0 && self.response_queue.is_empty() {
            self.timeout = None
        } else if restart || self.timeout.is_none() {
            self.timeout = Some(spawn(timeout_main(self.weak.clone())))
        }
    }

    fn estimate_load(&self) -> usize { self.request_queue_size + self.response_queue.len() }
}

async fn timeout_main(client: Weak<RefCell<Client>>) {
    sleep(Duration::from_secs(120)).await;
    if let Some(this) = client.upgrade() {
        let mut this = this.borrow_mut();
        this.log("request timeout");
        this.disconnect()
    }
}

async fn writer_main(client: Weak<RefCell<Client>>, mut sink: SplitSink<WebSocketStream<TcpStream>, Message>) {
    loop {
        let mut data = Vec::new();
        if let Some(this) = client.upgrade() {
            let mut this = this.borrow_mut();
            if let Some(group) = this.request_queue.pop_front() {
                this.request_queue_size -= group.len();
                let mut value = Vec::new();
                for x in group {
                    value.push(x.borrow_mut().build_request());
                    this.response_queue.push_back(x)
                }
                serialize(&vec_to_table(value).into(), &mut data)
            } else {
                this.writer = WriterState::NotWriting(sink);
                break;
            }
        } else {
            break;
        }
        if let Err(e) = sink.send(Message::Binary(data)).await {
            if let Some(this) = client.upgrade() {
                let mut this = this.borrow_mut();
                this.log(&format!("error writing: {}", e));
                this.disconnect()
            }
            break;
        }
    }
}

async fn reader_main(client: Weak<RefCell<Client>>, mut stream: SplitStream<WebSocketStream<TcpStream>>) {
    let mut parser = Parser::new();
    loop {
        let data = stream.next().await;
        if let Some(this) = client.upgrade() {
            let mut this = this.borrow_mut();
            match data {
                Some(Err(e)) => {
                    this.log(&format!("error reading: {}", e));
                    this.disconnect();
                    break;
                }
                Some(Ok(Message::Binary(data))) => {
                    if let Err(e) = parser.shift(&data, &mut |x| this.on_packet(x)) {
                        this.log(&format!("error decoding packet: {}", e));
                        this.disconnect();
                        break;
                    }
                }
                Some(Ok(data)) => {
                    this.log(&format!("non-binary data: {}", data));
                    this.disconnect();
                    break;
                }
                None => {
                    this.log("client disconnected");
                    this.disconnect();
                    break;
                }
            }
        } else {
            break;
        }
    }
}

async fn handshake_main(client: Weak<RefCell<Client>>, stream: TcpStream) {
    let result = accept_async(stream).await;
    if let Some(this) = client.upgrade() {
        let mut this = this.borrow_mut();
        match result {
            Ok(socket) => {
                this.log("handshaked");
                let (sink, stream) = socket.split();
                this._reader = spawn(reader_main(client, stream));
                this.writer = WriterState::NotWriting(sink)
            }
            Err(e) => {
                this.log(&format!("handshake failed: {}", e));
                this.disconnect()
            }
        }
    }
}

fn create_listener(port: u16) -> TcpListener {
    let socket = Socket::new(Domain::IPV6, Type::STREAM, None).unwrap();
    socket.set_reuse_address(true).unwrap();
    socket.set_only_v6(false).unwrap();
    socket.bind(&SockAddr::from(SocketAddr::from((Ipv6Addr::UNSPECIFIED, port)))).unwrap();
    socket.set_nonblocking(true).unwrap();
    socket.listen(128).unwrap();
    TcpListener::from_std(socket.into()).unwrap()
}

async fn acceptor_main(server: Weak<RefCell<Server>>, listener: TcpListener) {
    loop {
        let (stream, addr) = listener.accept().await.unwrap();
        match server.upgrade() {
            None => break,
            Some(this) => {
                let mut this = this.borrow_mut();
                this.clients = Some(Rc::new_cyclic(|weak| {
                    let client = Client {
                        weak: weak.clone(),
                        name: addr.to_string(),
                        next: this.clients.take(),
                        prev: None,
                        server: server.clone(),
                        login: None,
                        _reader: spawn(handshake_main(weak.clone(), stream)),
                        request_queue: VecDeque::new(),
                        request_queue_size: 0,
                        response_queue: VecDeque::new(),
                        writer: WriterState::Invalid,
                        timeout: None,
                    };
                    if let Some(ref next) = client.next {
                        next.borrow_mut().prev = Some(weak.clone())
                    }
                    client.log("connected");
                    RefCell::new(client)
                }))
            }
        }
    }
}

impl Server {
    pub fn new(port: u16) -> Rc<RefCell<Self>> {
        Rc::new_cyclic(|weak| {
            RefCell::new(Server {
                clients: None,
                logins: FnvHashMap::default(),
                _acceptor: spawn(acceptor_main(weak.clone(), create_listener(port))),
            })
        })
    }

    fn login(&mut self, name: String, client: Weak<RefCell<Client>>) {
        if let Some(old) = self.logins.insert(name, client) {
            upgrade_mut!(old, old);
            old.log("logged in from another address");
            old.login = None;
            old.disconnect_by_server(self)
        }
    }

    pub fn enqueue_request_group(&self, client: &str, group: Vec<Rc<RefCell<dyn ActionRequest>>>) {
        if let Some(client) = self.logins.get(client) {
            client.upgrade().unwrap().borrow_mut().enqueue_request_group(group)
        } else {
            let reason = format!("{} isn't connected", client);
            for x in group {
                x.borrow_mut().on_fail(reason.clone())
            }
        }
    }

    fn estimate_load(&self, client: &str) -> usize {
        if let Some(client) = self.logins.get(client) {
            client.upgrade().unwrap().borrow().estimate_load()
        } else {
            usize::MAX
        }
    }

    pub fn load_balance<'a, T: GetClient>(&self, iter: impl IntoIterator<Item = &'a T>) -> &'a T {
        let mut iter = iter.into_iter();
        let mut best_access = iter.next().unwrap();
        let mut best_load = self.estimate_load(best_access.get_client());
        for access in iter {
            let load = self.estimate_load(access.get_client());
            if load < best_load {
                best_load = load;
                best_access = access
            }
        }
        best_access
    }
}
