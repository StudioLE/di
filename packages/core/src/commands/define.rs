use crate::prelude::*;

define_commands_web!(Download(DownloadRequest));
#[cfg(feature = "server")]
define_commands_server!(Download(DownloadRequest, DownloadHandler));
