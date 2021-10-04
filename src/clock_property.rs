use std::time::Duration;

/*
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/.*
 */
use async_trait::async_trait;
use gateway_addon_rust::property::{Built, BuiltProperty, Init, InitProperty, Property, Type};
use serde_json::value::Value;
use tokio::time::sleep;

pub struct ClockProperty {}

#[async_trait]
impl Property for ClockProperty {
    async fn init(self: &mut Init<Self>) -> Result<(), String> {
        self.description_mut().name = Some("clock".to_owned());
        self.description_mut().title = Some("Time".to_owned());
        self.description_mut().description = Some("Current Time".to_owned());
        self.description_mut().read_only = Some(true);
        self.description_mut().value = Some(Value::from(0));

        Ok(())
    }

    async fn built(self: &mut Built<Self>) -> Result<(), String> {
        let weak = self.weak().clone();
        let arc = weak
            .upgrade()
            .ok_or_else(|| format!("Failed to obtain arc"))?;
        tokio::spawn(async move {
            loop {
                log::info!("Updating time");
                let mut property = arc.lock().await;
                property
                    .set_value(Value::String(format!("{:?}", chrono::offset::Local::now())))
                    .await
                    .unwrap();
                sleep(Duration::from_millis(1000)).await;
            }
        });
        Ok(())
    }

    fn id(&self) -> &str {
        "clock"
    }

    fn type_(&self) -> Type {
        Type::String
    }
}
