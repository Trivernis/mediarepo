use async_trait::async_trait;
use rmp_ipc::error::Result;
use rmp_ipc::prelude::IPCResult;
use rmp_ipc::protocol::*;
use std::io::Error;
use std::net::ToSocketAddrs;
use std::pin::Pin;
use std::task::{Context, Poll};
use tokio::io::{AsyncRead, AsyncWrite, ReadBuf};
use tokio::net::{TcpListener, TcpStream};

#[derive(Debug)]
pub enum ApiProtocolListener {
    #[cfg(unix)]
    UnixSocket(tokio::net::UnixListener),

    Tcp(TcpListener),
}

unsafe impl Send for ApiProtocolListener {}
unsafe impl Sync for ApiProtocolListener {}

#[async_trait]
impl AsyncStreamProtocolListener for ApiProtocolListener {
    type AddressType = String;
    type RemoteAddressType = String;
    type Stream = ApiProtocolStream;

    async fn protocol_bind(address: Self::AddressType) -> Result<Self> {
        if let Some(addr) = address.to_socket_addrs().ok().and_then(|mut a| a.next()) {
            let listener = TcpListener::bind(addr).await?;
            Ok(Self::Tcp(listener))
        } else {
            #[cfg(unix)]
            {
                use std::path::PathBuf;
                use tokio::net::UnixListener;
                let path = PathBuf::from(address);
                let listener = UnixListener::bind(path)?;

                Ok(Self::UnixSocket(listener))
            }
            #[cfg(not(unix))]
            {
                Err(IPCError::BuildError(
                    "The address can not be made into a socket address".to_string(),
                ))
            }
        }
    }

    async fn protocol_accept(&self) -> Result<(Self::Stream, Self::RemoteAddressType)> {
        match self {
            ApiProtocolListener::UnixSocket(listener) => {
                let (stream, addr) = listener.accept().await?;
                Ok((
                    ApiProtocolStream::UnixSocket(stream),
                    addr.as_pathname()
                        .map(|p| p.to_str().unwrap().to_string())
                        .unwrap_or(String::from("unknown")),
                ))
            }
            ApiProtocolListener::Tcp(listener) => {
                let (stream, addr) = listener.accept().await?;
                Ok((ApiProtocolStream::Tcp(stream), addr.to_string()))
            }
        }
    }
}

#[derive(Debug)]
pub enum ApiProtocolStream {
    #[cfg(unix)]
    UnixSocket(tokio::net::UnixStream),

    Tcp(TcpStream),
}

unsafe impl Send for ApiProtocolStream {}
unsafe impl Sync for ApiProtocolStream {}

impl AsyncProtocolStreamSplit for ApiProtocolStream {
    type OwnedSplitReadHalf = Box<dyn AsyncRead + Unpin + Send + Sync>;
    type OwnedSplitWriteHalf = Box<dyn AsyncWrite + Unpin + Send + Sync>;

    fn protocol_into_split(self) -> (Self::OwnedSplitReadHalf, Self::OwnedSplitWriteHalf) {
        match self {
            #[cfg(unix)]
            ApiProtocolStream::UnixSocket(stream) => {
                let (read, write) = stream.into_split();
                (Box::new(read), Box::new(write))
            }
            ApiProtocolStream::Tcp(stream) => {
                let (read, write) = stream.into_split();
                (Box::new(read), Box::new(write))
            }
        }
    }
}

#[async_trait]
impl AsyncProtocolStream for ApiProtocolStream {
    type AddressType = String;

    async fn protocol_connect(address: Self::AddressType) -> IPCResult<Self> {
        if let Some(addr) = address.to_socket_addrs().ok().and_then(|mut a| a.next()) {
            let stream = TcpStream::connect(addr).await?;
            Ok(Self::Tcp(stream))
        } else {
            #[cfg(unix)]
            {
                use std::path::PathBuf;
                use tokio::net::UnixStream;
                let path = PathBuf::from(address);
                let stream = UnixStream::connect(path).await?;

                Ok(Self::UnixSocket(stream))
            }
            #[cfg(not(unix))]
            {
                Err(IPCError::BuildError(
                    "The address can not be made into a socket address".to_string(),
                ))
            }
        }
    }
}

impl AsyncRead for ApiProtocolStream {
    fn poll_read(
        self: Pin<&mut Self>,
        cx: &mut Context<'_>,
        buf: &mut ReadBuf<'_>,
    ) -> Poll<std::io::Result<()>> {
        match self.get_mut() {
            #[cfg(unix)]
            ApiProtocolStream::UnixSocket(stream) => Pin::new(stream).poll_read(cx, buf),
            ApiProtocolStream::Tcp(stream) => Pin::new(stream).poll_read(cx, buf),
        }
    }
}

impl AsyncWrite for ApiProtocolStream {
    fn poll_write(
        self: Pin<&mut Self>,
        cx: &mut Context<'_>,
        buf: &[u8],
    ) -> Poll<std::prelude::rust_2015::Result<usize, Error>> {
        match self.get_mut() {
            #[cfg(unix)]
            ApiProtocolStream::UnixSocket(stream) => Pin::new(stream).poll_write(cx, buf),
            ApiProtocolStream::Tcp(stream) => Pin::new(stream).poll_write(cx, buf),
        }
    }

    fn poll_flush(
        self: Pin<&mut Self>,
        cx: &mut Context<'_>,
    ) -> Poll<std::prelude::rust_2015::Result<(), Error>> {
        match self.get_mut() {
            #[cfg(unix)]
            ApiProtocolStream::UnixSocket(stream) => Pin::new(stream).poll_flush(cx),
            ApiProtocolStream::Tcp(stream) => Pin::new(stream).poll_flush(cx),
        }
    }

    fn poll_shutdown(
        self: Pin<&mut Self>,
        cx: &mut Context<'_>,
    ) -> Poll<std::prelude::rust_2015::Result<(), Error>> {
        match self.get_mut() {
            #[cfg(unix)]
            ApiProtocolStream::UnixSocket(stream) => Pin::new(stream).poll_shutdown(cx),
            ApiProtocolStream::Tcp(stream) => Pin::new(stream).poll_flush(cx),
        }
    }
}
