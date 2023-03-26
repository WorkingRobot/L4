#![feature(trait_alias)]
#![feature(async_fn_in_trait)]
#![allow(incomplete_features)]

use plugins_core::Client;
use std::sync::Arc;

mod plugin;
mod web;

plugins_core::export_plugin!(register);

fn register(client: Arc<dyn Client>) -> Box<dyn plugins_core::Plugin> {
    Box::new(plugin::Plugin::new(client))
}
