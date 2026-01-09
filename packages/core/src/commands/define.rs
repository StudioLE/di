use crate::prelude::*;

define_commands_web!(Download(DownloadRequest), Fetch(FetchRequest));
#[cfg(feature = "server")]
define_commands_server!(
    Download(DownloadRequest, DownloadHandler),
    Fetch(FetchRequest, FetchHandler),
);
