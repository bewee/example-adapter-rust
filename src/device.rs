/*
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/.*
 */
use crate::{clock_property::ClockProperty, timer_property::TimerProperty};
use async_trait::async_trait;
use gateway_addon_rust::device::{Device, Init};

pub struct ExampleDevice {}

#[async_trait(?Send)]
impl Device for ExampleDevice {
    async fn init(self: &mut Init<Self>) -> Result<(), String> {
        self.description_mut().title = Some(String::from("Timer"));

        log::debug!("Creating property");

        if let Err(err) = self.add_property(TimerProperty {}).await {
            log::error!("Could not create property: {}", err)
        }

        if let Err(err) = self.add_property(ClockProperty {}).await {
            log::error!("Could not create property: {}", err)
        }

        Ok(())
    }

    fn id(&self) -> &str {
        "example-device-rust"
    }
}
