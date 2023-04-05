#![feature(trait_alias)]
#![feature(async_fn_in_trait)]
#![allow(incomplete_features)]

use std::sync::Arc;

mod plugin;
mod web;

plugins_core::export_plugin!(plugin::Plugin);
