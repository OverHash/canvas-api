use reqwest::{
    header::{HeaderMap, HeaderValue, AUTHORIZATION},
    Client, RequestBuilder,
};

const BASE_API_URL: &str = "https://canvas.instructure.com/api";

/// Represents the main canvas client that implements API functionality.
pub struct CanvasClient {
    /// The HTTP client to make requests with.
    http_client: Client,
    /// The base API url for each request.
    api_url: String,
}

pub struct CanvasClientBuilder {
    /// The configuration to create the [`CanvasClient`] with.
    config: CanvasClientConfig,
}

struct CanvasClientConfig {
    /// The user token for making requests.
    canvas_token: String,
    /// The url for API requests.
    api_url: String,
}

impl CanvasClient {
    /// Creates a new [`CanvasClientBuilder`] to configure a [`CanvasClient`].
    ///
    /// This is the same as [`CanvasClientBuilder::new()`].
    pub fn builder(canvas_token: String) -> CanvasClientBuilder {
        CanvasClientBuilder::new(canvas_token)
    }

    pub(crate) fn make_query(&self, path: &str) -> RequestBuilder {
        self.http_client.get(format!("{}/{path}", self.api_url))
    }

    pub(crate) fn make_put(&self, path: &str) -> RequestBuilder {
        self.http_client.put(format!("{}/{path}", self.api_url))
    }
}

impl CanvasClientBuilder {
    /// Creates a new [`CanvasClientBuilder`] to configure a [`CanvasClient`].
    pub fn new(canvas_token: String) -> Self {
        Self {
            config: CanvasClientConfig {
                canvas_token,
                api_url: BASE_API_URL.to_string(),
            },
        }
    }

    /// Sets the base API url to perform requests to. Defaults to [`BASE_API_URL`] if not specified.
    pub fn set_api_url(mut self, api_url: impl Into<String>) -> CanvasClientBuilder {
        self.config.api_url = api_url.into();

        self
    }

    /// Builds the [`CanvasClient`], returning an error if the client could not be built.
    pub fn build(self) -> Result<CanvasClient, crate::Error> {
        let mut default_client_headers = HeaderMap::new();

        default_client_headers.insert(
            AUTHORIZATION,
            HeaderValue::from_str(&format!("Bearer {}", self.config.canvas_token))
                .map_err(|e| crate::Error::CreatingHeader { header: e })?,
        );

        Ok(CanvasClient {
            http_client: Client::builder()
                .default_headers(default_client_headers)
                .build()?,
            api_url: self.config.api_url,
        })
    }
}
