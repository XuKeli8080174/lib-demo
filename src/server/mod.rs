use crate::common::{Request, Response};
use crate::{KvsEngine, KvsError, Result};
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
            serve(engine, tcp, peer_addr)?
        }
    }
}

fn serve<E: KvsEngine>(engine: E, tcp: TcpStream, peer_addr: SocketAddr) -> Result<()> {
    
    Ok(())
}