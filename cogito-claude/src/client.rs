// SPDX-License-Identifier: Apache-2.0
// Copyright (C) 2025 Michael Dippery <michael@monkey-robot.com>

//! Anthropic Claude API client.
//!
//! When you create a client, you will have to select a [model](ClaudeModel)
//! to use. By default, the [cheapest](ClaudeModel::cheapest()) model will be
//! selected. Read the [Claude model documentation] for more information on
//! the various models offered by the OpenAI API.
//!
//! # Access
//!
//! You will need to set up a [Claude API account] and generate your own
//! authentication key to use the Claude API. Your key will be passed to the
//! [`ClaudeClient`] using a [`cogito::service::Auth`] struct.
//!
//! **Note that you are solely responsible for paying the costs of Claude API
//! access.** The Cogito developers are not responsible for costs you incur
//! while making use of the Cogito Claude service implementation.
//!
//! # Cost
//!
//! For usage costs, see the [cost breakdown] or visit the Claude platform's
//! latest [pricing docs].
//!
//! [Claude API account]: https://platform.claude.com/docs/en/home
//! [Claude model documentation]: https://platform.claude.com/docs/en/about-claude/models/overview
//! [pricing docs]: https://platform.claude.com/docs/en/about-claude/pricing
//! [cost breakdown]: crate#Cost
//! [`cogito::service::Auth`]: https://docs.rs/cogito/latest/cogito/service/struct.Auth.html

use crate::ClaudeModel;
use crate::service::ClaudeService;
use cogito::prelude::*;
use hypertyper::prelude::*;
use serde::{Deserialize, Serialize};
use std::slice::Iter;

#[cfg(doc)]
use cogito::AiModel;

/// An Anthropic Claude API client.
///
/// # Examples
///
/// Create an Anthropic Claude API client with a standard HTTP client factory
/// and authentication data:
///
/// ```
/// use cogito_claude::client::ClaudeClient;
/// use hypertyper::prelude::*;
///
/// let auth = Auth::new("my-claude-api-key");
/// let factory = HttpClientFactory::new("my-package", "v1.0.0");
/// let client = ClaudeClient::new(auth, factory);
/// ```
#[derive(Debug)]
pub struct ClaudeClient<T: HttpPost + Sync> {
    auth: Auth,
    service: T,
}

impl<T: HttpPost + Sync> AiClient for ClaudeClient<T> {
    type AiRequest = ClaudeRequest;
    type AiResponse = ClaudeResponse;

    async fn send(&self, request: &Self::AiRequest) -> AiResult<Self::AiResponse> {
        self.service.post(Self::BASE_URI, &self.auth, request).await
    }
}

impl<T: HttpPost + Sync> ClaudeClient<T> {
    /// The base URI for Claude API requests.
    const BASE_URI: &'static str = "https://api.anthropic.com/v1/messages";

    fn with_service(auth: Auth, service: T) -> Self {
        Self { auth, service }
    }
}

impl ClaudeClient<ClaudeService> {
    /// Create a new Claude API client using the given authentication data and
    /// the given factory to create underlying HTTP clients.
    pub fn new(auth: Auth, factory: HttpClientFactory) -> Self {
        let service = ClaudeService::new(factory);
        Self::with_service(auth, service)
    }
}

/// Parameters and data for a Claude API request.
///
/// # Examples
///
/// `ClaudeRequest` uses a builder pattern to build up its internal
/// structure over time, allowing you to use default values for
/// values you do not care about.
#[derive(Debug, Deserialize, Serialize)]
pub struct ClaudeRequest {
    model: ClaudeModel,
    max_tokens: u32,
    messages: Vec<ClaudeMessage>,
}

impl Default for ClaudeRequest {
    fn default() -> Self {
        Self {
            model: ClaudeModel::default(),
            max_tokens: 1024,
            messages: vec![],
        }
    }
}

impl AiRequest for ClaudeRequest {
    /// This request uses Claude API-specific [models](ClaudeModel).
    type Model = ClaudeModel;

    /// Sets the model used by the Claude API request.
    ///
    /// If not specified, the [default] model will be used. If you are on a
    /// budget, you can try using the [cheapest]  model instead by explicitly
    /// calling this method when building your API request.
    ///
    /// [default]: ClaudeModel::default()
    /// [cheapest]: ClaudeModel::cheapest()
    fn model(self, model: ClaudeModel) -> Self {
        Self { model, ..self }
    }

    fn instructions(self, instructions: impl Into<String>) -> Self {
        self.input(instructions)
    }

    fn input(self, input: impl Into<String>) -> Self {
        let message = ClaudeMessage::with_content(input);
        let mut messages = self.messages.clone();
        messages.push(message);
        Self { messages, ..self }
    }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
struct ClaudeMessage {
    role: ClaudeRole,
    content: String,
}

impl ClaudeMessage {
    pub fn with_content(content: impl Into<String>) -> Self {
        Self {
            // I think we always want to use user but who knows, the
            // documentation is sparse.
            role: ClaudeRole::User,
            content: content.into(),
        }
    }
}

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(rename_all = "lowercase")]
enum ClaudeRole {
    User,
    Assistant,
}

/// A response from the Claude API.
#[derive(Debug, Deserialize, Serialize)]
pub struct ClaudeResponse {
    id: String,

    // TODO: Use an enum, when I figure out what the possible values are
    #[serde(rename = "type")]
    response_type: String,

    role: ClaudeRole,

    content: Vec<ClaudeContent>,

    // Useful for debugging
    usage: ClaudeUsage,
}

impl AiResponse for ClaudeResponse {
    fn result(&self) -> String {
        self.content()
            .map(|c| c.text.as_str())
            .collect::<Vec<_>>()
            .join("\n")
            .trim()
            .to_string()
    }
}

impl ClaudeResponse {
    /// Claude API response output, as a series of responses.
    ///
    /// There should be at least one item in the output, but there could
    /// be multiple output objects.
    fn content(&self) -> Iter<'_, ClaudeContent> {
        self.content.iter()
    }
}

#[derive(Debug, Deserialize, Serialize)]
struct ClaudeContent {
    // TODO: Use an enum, when I figure out what the possible values are
    #[serde(rename = "type")]
    content_type: String,

    text: String,
}

#[derive(Debug, Deserialize, Serialize)]
struct ClaudeUsage {
    input_tokens: u64,
    output_tokens: u64,

    #[serde(skip_serializing_if = "Option::is_none")]
    cache_creation_input_tokens: Option<u64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    cache_read_input_tokens: Option<u64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    cache_creation: Option<ClaudeCacheCreation>,
}

#[derive(Debug, Deserialize, Serialize)]
struct ClaudeCacheCreation {
    ephemeral_5m_input_tokens: u64,
    ephemeral_1h_input_tokens: u64,
}

#[cfg(test)]
mod tests {
    use std::fs;

    fn load_data(filename: &str) -> String {
        let path = format!("tests/data/{filename}.json");
        fs::read_to_string(path).expect("could not load test data")
    }

    mod role {
        use super::super::ClaudeRole;
        use serde::{Deserialize, Serialize};

        #[derive(Debug, Serialize, Deserialize)]
        struct Object {
            pub role: ClaudeRole,
        }

        #[test]
        fn it_is_converted_to_a_lowercase_string() {
            let test_cases = [
                (ClaudeRole::User, "user"),
                (ClaudeRole::Assistant, "assistant"),
            ];

            for (role, s) in test_cases {
                let j = serde_json::to_string(&role)
                    .expect(&format!("could not serialize role {role:?}"));
                assert_eq!(j, format!("\"{s}\""), "ClaudeRole::{:?}", role);
            }
        }

        #[test]
        fn it_can_be_deserialized_from_a_string() {
            let test_cases = [
                (ClaudeRole::User, "user"),
                (ClaudeRole::Assistant, "assistant"),
            ];

            for (role, s) in test_cases {
                let json_str = format!("{{\"role\": \"{s}\"}}");
                let actual: Object = serde_json::from_str(&json_str).expect(&format!(
                    "could not deserialize role {role:?} in {json_str}"
                ));
                assert_eq!(actual.role, role, "ClaudeRole::{:?}", role);
            }
        }
    }

    mod client {
        use super::super::{ClaudeClient, ClaudeRequest};
        use super::load_data;
        use cogito::prelude::*;
        use hypertyper::prelude::*;
        use serde::Serialize;
        use serde::de::DeserializeOwned;

        #[derive(Default)]
        struct TestApiService {}

        impl HttpPost for TestApiService {
            async fn post<U, D, R>(&self, _uri: U, _auth: &Auth, _data: &D) -> HttpResult<R>
            where
                U: IntoUrl + Send,
                D: Serialize + Sync,
                R: DeserializeOwned,
            {
                let data = self.load_data();
                Ok(serde_json::from_str(&data)?)
            }
        }

        impl TestApiService {
            fn load_data(&self) -> String {
                load_data("responses_multi")
            }
        }

        impl ClaudeClient<TestApiService> {
            fn test() -> Self {
                let auth = Auth::new("some-api-key");
                ClaudeClient::with_service(auth, TestApiService::default())
            }
        }

        #[tokio::test]
        async fn it_sends_a_request_and_returns_a_response() {
            let client = ClaudeClient::test();
            let request = ClaudeRequest::default().input("Hello, world");
            let response = client.send(&request).await;
            assert!(response.is_ok());

            let response = response.unwrap();
            let text = response.result();
            assert_eq!(
                text,
                "Hello! How can I help you today?\nI am a friendly robot.\nBeep beep!"
            )
        }
    }

    mod request {
        use super::super::ClaudeRequest;
        use crate::ClaudeModel;
        use crate::client::ClaudeRole;
        use cogito::prelude::*;

        #[test]
        fn it_serializes() {
            let request = ClaudeRequest::default()
                .model(ClaudeModel::Haiku45)
                .input("Serialize me, Claude!");
            let expected = r#"{
  "model": "claude-haiku-4-5",
  "max_tokens": 1024,
  "messages": [
    {
      "role": "user",
      "content": "Serialize me, Claude!"
    }
  ]
}"#;
            let actual = serde_json::to_string_pretty(&request).expect("could not serialize json");
            assert_eq!(
                actual, expected,
                "\n\nleft:\n{actual}\n\nright:\n{expected}\n"
            )
        }

        #[test]
        fn it_deserializes() {
            let data = r#"{
  "model": "claude-haiku-4-5",
  "max_tokens": 1024,
  "messages": [
    {
      "role": "user",
      "content": "Serialize me, Claude!"
    }
  ]
}"#;
            let request: ClaudeRequest =
                serde_json::from_str(data).expect("could not deserialize json");
            assert_eq!(request.model, ClaudeModel::Haiku45);
            assert_eq!(request.max_tokens, 1024);
            assert_eq!(request.messages.len(), 1);
            let message = request
                .messages
                .iter()
                .nth(0)
                .expect("could not get message");
            assert_eq!(message.role, ClaudeRole::User);
            assert_eq!(message.content, "Serialize me, Claude!");
        }
    }

    mod response {
        use super::super::ClaudeResponse;
        use super::load_data;
        use crate::client::ClaudeRole;
        use cogito::prelude::*;

        fn load_response(filename: &str) -> ClaudeResponse {
            let data = load_data(filename);
            serde_json::from_str(&data).expect("could not parse json")
        }

        #[test]
        fn it_returns_an_id() {
            let resp = load_response("responses");
            assert_eq!(resp.id, "msg_01UiL2duVWmZVLJf83nn6gLQ");
        }

        #[test]
        fn it_returns_a_response_type() {
            let resp = load_response("responses");
            assert_eq!(resp.response_type, "message")
        }

        #[test]
        fn it_returns_its_role() {
            let resp = load_response("responses");
            assert_eq!(resp.role, ClaudeRole::Assistant);
        }

        #[test]
        fn it_returns_usage() {
            let resp = load_response("responses");
            let usage = resp.usage;
            assert_eq!(usage.input_tokens, 10);
            assert!(usage.cache_creation_input_tokens.is_some());
            assert_eq!(usage.cache_creation_input_tokens.unwrap(), 0);
            assert!(usage.cache_read_input_tokens.is_some());
            assert_eq!(usage.cache_read_input_tokens.unwrap(), 0);
            assert_eq!(usage.output_tokens, 12);

            assert!(usage.cache_creation.is_some());
            let cache = usage.cache_creation.unwrap();
            assert_eq!(cache.ephemeral_5m_input_tokens, 0);
            assert_eq!(cache.ephemeral_1h_input_tokens, 0);
        }

        #[test]
        fn it_concatenates_a_single_response() {
            let resp = load_response("responses");
            let text = resp.result();
            assert_eq!(text, "Hello! How can I help you today?");
        }

        #[test]
        fn it_concatenates_many_responses() {
            let resp = load_response("responses_multi");
            let text = resp.result();
            assert_eq!(
                text,
                "Hello! How can I help you today?\nI am a friendly robot.\nBeep beep!"
            );
        }
    }

    mod content {
        use super::super::ClaudeContent;

        #[test]
        fn it_deserializes() {
            let json_str = r#"{"type": "text", "text": "Hello! How can I help you today?"}"#;
            let content: ClaudeContent =
                serde_json::from_str(json_str).expect("could not parse json");
            assert_eq!(content.content_type, "text");
            assert_eq!(content.text, "Hello! How can I help you today?");
        }
    }

    mod usage {
        use super::super::ClaudeUsage;

        #[test]
        fn it_deserializes() {
            let json_str = r#"{
"input_tokens": 1024,
"cache_creation_input_tokens": 512,
"cache_read_input_tokens": 256,
"cache_creation": {
    "ephemeral_5m_input_tokens": 10,
    "ephemeral_1h_input_tokens": 20
},
"output_tokens": 128,
"service_tier": "standard"
}"#;
            let usage: ClaudeUsage = serde_json::from_str(json_str).expect("could not parse json");
            assert_eq!(usage.input_tokens, 1024);
            assert!(usage.cache_creation_input_tokens.is_some());
            assert_eq!(usage.cache_creation_input_tokens.unwrap(), 512);
            assert!(usage.cache_read_input_tokens.is_some());
            assert_eq!(usage.cache_read_input_tokens.unwrap(), 256);
            assert_eq!(usage.output_tokens, 128);

            assert!(usage.cache_creation.is_some());
            let cache = usage.cache_creation.unwrap();
            assert_eq!(cache.ephemeral_5m_input_tokens, 10);
            assert_eq!(cache.ephemeral_1h_input_tokens, 20);
        }

        #[test]
        fn it_deserializes_when_optional_properties_are_not_present() {
            let json_str = r#"{
"input_tokens": 1024,
"output_tokens": 128,
"service_tier": "standard"
}"#;
            let usage: ClaudeUsage = serde_json::from_str(json_str).expect("could not parse json");
            assert!(usage.cache_read_input_tokens.is_none());
            assert!(usage.cache_creation_input_tokens.is_none());
            assert!(usage.cache_creation.is_none());
        }
    }

    mod cache_creation {
        use super::super::ClaudeCacheCreation;

        #[test]
        fn it_deserializes() {
            let json_str = r#"{"ephemeral_5m_input_tokens": 10, "ephemeral_1h_input_tokens": 20}"#;
            let cache: ClaudeCacheCreation =
                serde_json::from_str(json_str).expect("could not parse json");
            assert_eq!(cache.ephemeral_5m_input_tokens, 10);
            assert_eq!(cache.ephemeral_1h_input_tokens, 20);
        }
    }
}
