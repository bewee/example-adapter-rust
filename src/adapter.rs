/*
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/.*
 */
use crate::device::ExampleDevice;
use async_trait::async_trait;
use chrono;
use gateway_addon_rust::{
    adapter::{Adapter, Built},
    device::BuiltDevice,
};
use serde_json::Value;
use std::time::Duration;
use tokio::time::sleep;

pub struct ExampleAdapter {}

impl ExampleAdapter {
    pub fn new() -> Self {
        ExampleAdapter {}
    }
}

#[async_trait(?Send)]
impl Adapter for ExampleAdapter {
    async fn init(self: &mut Built<Self>) -> Result<(), String> {
        log::debug!("Creating device");

        match self.add_device(ExampleDevice {}).await {
            Err(err) => log::error!("Could not create device: {}", err),
            Ok(device) => {
                tokio::spawn(async move {
                    loop {
                        log::info!("Updating time");
                        let device = device.lock().await;
                        let property = device.get_property("clock");
                        let property = property.unwrap();
                        let mut property = property.lock().await;
                        property
                            .set_value(Value::String(format!("{:?}", chrono::offset::Local::now())))
                            .await
                            .unwrap();
                        sleep(Duration::from_millis(1000)).await;
                    }
                });
            }
        }

        Ok(())
    }

    async fn on_device_saved(
        self: &Built<Self>,
        id: String,
        _device_description: webthings_gateway_ipc_types::DeviceWithoutId,
    ) -> Result<(), String> {
        println!("Device saved: {}", id);
        Ok(())
    }

    fn id(&self) -> &str {
        "example-adapter-rust"
    }

    fn name(&self) -> &str {
        "Example Rust Adapter"
    }
}
