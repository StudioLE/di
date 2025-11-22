use alnwick_core::prelude::*;
use std::process::exit;

#[tokio::main]
async fn main() {
    init_logger();
    let cli = Cli::parse();
    let services = match ServiceProvider::create().await {
        Ok(services) => services,
        Err(e) => {
            error!("An error occured during service creation");
            eprintln!("{e:?}");
            exit(1);
        }
    };
    match cli.command {
        Command::Scrape(options) => {
            let command = ScrapeCommand::new(services.http, services.metadata);
            if let Err(e) = command.execute(options).await {
                error!("Failed to scrape podcast");
                eprintln!("{e:?}");
                exit(1);
            }
        }
        Command::Download(options) => {
            let command = DownloadCommand::new(services.paths, services.http, services.metadata);
            if let Err(e) = command.execute(options).await {
                error!("Failed to download podcast");
                eprintln!("{e:?}");
                exit(1);
            }
        }
        Command::Emulate(options) => {
            let command = EmulateCommand::new(services.paths, services.metadata);
            if let Err(e) = command.execute(options).await {
                error!("Failed to create RSS feeds");
                eprintln!("{e:?}");
                exit(1);
            }
        }
        Command::Cover(options) => {
            let command = CoverCommand::new(services.paths, services.http, services.metadata);
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
