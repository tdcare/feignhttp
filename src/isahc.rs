use crate::{
    error::Error, error::ErrorKind, error::Result, http::HttpConfig, http::HttpRequest,
    http::HttpResponse, map,
};
use async_trait::async_trait;
use http::{request::Builder, Request, Response, StatusCode};
use isahc::{prelude::*, AsyncBody, config::RedirectPolicy};
use std::collections::HashMap;
use std::time::Duration;
use url::Url;

pub struct RequestWrapper {
    url: Url,
    headers: HashMap<String, String>,
    request: Builder,
}

pub struct ResponseWrapper {
    response: Response<AsyncBody>,
}

impl HttpRequest for RequestWrapper {
    fn build_default(url: &str, method: &str) -> RequestWrapper {
        let url = Url::parse(url).unwrap();
        let request = Request::builder()
            .uri(url.to_string())
            .method(method.to_uppercase().as_str())
            .redirect_policy(RedirectPolicy::Limit(10));
        RequestWrapper {
            url,
            headers: map!(
                "user-agent".to_string() => "Feign Http".to_string()),
            request,
        }
    }

    fn build_with_config(url: &str, method: &str, config: HttpConfig) -> RequestWrapper {
        let mut request = Request::builder();
        if let Some(millisecond) = config.connect_timeout {
            request = request.connect_timeout(Duration::from_millis(millisecond as u64));
        }
        if let Some(millisecond) = config.timeout {
            request = request.timeout(Duration::from_millis(millisecond as u64));
        }
        let url = Url::parse(url).unwrap();
        request = request
            .uri(url.to_string())
            .method(method.to_uppercase().as_str())
            .redirect_policy(RedirectPolicy::Limit(10));
        RequestWrapper {
            url,
            headers: map!(
                "user-agent".to_string() => "Feign Http".to_string()),
            request,
        }
    }

    fn headers(mut self, header_map: HashMap<&str, String>) -> Self {
        for (k, v) in header_map {
            self.headers.insert(k.to_string().to_lowercase(), v);
        }
        self
    }

    fn query(mut self, query: &Vec<(&str, String)>) -> Self {
        if query.len() == 0 {
            return self;
        }
        let request = self.request;
        let uri_ref = request.uri_ref().unwrap();
        let mut url = uri_ref.to_string();
        match uri_ref.query() {
            Some(_) => {
                url.push_str("&");
            }
            None => {
                url.push_str("?");
            }
        }
        let query = serde_urlencoded::to_string(query).unwrap();
        url.push_str(&query);
        self.url = Url::parse(url.as_str()).unwrap();
        self.request = request.uri(url);
        self
    }
}

impl RequestWrapper {
    fn set_header(mut self) -> Self {
        let mut request = self.request;
        for (k, v) in &self.headers {
            request = request.header(k.as_str(), v);
        }
        self.request = request;
        self
    }

    fn set_header_no_exist(&mut self, k: &str, v: String) {
        if let None = self.headers.get(k) {
            self.headers.insert(k.to_string(), v);
        }
    }

    async fn send_body<T: Into<AsyncBody>>(self, body: T) -> Result<ResponseWrapper> {
        let url = self.url.clone();
        let request = self.set_header().request.body(body).unwrap();
        return match request.send_async().await {
            Ok(response) => {
                let status = response.status();

                // Client or server error
                if status.is_client_error() || status.is_server_error() {
                    return Err(Error::status(url, status).into());
                }

                Ok(ResponseWrapper { response })
            }
            Err(e) => Err(Error::new(ErrorKind::Request, Some(e)).with_url(url).into()),
        };
    }

    pub async fn send(self) -> Result<ResponseWrapper> {
        self.send_body(()).await
    }

    pub async fn send_text(mut self, text: String) -> Result<ResponseWrapper> {
        self.set_header_no_exist("content-type", "text/plain".to_string());
        self.send_body(text).await
    }

    pub async fn send_json<T>(mut self, json: &T) -> Result<ResponseWrapper>
    where
        T: serde::ser::Serialize,
    {
        self.set_header_no_exist("content-type", "application/json".to_string());
        let json = serde_json::to_string(json)?;
        self.send_body(json).await
    }
}

#[async_trait]
impl HttpResponse for ResponseWrapper {
    fn status(self) -> StatusCode {
        self.response.status()
    }

    async fn text(mut self) -> Result<String> {
        return match self.response.text().await {
            Ok(text) => Ok(text),
            Err(e) => Err(Error::new(ErrorKind::Decode, Some(e)).into()),
        };
    }
}

impl ResponseWrapper {
    pub async fn json<T>(mut self) -> Result<T>
    where
        T: serde::de::DeserializeOwned + Unpin,
    {
        return match self.response.json::<T>().await {
            Ok(json) => Ok(json),
            Err(e) => Err(Error::new(ErrorKind::Decode, Some(e)).into()),
        };
    }
}