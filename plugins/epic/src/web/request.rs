use reqwest::RequestBuilder;
use serde::de::DeserializeOwned;

use super::Result;

pub trait Sendable: Sized {
    async fn send_into<T: DeserializeOwned>(self) -> Result<T>;

    async fn send_into_empty(self) -> Result<()>;
}

impl Sendable for RequestBuilder {
    async fn send_into<T: DeserializeOwned>(self) -> Result<T> {
        Ok(self.send().await?.json::<T>().await?)
    }

    async fn send_into_empty(self) -> Result<()> {
        if self.send().await?.bytes().await?.is_empty() {
            Ok(())
        } else {
            Err(super::Error::ResponseShouldBeEmpty)
        }
    }
}
