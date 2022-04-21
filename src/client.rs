use crate::{
    common::{Request, Response},
    KvsError, Result,
};
use bytes::{Buf, BytesMut};
use serde::Deserialize;
use serde_json::{de::IoRead, Deserializer};
use tokio::{
    io::{AsyncReadExt, AsyncWriteExt, BufWriter},
    net::{TcpStream, ToSocketAddrs},
};

/// Key value store client
pub struct KvsClient {
    stream: BufWriter<TcpStream>,
    buffer: BytesMut,
}

impl KvsClient {
    /// Connect to `addr` to access `KvsServer`.
    pub async fn connect<A: ToSocketAddrs>(addr: A) -> Result<Self> {
        let socket = TcpStream::connect(addr).await?;
        Ok(KvsClient {
            stream: BufWriter::new(socket),
            buffer: BytesMut::with_capacity(4 * 1024),
        })
    }

    /// Get the value of a given key from the server.
    pub async fn get(&mut self, key: String) -> Result<Option<String>> {
        let json = serde_json::to_string(&Request::Get { key })?;
        self.stream.write_all(json.as_bytes()).await?;

        match self.read_response().await? {
            Response::Get(value) => Ok(value),
            Response::Err(msg) => Err(KvsError::StringError(msg)),
            _ => unreachable!(),
        }
    }

    /// Set the value of a string key in the server.
    pub async fn set(&mut self, key: String, value: String) -> Result<()> {
        let json = serde_json::to_string(&Request::Set { key, value })?;
        self.stream.write_all(json.as_bytes()).await?;

        match self.read_response().await? {
            Response::Set => Ok(()),
            Response::Err(msg) => Err(KvsError::StringError(msg)),
            _ => unreachable!(),
        }
    }

    /// Remove a string key in the server.
    pub async fn remove(&mut self, key: String) -> Result<()> {
        let json = serde_json::to_string(&Request::Remove { key })?;
        self.stream.write_all(json.as_bytes()).await?;

        match self.read_response().await? {
            Response::Remove => Ok(()),
            Response::Err(msg) => Err(KvsError::StringError(msg)),
            _ => unreachable!(),
        }
    }

    async fn read_response(&mut self) -> Result<Response> {
        loop {
            if let Some(resp) = self.parse_json()? {
                return Ok(resp);
            }

            if 0 == self.stream.read_buf(&mut self.buffer).await? {
                return Err(KvsError::StringError("connection reset by peer".into()));
            }
        }
    }

    fn parse_json(&mut self) -> Result<Option<Response>> {
        let slice = &self.buffer[..];
        let mut json_stream = Deserializer::from_slice(slice).into_iter::<Response>();

        match json_stream.next() {
            None => Ok(None),
            Some(Ok(resp)) => {
                let offset = json_stream.byte_offset();
                self.buffer.advance(offset);
                Ok(Some(resp))
            }
            Some(Err(e)) => {
                if e.is_eof() {
                    Ok(None)
                } else {
                    Err(KvsError::Serde(e))
                }
            }
        }
    }
}
