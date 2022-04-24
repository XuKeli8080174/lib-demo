use bytes::{BytesMut, Buf};
use serde_json::{Deserializer};
use tokio::{net::TcpStream, io::{BufWriter, AsyncReadExt, AsyncWriteExt}};

use crate::{common::{Response, Request}, KvsError, Result};

/// inspired by mini-redis
#[derive(Debug)]
pub struct Connection {
    stream: BufWriter<TcpStream>,
    buffer: BytesMut,
}

impl Connection {
    pub fn new(socket: TcpStream) -> Connection {
        Connection {
            stream: BufWriter::new(socket),
            buffer: BytesMut::with_capacity(4 * 1024),
        }
    }

    pub async fn read_resp(&mut self) -> Result<Response> {
        loop {
            if let Some(resp) = self.parse_resp()? {
                return Ok(resp);
            }

            if 0 == self.stream.read_buf(&mut self.buffer).await? {
                return Err(KvsError::StringError("connection reset by peer".into()));
            }
        }
    }

    pub async fn read_req(&mut self) -> Result<Request> {
        loop {
            if let Some(req) = self.parse_req()? {
                return Ok(req)
            }

            if 0 == self.stream.read_buf(&mut self.buffer).await? {
                return Err(KvsError::StringError("connection reset by peer".into()));
            }
        }
    }

    pub async fn write_json(&mut self, json: &str) -> Result<()> {
        match self.stream.write_all(json.as_bytes()).await {
            Ok(_) => Ok(()),
            Err(err) => Err(KvsError::from(err)),
        }
    }

    fn parse_resp(&mut self) -> Result<Option<Response>> {
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

    fn parse_req(&mut self) -> Result<Option<Request>> {
        let mut json_stream = Deserializer::from_slice(&self.buffer[..]).into_iter::<Request>();
        match json_stream.next() {
            None => Ok(None),
            Some(Ok(req)) => {
                let offset = json_stream.byte_offset();
                self.buffer.advance(offset);
                Ok(Some(req))
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
