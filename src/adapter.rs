/*
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/.*
 */
use crate::device::ExampleDevice;
use async_trait::async_trait;
use gateway_addon_rust::adapter::{Adapter, Built};

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

        if let Err(err) = self.add_device(ExampleDevice {}).await {
            log::error!("Could not create device: {}", err)
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
