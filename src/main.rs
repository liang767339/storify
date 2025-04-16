use anyhow::Result;
use clap::{Parser, Subcommand};
use std::env;

mod cli;

#[derive(Parser, Debug)]
#[command(
    version = "0.1.0",
    author = "WangErxi",
    about = "ossify is a tool for managing OSS files"
)]
struct CommandLineArgs {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand, Debug)]
enum Commands {
    /// List files in a directory
    List {
        /// Target path
        path: String,
    },
    /// Download files
    Download {
        /// Remote path
        remote_path: String,
        /// Local save path
        local_path: String,
    },
    /// Count files
    Count {
        /// Target path
        path: String,
    },
}

#[tokio::main]
async fn main() -> Result<()> {
    let args = CommandLineArgs::parse();

    let bucket = env::var("OSS_BUCKET").expect("Please set the environment variable OSS_BUCKET");
    let access_key_id = env::var("OSS_ACCESS_KEY_ID")
        .expect("Please set the environment variable OSS_ACCESS_KEY_ID");
    let access_key_secret = env::var("OSS_ACCESS_KEY_SECRET")
        .expect("Please set the environment variable OSS_ACCESS_KEY_SECRET");
    let endpoint = env::var("OSS_ENDPOINT")
        .unwrap_or_else(|_| "https://oss-cn-hangzhou.aliyuncs.com".to_string());

    match args.command {
        Commands::List { path } => {
            cli::list_directory(
                path,
                bucket,
                endpoint.clone(),
                access_key_id,
                access_key_secret,
            )
            .await?;
        }
        Commands::Download {
            remote_path,
            local_path,
        } => {
            cli::download_files(
                remote_path,
                local_path,
                bucket,
                endpoint.clone(),
                access_key_id,
                access_key_secret,
            )
            .await?;
        }
        Commands::Count { path } => {
            let count = cli::count_files_in_directory(
                path,
                bucket,
                endpoint.clone(),
                access_key_id,
                access_key_secret,
            )
            .await?;
            println!("File count: {}", count);
        }
    }

    Ok(())
}
