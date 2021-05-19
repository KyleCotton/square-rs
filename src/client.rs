/*!
The [SquareClient](crate::client::SquareClient) allows the user of the crate
to use the [Square API](https://developer.squareup.com) in an idiomatic way.

# Example: Creating a client
In order to create a client you will need your account access token that can be found
in the [Developer Apps](https://developer.squareup.com/apps) page for the specific
application you are wanting to use.

```rust
use square_rs::client::SquareClient;
let client = SquareClient::new(ACCESS_TOKEN);
```
After creating a client you will be able to use all of the clients methods.

*/
use crate::endpoint::SquareEndpoint;
use crate::error::SquareError;
use crate::response::SquareResponse;

use reqwest::{header, Client};
use serde::Serialize;
use std::default::Default;

#[derive(Copy, Clone)]
pub enum ClientMode {
    Production,
    Sandboxed,
}

/// The default mode we start a client in is Sandboxed
impl Default for ClientMode {
    fn default() -> Self {
        Self::Sandboxed
    }
}

/// The SquareClient contains many useful methods allowing for convenient
/// use of the [Square API](https://developer.squareup.com).
#[derive(Clone)]
pub struct SquareClient {
    access_token: String,
    pub(crate) client_mode: ClientMode,
}

impl SquareClient {
    /// Create a new [SquareClient](SquareClient)
    ///
    /// # Arguments
    /// * `access_token` - The access token for the Square App you
    /// want to use the client with is required.
    ///
    /// # Example: Create a new client
    /// ```
    /// let client = SquareClient::new(ACCESS_TOKEN);
    /// ```
    pub fn new(access_token: &str) -> Self {
        Self {
            access_token: access_token.to_string(),
            client_mode: Default::default(),
        }
    }

    /// Set the client to Production Mode
    ///
    /// # Arguments
    /// This method takes no arguments, as by default the client will use SandBox Mode.
    ///
    /// # Example
    /// ```
    /// let client = SquareClient::new(ACCESS_TOKEN).production();
    /// ```
    pub fn production(self) -> Self {
        Self {
            access_token: self.access_token,
            client_mode: ClientMode::Production,
        }
    }

    /// Sends a request to a given [SquareEndpoint](crate::endpoint::SquareEndpoint)
    /// # Arguments
    /// * `endpoint` - The [SquareEndpoint](crate::endpoint::SquareEndpoint) to send the request to
    /// * `body` - The json that will be included in the request.
    /// All types that meet the conditions to be deserialized to JSON are accepted.
    ///
    /// # Example:
    /// ```
    /// self.request(SquareEndpoint::Payments, &payment).await
    /// ```
    pub async fn request<T>(
        &self,
        endpoint: SquareEndpoint,
        json: &T,
    ) -> Result<SquareResponse, SquareError>
    where
        T: Serialize + ?Sized,
    {
        let url = &self.endpoint(endpoint);
        let authorization_header = format!("Bearer {}", &self.access_token);

        // Add the headers to the request
        let mut headers = header::HeaderMap::new();
        headers.insert(
            header::AUTHORIZATION,
            header::HeaderValue::from_str(&authorization_header)?,
        );

        // Create a client with the appropiate headers
        let client = Client::builder().default_headers(headers).build()?;

        // Send the request to the Square API, and get the response
        let response = client.post(url).json(json).send().await?.text().await?;

        // Deserialize the response into a SquareResponse
        Ok(serde_json::from_str(&response)?)
    }
}
