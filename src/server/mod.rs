use crate::{KvsEngine, Result, connection::Connection, common::{Request, Response}};
use std::net::SocketAddr;
use tokio::net::{TcpListener, TcpStream};

/// The server of a key value store.
pub struct KvsServer<E: KvsEngine> {
    engine: E,
}

impl<E: KvsEngine> KvsServer<E> {
    /// Create a `KvsServer` with a given storage engine.
    pub fn new(engine: E) -> Self {
        KvsServer { engine }
    }

    /// Run the server listening on the given address
    pub async fn run(self, addr: SocketAddr) -> Result<()> {
        let listener = TcpListener::bind(&addr).await?;
        loop {
            let (tcp, peer_addr) = listener.accept().await?;
            let engine = self.engine.clone();
            serve(engine, tcp, peer_addr);
        }
    }
}

fn serve<E: KvsEngine>(engine: E, socket: TcpStream, _peer_addr: SocketAddr) {
    let mut handler = Handler {
        engine,
        connection: Connection::new(socket),
    };
    tokio::spawn(async move {
        if let Err(e) = handler.run().await {
            eprintln!("connection error: {:?}", e)
        }
    });
}

#[derive(Debug)]
struct Handler<E: KvsEngine> {
    engine: E,
    connection: Connection,
}

impl<E: KvsEngine> Handler<E> {
    async fn run(&mut self) -> Result<()> {
        loop {
            let resp = match self.connection.read_req().await? {
                Request::Get{ key } => {
                    let get_future = self.engine.get(key);
                    get_future.await.map(Response::Get)
                }
                Request::Set{ key, value } => {
                    let set_future = self.engine.set(key, value);
                    set_future.await.map(|_| Response::Set)
                }
                Request::Remove { key } => {
                    let rm_future = self.engine.remove(key);
                    rm_future.await.map(|_| Response::Remove)
                }
            };
            match resp {
                Ok(resp) => self.connection.write_json(serde_json::to_string(&resp).unwrap().as_str()).await?,
                Err(err) => {
                    let e = Response::Err(format!("{}", err));
                    self.connection.write_json(serde_json::to_string(&e).unwrap().as_str()).await?
                }
            }
        }
    }
}