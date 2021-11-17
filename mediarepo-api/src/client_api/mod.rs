pub mod error;
pub mod file;
pub mod protocol;
pub mod tag;

use crate::client_api::error::{ApiError, ApiResult};
use crate::client_api::file::FileApi;
use crate::client_api::tag::TagApi;
use crate::types::misc::{check_apis_compatible, get_api_version, InfoResponse};
use async_trait::async_trait;
use rmp_ipc::context::{PoolGuard, PooledContext};
use rmp_ipc::ipc::context::Context;
use rmp_ipc::ipc::stream_emitter::EmitMetadata;
use rmp_ipc::payload::{EventReceivePayload, EventSendPayload};
use rmp_ipc::prelude::{AsyncProtocolStream, AsyncStreamProtocolListener};
use rmp_ipc::IPCBuilder;
use std::time::Duration;

#[async_trait]
pub trait IPCApi<S: AsyncProtocolStream> {
    fn namespace() -> &'static str;
    fn ctx(&self) -> PoolGuard<Context<S>>;

    async fn emit<T: EventSendPayload + Send>(
        &self,
        event_name: &str,
        data: T,
    ) -> ApiResult<EmitMetadata> {
        let ctx = self.ctx();
        let meta = ctx
            .emitter
            .emit_to(Self::namespace(), event_name, data)
            .await?;

        Ok(meta)
    }

    async fn emit_and_get<T: EventSendPayload + Send, R: EventReceivePayload + Send>(
        &self,
        event_name: &str,
        data: T,
    ) -> ApiResult<R> {
        let meta = self.emit(event_name, data).await?;
        let response = meta.await_reply(&self.ctx()).await?;

        Ok(response.data()?)
    }
}
pub struct ApiClient<L: AsyncStreamProtocolListener> {
    ctx: PooledContext<L::Stream>,
    pub file: FileApi<L::Stream>,
    pub tag: TagApi<L::Stream>,
}

impl<L> Clone for ApiClient<L>
where
    L: AsyncStreamProtocolListener,
{
    fn clone(&self) -> Self {
        Self {
            ctx: self.ctx.clone(),
            file: self.file.clone(),
            tag: self.tag.clone(),
        }
    }
}

impl<L> ApiClient<L>
where
    L: AsyncStreamProtocolListener,
{
    /// Creates a new client from an existing ipc context
    pub fn new(ctx: PooledContext<L::Stream>) -> Self {
        Self {
            file: FileApi::new(ctx.clone()),
            tag: TagApi::new(ctx.clone()),
            ctx,
        }
    }

    /// Connects to the ipc Socket
    #[tracing::instrument(level = "debug")]
    pub async fn connect(address: L::AddressType) -> ApiResult<Self> {
        let ctx = IPCBuilder::<L>::new()
            .address(address)
            .timeout(Duration::from_secs(10))
            .build_pooled_client(8)
            .await?;
        let client = Self::new(ctx);
        let info = client.info().await?;
        let server_api_version = info.api_version();

        if !check_apis_compatible(get_api_version(), server_api_version) {
            let server_version_string = format!(
                "{}.{}.{}",
                server_api_version.0, server_api_version.1, server_api_version.2
            );
            let client_version_string = env!("CARGO_PKG_VERSION").to_string();
            Err(ApiError::VersionMismatch {
                server: server_version_string,
                client: client_version_string,
            })
        } else {
            Ok(client)
        }
    }

    /// Returns information about the connected ipc server
    #[tracing::instrument(level = "debug", skip(self))]
    pub async fn info(&self) -> ApiResult<InfoResponse> {
        let ctx = self.ctx.acquire();
        let res = ctx
            .emitter
            .emit("info", ())
            .await?
            .await_reply(&ctx)
            .await?;
        Ok(res.data::<InfoResponse>()?)
    }

    #[tracing::instrument(level = "debug", skip(self))]
    pub async fn exit(self) -> ApiResult<()> {
        let ctx = (*self.ctx.acquire()).clone();
        ctx.stop().await?;
        Ok(())
    }
}
