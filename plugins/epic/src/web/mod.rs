mod client;
mod client_authed;
mod credentials;
mod decorate;
mod error;
mod hostname;
mod request;
pub mod responses;

pub use client::Client;
pub use client_authed::ClientAuthed;
pub use credentials::Credentials;
pub use decorate::{ClientTrait, Decoratable};
pub use error::{Error, FutureResult, Result};
pub use hostname::{format_url, Hostname};
pub use request::Sendable;
