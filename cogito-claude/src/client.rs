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

impl ClaudeClient<Service> {
    /// Create a new Claude API client using the given authentication data and
    /// the given factory to create underlying HTTP clients.
    pub fn new(auth: Auth, factory: HttpClientFactory) -> Self {
        let service = Service::new(factory);
        Self::with_service(auth, service)
    }
}

/* TODO: Test these JSON response models.
 *
 * But also, in cogito-openai, the individual submodels are pub, whereas
 * here, they are not pub; and in cogito-openai, there are doctests for
 * some of the simpler submodels, but we cannot have doc tests for
 * non-pub structs.
 *
 * We should be consistent across crates, so either we should make these
 * submodels pub and write doctests for them, or we should make the
 * submodels in cogito-openai non-pub and then necessarily change the
 * tests from doctests to regular tests (which might be better for
 * performance reasons anyway).
 *
 * I hate to pollute the API with unnecessary structs, but some of the
 * submodels, such as ClaudeUsage, might actually be useful to consumers
 * of this crate.
 *
 * But then I also don't feel like going through and writing a ton of
 * simple accessor methods, so if these structs do become pub, then maybe
 * the attributes should just be pub as well; but I kind of hate that
 * because then what if I want to write more complicated accessors
 * eventually? Also, if attributes for submodels are pub here, they should
 * be pub in cogito-openai, too, for consistency...but I already wrote all
 * those nice accessor methods in cogito-openai, and I hate to get rid of
 * them.
 */

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
    // TODO: This is private. Make OpenAIResponse::output() and
    //       OpenAIResponse::concatenate() private to match?
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
    cache_creation_input_tokens: u64,
    cache_read_input_tokens: u64,
    cache_creation: ClaudeCacheCreation,
}

#[derive(Debug, Deserialize, Serialize)]
struct ClaudeCacheCreation {
    ephemeral_5m_input_tokens: u64,
    ephemeral_1h_input_tokens: u64,
}

#[cfg(test)]
mod tests {
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
}
