pub mod error;

use rmp_ipc::ipc::context::Context;
use rmp_ipc::IPCBuilder;
use crate::client_api::error::ApiResult;

#[derive(Clone)]
pub struct ApiClient {
    ctx: Context,
}

impl ApiClient {
    /// Creates a new client from an existing ipc context
    pub fn new(ctx: Context) -> Self {
        Self {ctx}
    }

    /// Connects to the ipc Socket
    pub async fn connect(address: &str) -> ApiResult<Self> {
        let ctx = IPCBuilder::new().address(address).build_client().await?;

        Ok(Self::new(ctx))
    }
}