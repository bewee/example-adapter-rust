# Example Rust Adapter

[![build](https://github.com/bewee/example-adapter-rust/workflows/Build/badge.svg)](https://github.com/bewee/example-adapter-rust/actions?query=workflow:Build)
[![license](https://img.shields.io/badge/license-MPL--2.0-blue.svg)](LICENSE)

This adapter provides an example of writing adapters for the Webthings gateway in Rust.
Specifcally, a countdown device is added which has a time property, can be started via an action and emits an event on timeout.