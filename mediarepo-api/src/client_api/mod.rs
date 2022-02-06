pub mod error;
pub mod file;
pub mod job;
pub mod protocol;
pub mod repo;
pub mod tag;
pub mod preset;

use crate::client_api::error::{ApiError, ApiResult};
use crate::client_api::file::FileApi;
use crate::client_api::job::JobApi;
use crate::client_api::repo::RepoApi;
use crate::client_api::tag::TagApi;
use crate::types::misc::{check_apis_compatible, get_api_version, InfoResponse};
use async_trait::async_trait;
use bromine::prelude::*;
use bromine::prelude::emit_metadata::EmitMetadata;
use tokio::time::Duration;
use crate::client_api::preset::PresetApi;

#[async_trait]
pub trait IPCApi {
    fn namespace() -> &'static str;
    fn ctx(&self) -> PoolGuard<Context>;

    fn emit<T: IntoPayload + Send>(&self, event_name: &str, data: T) -> EmitMetadata<T> {
        let ctx = self.ctx();
        ctx.emit_to(Self::namespace(), event_name, data)
    }

    async fn emit_and_get<T: IntoPayload + Send + Sync + 'static, R: FromPayload + Send>(
        &self,
        event_name: &str,
        data: T,
        timeout: Option<Duration>,
    ) -> ApiResult<R> {
        let mut meta = self.emit(event_name, data).await_reply();

        if let Some(timeout) = timeout {
            meta = meta.with_timeout(timeout);
        }
        let response = meta.await?;

        Ok(response.payload()?)
    }
}
pub struct ApiClient {
    ctx: PooledContext,
    pub file: FileApi,
    pub tag: TagApi,
    pub repo: RepoApi,
    pub job: JobApi,
    pub preset: PresetApi,
}

impl Clone for ApiClient {
    fn clone(&self) -> Self {
        Self {
            ctx: self.ctx.clone(),
            file: self.file.clone(),
            tag: self.tag.clone(),
            repo: self.repo.clone(),
            job: self.job.clone(),
            preset: self.preset.clone(),
        }
    }
}

impl ApiClient {
    /// Creates a new client from an existing ipc context
    pub fn new(ctx: PooledContext) -> Self {
        Self {
            file: FileApi::new(ctx.clone()),
            tag: TagApi::new(ctx.clone()),
            repo: RepoApi::new(ctx.clone()),
            job: JobApi::new(ctx.clone()),
            preset: PresetApi::new(ctx.clone()),
            ctx,
        }
    }

    /// Connects to the ipc Socket
    #[tracing::instrument(level = "debug")]
    pub async fn connect<L: AsyncStreamProtocolListener>(
        address: L::AddressType,
    ) -> ApiResult<Self> {
        tracing::debug!("Connecting to {:?}", address);
        let ctx = IPCBuilder::<L>::new()
            .address(address)
            .timeout(Duration::from_secs(10))
            .build_pooled_client(8)
            .await?;
        let client = Self::new(ctx);
        tracing::debug!("Retrieving info on daemon version...");
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
        let res = ctx.emit("info", ()).await_reply().await?;
        tracing::trace!("Got info event {:?}", res);

        Ok(res.payload::<InfoResponse>()?)
    }

    /// Shuts down the daemon that the client is connected to.
    #[tracing::instrument(level = "debug", skip(self))]
    pub async fn shutdown_daemon(&self) -> ApiResult<()> {
        self.ctx
            .acquire()
            .emit("shutdown", ())
            .await_reply()
            .with_timeout(Duration::from_secs(5))
            .await?;
        Ok(())
    }

    #[tracing::instrument(level = "debug", skip(self))]
    pub async fn exit(self) -> ApiResult<()> {
        let ctx = (*self.ctx.acquire()).clone();

        ctx.stop().await?;
        Ok(())
    }
}
