pub mod error;
pub mod file;
pub mod tag;

use crate::client_api::error::ApiResult;
use crate::client_api::file::FileApi;
use crate::client_api::tag::TagApi;
use crate::types::misc::InfoResponse;
use async_trait::async_trait;
use rmp_ipc::ipc::context::Context;
use rmp_ipc::ipc::stream_emitter::EmitMetadata;
use rmp_ipc::payload::{EventReceivePayload, EventSendPayload};
use rmp_ipc::IPCBuilder;
use std::fmt::Debug;

#[async_trait]
pub trait IPCApi {
    fn namespace() -> &'static str;
    fn ctx(&self) -> &Context;

    async fn emit<T: EventSendPayload + Debug + Send>(
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

    async fn emit_and_get<
        T: EventSendPayload + Debug + Send,
        R: EventReceivePayload + Debug + Send,
    >(
        &self,
        event_name: &str,
        data: T,
    ) -> ApiResult<R> {
        let meta = self.emit(event_name, data).await?;
        let response = meta.await_reply(self.ctx()).await?;

        Ok(response.data()?)
    }
}

#[derive(Clone)]
pub struct ApiClient {
    ctx: Context,
    pub file: FileApi,
    pub tag: TagApi,
}

impl ApiClient {
    /// Creates a new client from an existing ipc context
    pub fn new(ctx: Context) -> Self {
        Self {
            file: FileApi::new(ctx.clone()),
            tag: TagApi::new(ctx.clone()),
            ctx,
        }
    }

    /// Connects to the ipc Socket
    pub async fn connect(address: &str) -> ApiResult<Self> {
        let ctx = IPCBuilder::new().address(address).build_client().await?;

        Ok(Self::new(ctx))
    }

    /// Returns information about the connected ipc server
    #[tracing::instrument(level = "debug", skip(self))]
    pub async fn info(&self) -> ApiResult<InfoResponse> {
        let res = self
            .ctx
            .emitter
            .emit("info", ())
            .await?
            .await_reply(&self.ctx)
            .await?;
        Ok(res.data::<InfoResponse>()?)
    }
}
