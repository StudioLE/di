use crate::prelude::*;
use core::result::Result;
use error_stack::{IntoReport, ResultExt as DefaultResultExt};

pub trait CustomResultExt {
    type Context: ?Sized;
    type Ok;

    fn attach_episode(self, episode: &EpisodeInfo) -> Result<Self::Ok, Report<Self::Context>>;

    fn attach_path<P>(self, path: P) -> Result<Self::Ok, Report<Self::Context>>
    where
        P: AsRef<Path>;

    fn attach_url(self, url: &UrlWrapper) -> Result<Self::Ok, Report<Self::Context>>;
}

impl<T, E> CustomResultExt for Result<T, E>
where
    E: IntoReport,
{
    type Context = E::Context;
    type Ok = T;

    fn attach_episode(self, episode: &EpisodeInfo) -> Result<T, Report<E::Context>> {
        self.attach_with(|| format!("Episode: {episode}"))
    }

    fn attach_path<P>(self, path: P) -> Result<T, Report<E::Context>>
    where
        P: AsRef<Path>,
    {
        self.attach_with(|| format!("Path: {}", path.as_ref().display()))
    }

    fn attach_url(self, url: &UrlWrapper) -> Result<T, Report<E::Context>> {
        self.attach_with(|| format!("URL: {url}"))
    }
}
