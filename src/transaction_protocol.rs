use libp2p::request_response::{ProtocolName, RequestResponseCodec};
use async_trait::async_trait;
use std::io;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TransactionRequest(pub Vec<u8>);

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TransactionResponse(pub Vec<u8>);

#[derive(Debug, Clone)]
pub struct TransactionProtocol;

impl ProtocolName for TransactionProtocol {
    fn protocol_name(&self) -> &[u8] {
        "/transaction/1".as_bytes()
    }
}

#[derive(Clone)]
pub struct TransactionCodec;

#[async_trait]
impl RequestResponseCodec for TransactionCodec {
    type Protocol = TransactionProtocol;
    type Request = TransactionRequest;
    type Response = TransactionResponse;

    async fn read_request<T>(
        &mut self,
        _: &TransactionProtocol,
        io: &mut T,
    ) -> io::Result<Self::Request>
    where
        T: async_std::io::AsyncRead + Unpin + Send,
    {
        let mut buf = Vec::new();
        async_std::io::ReadExt::read_to_end(io, &mut buf).await?;
        Ok(TransactionRequest(buf))
    }

    async fn read_response<T>(
        &mut self,
        _: &TransactionProtocol,
        io: &mut T,
    ) -> io::Result<Self::Response>
    where
        T: async_std::io::AsyncRead + Unpin + Send,
    {
        let mut buf = Vec::new();
        async_std::io::ReadExt::read_to_end(io, &mut buf).await?;
        Ok(TransactionResponse(buf))
    }

    async fn write_request<T>(
        &mut self,
        _: &TransactionProtocol,
        io: &mut T,
        TransactionRequest(data): TransactionRequest,
    ) -> io::Result<()>
    where
        T: async_std::io::AsyncWrite + Unpin + Send,
    {
        async_std::io::WriteExt::write_all(io, &data).await?;
        Ok(())
    }

    async fn write_response<T>(
        &mut self,
        _: &TransactionProtocol,
        io: &mut T,
        TransactionResponse(data): TransactionResponse,
    ) -> io::Result<()>
    where
        T: async_std::io::AsyncWrite + Unpin + Send,
    {
        async_std::io::WriteExt::write_all(io, &data).await?;
        Ok(())
    }
}