use crate::prelude::*;

/// Global podcasts [context](https://dioxuslabs.com/learn/0.6/reference/context/).
#[derive(Clone, Copy, Debug)]
pub struct PodcastsContext {
    pub loading: Signal<bool>,
    pub error: Signal<Option<String>>,
    pub podcasts: Signal<HashMap<String, PodcastFeed>>,
}

impl PodcastsContext {
    /// Creates a new instance of the context.
    ///
    /// This should be called at the top of the `App` component.
    pub fn create() {
        let context = Self {
            loading: use_signal(|| true),
            podcasts: use_signal(HashMap::new),
            error: use_signal(|| None),
        };
        let mut context = use_context_provider(|| context);
        context.update();
    }

    /// Consume the context from the current scope.
    #[must_use]
    pub fn consume() -> Self {
        consume_context()
    }

    /// Creates a new instance of the context.
    pub fn update(&mut self) {
        let mut context = *self;
        spawn(async move {
            match get_podcasts().await {
                Ok(podcasts) => {
                    context.podcasts.set(podcasts);
                }
                Err(error) => {
                    error!("{error:?}");
                    context.error.set(Some(error.to_string()));
                }
            }
            context.loading.set(false);
        });
    }
}

#[get("/api/podcasts")]
async fn get_podcasts() -> Result<HashMap<String, PodcastFeed>, ServerFnError> {
    let services = ServiceProvider::create()
        .await
        .expect("ServiceProvider should not fail");
    let command = ListCommand::new(services.paths, services.metadata);
    match command.execute().await {
        Ok(podcasts) => Ok(podcasts),
        Err(error) => {
            error!("{error:?}");
            Err(ServerFnError::new(error.to_string()))
        }
    }
}
