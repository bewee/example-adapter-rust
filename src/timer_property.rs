/*
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/.*
 */
use async_trait::async_trait;
use gateway_addon_rust::property::{Built, Init, InitProperty, Property, Type};
use serde_json::value::Value;

pub struct TimerProperty {}

#[async_trait]
impl Property for TimerProperty {
    async fn init(self: &mut Init<Self>) -> Result<(), String> {
        self.description_mut().name = Some("timer".to_owned());
        self.description_mut().title = Some("Timer".to_owned());
        self.description_mut().description = Some("A Timer".to_owned());
        self.description_mut().minimum = Some(0_f64);
        self.description_mut().maximum = Some(255_f64);
        self.description_mut().multiple_of = Some(1_f64);
        self.description_mut().read_only = Some(false);
        self.description_mut().value = Some(Value::from(0));
        Ok(())
    }

    async fn on_update(self: &Built<Self>, value: Value) -> Result<(), String> {
        println!("Value updated: {}", value);
        Ok(())
    }

    fn id(&self) -> &str {
        "timer"
    }

    fn type_(&self) -> Type {
        Type::Integer
    }
}
