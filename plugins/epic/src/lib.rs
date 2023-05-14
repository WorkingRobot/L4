#![feature(trait_alias)]
#![feature(async_fn_in_trait)]
#![allow(incomplete_features)]
#![allow(dead_code)]

mod plugin;
mod ui;
mod user;
mod web;

pub use plugin::Plugin;
