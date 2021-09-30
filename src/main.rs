/*
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/.*
 */

#![feature(arbitrary_self_types)]

use crate::{adapter::ExampleAdapter, config::Config};
use gateway_addon_rust::{api_error::ApiError, plugin::connect};
use log::LevelFilter;
use simple_logger::SimpleLogger;

mod adapter;
mod clock_property;
mod config;
mod device;
mod timer_property;

#[tokio::main]
async fn main() {
    SimpleLogger::new()
        .with_level(LevelFilter::Debug)
        .init()
        .unwrap();
    run().await.expect("Could not start adapter");
    log::info!("Exiting adapter");
}

async fn run() -> Result<(), ApiError> {
    let mut plugin = connect("example-adapter-rust").await?;
    log::debug!("Plugin registered");

    let database = plugin.get_config_database();
    let conf: Option<Config> = database.load_config()?;

    if let Some(conf) = conf {
        database.save_config(&conf)?;

        log::debug!("Creating adapter");

        let result = plugin.create_adapter(ExampleAdapter::new()).await;

        if let Err(err) = result {
            plugin
                .fail(format!("Failed to create adapter: {}", err))
                .await?;
        }
    }

    plugin.event_loop().await;

    Ok(())
}
