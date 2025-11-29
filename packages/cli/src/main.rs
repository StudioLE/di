use alnwick_core::prelude::*;
use std::process::exit;

#[tokio::main]
async fn main() {
    init_logger().expect("should be able to init logger");
    let cli = Cli::parse();
    let services = ServiceProvider::new();
    match cli.command {
        Command::Scrape(options) => {
            let command = services
                .get_service::<ScrapeCommand>()
                .await
                .expect("should be able to get command");
            if let Err(e) = command.execute(options).await {
                error!("Failed to scrape podcast");
                eprintln!("{e:?}");
                exit(1);
            }
        }
        Command::Download(options) => {
            let command = services
                .get_service::<DownloadCommand>()
                .await
                .expect("should be able to get command");
            if let Err(e) = command.execute(options).await {
                error!("Failed to download podcast");
                eprintln!("{e:?}");
                exit(1);
            }
        }
        Command::Emulate(options) => {
            let command = services
                .get_service::<EmulateCommand>()
                .await
                .expect("should be able to get command");
            if let Err(e) = command.execute(options).await {
                error!("Failed to create RSS feeds");
                eprintln!("{e:?}");
                exit(1);
            }
        }
        Command::Cover(options) => {
            let command = services
                .get_service::<CoverCommand>()
                .await
                .expect("should be able to get command");
            if let Err(e) = command.execute(options).await {
                error!("Failed to create banner and cover images");
                eprintln!("{e:?}");
                exit(1);
            }
        }
    }
}

#[derive(Debug, Parser)]
struct Cli {
    #[command(subcommand)]
    command: Command,
}

#[derive(Debug, Subcommand)]
enum Command {
    /// Scrape a podcast from an RSS feed or website.
    Scrape(ScrapeOptions),
    /// Download episodes of a scraped podcast.
    Download(DownloadOptions),
    /// Create emulated RSS of a scraped podcast.
    Emulate(EmulateOptions),
    /// Download cover and banner images of a scraped podcast.
    Cover(CoverOptions),
}
