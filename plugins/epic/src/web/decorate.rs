use reqwest::RequestBuilder;

pub trait ClientTrait {
    fn decorate_request(&self, req: RequestBuilder) -> RequestBuilder;
}

pub trait Decoratable: Sized + Into<RequestBuilder> {
    fn decorate<T: ClientTrait>(self, client: &T) -> RequestBuilder {
        client.decorate_request(self.into())
    }
}

impl Decoratable for RequestBuilder {}
