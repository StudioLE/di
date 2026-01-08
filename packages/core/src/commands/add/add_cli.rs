use crate::prelude::*;

/// CLI command for adding a new podcast.
#[derive(Service)]
pub struct AddCliCommand {
    handler: Arc<AddHandler>,
}

impl AddCliCommand {
    /// Execute the add command.
    pub async fn execute(&self, options: AddOptions) -> Result<AddResponse, Report<AddError>> {
        self.handler.execute(&AddRequest::from(options)).await
    }
}
