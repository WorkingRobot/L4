use plugins_core::async_trait;
use serde::de::DeserializeOwned;

use super::Result;

#[async_trait]
pub trait Sendable: Sized {
    async fn send_into<T: DeserializeOwned>(self) -> Result<T>;
}

#[async_trait]
impl Sendable for reqwest::RequestBuilder {
    async fn send_into<T: DeserializeOwned>(self) -> Result<T> {
        Ok(self.send().await?.json::<T>().await?)
    }
}
