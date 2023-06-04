#![feature(trait_alias)]
#![feature(async_fn_in_trait)]
#![feature(lazy_cell)]
#![allow(incomplete_features)]
#![allow(dead_code)]

mod config;
mod plugin;
mod ui;
mod user;
mod web;

pub use plugin::Plugin;
