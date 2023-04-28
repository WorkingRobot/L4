#![feature(trait_alias)]
#![feature(async_fn_in_trait)]
#![allow(incomplete_features)]
#![allow(dead_code)]

use std::sync::Arc;

mod plugin;
mod user;
mod web;

plugins_core::export_plugin!(plugin::Plugin);

pub use user::*;
