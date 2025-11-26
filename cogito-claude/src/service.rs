// SPDX-License-Identifier: Apache-2.0
// Copyright (C) 2025 Michael Dippery <michael@monkey-robot.com>

//! Services for communicating with the Claude API over HTTP.
//!
//! [`Service`] acts as a proxy for Claude's API. As an implementation-agnostic
//! definition of the API service, it allows consumers to implement a single
//! API client that can communicate with Claude using various mechanisms. In
//! particular, it provides an easy way to "mock" an API client's
//! HTTP functionality in testing by providing a mocked `Service`
//! implementation for an API client under test, or an actual HTTP
//! client when the API client is used in production.
//!
//! # See Also
//!
//! - [`hypertyper.service`] for an example of how to use `Service` to mock
//!   HTTP calls.
//!
//! [`hypertyper.service`]: https://docs.rs/hypertyper/latest/hypertyper/service/index.html
//! [`Service`]: https://docs.rs/cogito/latest/cogito/service/struct.Service.html

use hypertyper::prelude::*;
use log::debug;
use reqwest::header;
use serde::Serialize;
use serde::de::DeserializeOwned;

/// Communicates with the Claude API over HTTP.
///
/// This is the "default" service used by the Claude API clients. It more or
/// less just wraps a Reqwest client, making it easier to swap out the
/// service for a deterministic service when writing tests. Most Claude API
/// clients should use this `Service` by default.
#[derive(Debug)]
pub struct ClaudeService {
    client: HttpClient,
}

impl ClaudeService {
    const ANTHROPIC_VERSION: &'static str = "2023-06-01";

    /// Creates a new HTTP service that communicates using clients from the
    /// given factory.
    pub fn new(factory: HttpClientFactory) -> Self {
        let client = factory.create();
        Self { client }
    }
}

impl HttpPost for ClaudeService {
    async fn post<U, D, R>(&self, uri: U, auth: &Auth, data: &D) -> HttpResult<R>
    where
        U: IntoUrl + Send,
        D: Serialize + Sync,
        R: DeserializeOwned,
    {
        let response = self
            .client
            .post(uri)
            .header(header::CONTENT_TYPE, "application/json")
            .header("anthropic-version", Self::ANTHROPIC_VERSION)
            .header("x-api-key", auth.api_key())
            .json(data)
            .send()
            .await?;

        debug!("HTTP response is:\n{response:?}");

        let json_object = response.json::<R>().await?;
        Ok(json_object)
    }
}
