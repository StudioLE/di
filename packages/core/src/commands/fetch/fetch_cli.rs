use crate::prelude::*;

/// CLI command for fetching an existing podcast.
#[derive(Service)]
pub struct FetchCliCommand {
    handler: Arc<FetchHandler>,
}

impl FetchCliCommand {
    /// Execute the fetch command.
    pub async fn execute(
        &self,
        options: FetchOptions,
    ) -> Result<FetchResponse, Report<FetchError>> {
        self.handler.execute(&FetchRequest::from(options)).await
    }
}
