mod utils;
mod frontend;
mod internals;
mod parsers;

use utils::logger;
use frontend::server;
use log::LevelFilter;
use serde::Deserialize;
use std::fs;
use internals::folder_processing;
use tokio::time::Duration;
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};
use std::env;

extern crate ctrlc;

#[derive(Deserialize)]
struct Config {
    log_file: String,
    log_level: String,
    sequester_path: String,
}

#[actix_web::main]
async fn main() {
    let args: Vec<String> = env::args().collect();
    let config_path;

    if args.len() > 2 && &args[1] == "-f" {
        config_path = &args[2];
    } else {
        eprintln!("Error: The '-f' argument followed by the configuration file path is required.");
        return;
    }

    let config_yaml = fs::read_to_string(config_path).expect("Failed to read the configuration file");
    let config: Config = serde_yaml::from_str(&config_yaml).expect("Failed to parse YAML in the configuration file");

    let logger = logger::JsonLogger::new(&config.log_file, None);
    let leaked_logger = Box::leak(Box::new(logger));
    log::set_logger(leaked_logger).unwrap();

    let level_filter = match config.log_level.to_lowercase().as_str() {
        "error" => LevelFilter::Error,
        "warn" => LevelFilter::Warn,
        "info" => LevelFilter::Info,
        "debug" => LevelFilter::Debug,
        "trace" => LevelFilter::Trace,
        _ => LevelFilter::Info,
    };
    log::set_max_level(level_filter);

    log::info!("Starting the application");

    let folder_path = &config.sequester_path;

    let should_exit = Arc::new(AtomicBool::new(false));

    ctrlc::set_handler({
        let should_exit = Arc::clone(&should_exit);
        move || {
            log::info!("CTRL+C received. Shutting down...");
            should_exit.store(true, Ordering::Relaxed);
        }
    }).expect("Error setting Ctrl-C handler");

    std::thread::spawn({
        let folder_path = folder_path.clone();
        let should_exit = Arc::clone(&should_exit);
        move || {
            let rt = tokio::runtime::Runtime::new().expect("Failed to create a runtime");
            rt.block_on(async {
                loop {
                    if should_exit.load(Ordering::Relaxed) {
                        break;
                    }
                    folder_processing::process_files_in_folder(&folder_path).await;
                    tokio::time::sleep(Duration::from_secs(5)).await;
                }
            });
        }
    });

    server::run_server().await.expect("Server failed");
}
