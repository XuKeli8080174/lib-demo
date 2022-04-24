use crate::{
    common::{Request, Response},
    KvsError, Result, connection::Connection,
};
use tokio::{
    net::{TcpStream, ToSocketAddrs},
};

/// Key value store client
pub struct KvsClient {
    connection: Connection,
}

impl KvsClient {
    /// Connect to `addr` to access `KvsServer`.
    pub async fn connect<A: ToSocketAddrs>(addr: A) -> Result<Self> {
        let socket = TcpStream::connect(addr).await?;
        let connection = Connection::new(socket);
        Ok(KvsClient {
            connection
        })
    }

    /// Get the value of a given key from the server.
    pub async fn get(&mut self, key: String) -> Result<Option<String>> {
        let json = serde_json::to_string(&Request::Get { key })?;
        self.connection.write_json(&json).await?;

        match self.connection.read_resp().await? {
            Response::Get(value) => Ok(value),
            Response::Err(msg) => Err(KvsError::StringError(msg)),
            _ => unreachable!(),
        }
    }

    /// Set the value of a string key in the server.
    pub async fn set(&mut self, key: String, value: String) -> Result<()> {
        let json = serde_json::to_string(&Request::Set { key, value })?;
        self.connection.write_json(&json).await?;

        match self.connection.read_resp().await? {
            Response::Set => Ok(()),
            Response::Err(msg) => Err(KvsError::StringError(msg)),
            _ => unreachable!(),
        }
    }

    /// Remove a string key in the server.
    pub async fn remove(&mut self, key: String) -> Result<()> {
        let json = serde_json::to_string(&Request::Remove { key })?;
        self.connection.write_json(&json).await?;

        match self.connection.read_resp().await? {
            Response::Remove => Ok(()),
            Response::Err(msg) => Err(KvsError::StringError(msg)),
            _ => unreachable!(),
        }
    }
}
