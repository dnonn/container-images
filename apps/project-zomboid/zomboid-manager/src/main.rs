mod zomboid;
mod config;

use crate::zomboid::server;
use ctrlc::set_handler;
use std::{process::exit, time::Duration};
use tokio::sync::mpsc;

#[macro_use]
extern crate log;

#[tokio::main()]
async fn main() {
    env_logger::init();
    let config = config::parse();

    let (tx, rx) = mpsc::channel(32);
    set_handler(move || {
        tx.blocking_send(()).expect("Unable to kill zomboid server");
    })
    .expect("Unable to install handler");
    info!("Installed signal handler for stopping server");

    info!("Starting wrapper");
    let game = match server::run(&config.install_path, config.server_arguments).await {
        Ok(g) => g,
        Err(error) => {
            error!("Failed to run game: {}", error);
            return;
        }
    };

    match server::wait_for(game, rx, Duration::from_secs(config.exit_timeout)).await {
        Ok(code) => {
            info!("Exit status: {}", code);
            // We do this to propagate errors to caller, and to ensure our
            // other routines die.
            exit(code)
        }
        Err(error) => error!("Failed: {}", error),
    }
}
