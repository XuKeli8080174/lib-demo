use crate::{KvsEngine, Result, connection::Connection, common::{Request, Response}};
use std::{net::SocketAddr, sync::Arc};
use log::{error, info};
use tokio::{net::{TcpListener, TcpStream}, sync::{Semaphore, broadcast, mpsc}, signal};

const MAX_CONNECTIONS: usize = 250;

/// run the server
pub async fn run<E: KvsEngine>(listener: TcpListener, engine: E) {
    let mut server = KvsServer::new(engine, listener);

    tokio::select! {
        ret = server.run() => {
            if let Err(err) = ret {
                error!("failed to accept, err: {}", err);
            }
        }
        _ = signal::ctrl_c() => {
            info!("shutting down");
        }
    }

    let KvsServer {
        mut shutdown_complete_rx,
        shutdown_complete_tx,
        notify_shutdown,
        ..
    } = server;

    drop(notify_shutdown);
    drop(shutdown_complete_tx);

    let _ = shutdown_complete_rx.recv().await;
}

/// The server of a key value store.
pub struct KvsServer<E: KvsEngine> {
    listener: TcpListener,
    engine: E,
    limit_connections: Arc<Semaphore>,
    notify_shutdown: broadcast::Sender<()>,
    shutdown_complete_rx: mpsc::Receiver<()>,
    shutdown_complete_tx: mpsc::Sender<()>,
}

impl<E: KvsEngine> KvsServer<E> {
    /// Create a `KvsServer` with a given storage engine.
    pub fn new(engine: E, listener: TcpListener) -> Self {
        let (notify_shutdown, _) = broadcast::channel(1);
        let (shutdown_complete_tx, shutdown_complete_rx) = mpsc::channel(1);

        KvsServer {
            listener,
            engine,
            limit_connections: Arc::new(Semaphore::new(MAX_CONNECTIONS)),
            notify_shutdown,
            shutdown_complete_tx,
            shutdown_complete_rx,
        }
    }

    /// Run the server listening on the given address
    pub async fn run(&mut self) -> Result<()> {
        loop {
            self.limit_connections.acquire().await.unwrap().forget();
            let (tcp, peer_addr) = self.listener.accept().await?;
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