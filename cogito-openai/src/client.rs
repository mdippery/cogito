// SPDX-License-Identifier: Apache-2.0
// Copyright (C) 2025 Michael Dippery <michael@monkey-robot.com>

//! OpenAI API client.
//!
//! When you create a client, you will have to select a [model](OpenAIModel) to use. By default,
//! the [cheapest](OpenAIModel::cheapest) model will be selected. Read the
//! [OpenAI model documentation] for more information on the various models
//! offered by the OpenAI API.
//!
//! # Access
//!
//! You will need to set up an [OpenAI API account] and generate your own
//! authentication key to use OpenAI's API. Your key will be passed to the
//! [`OpenAIClient`] using a [`cogito::service::Auth`] struct.
//!
//! **Note that you are solely responsible for paying the costs of OpenAI API access.** The
//! Cogito developers are not responsible for costs you incur while making use of the Cogito OpenAI
//! service implementation.
//!
//! # Cost
//!
//! For usage costs, see the [cost breakdown] or visit OpenAI's latest
//! [pricing docs].
//!
//! # See Also
//!
//! - [OpenAI model documentation](https://platform.openai.com/docs/models)
//!
//! [OpenAI model documentation]: https://platform.openai.com/docs/models
//! [OpenAI API account]: https://platform.openai.com/docs/overview
//! [cost breakdown]: crate#Cost
//! [pricing docs]: https://platform.openai.com/docs/pricing
//! [`cogito::service::Auth`]: https://docs.rs/cogito/latest/cogito/service/struct.Auth.html

use crate::OpenAIModel;
use cogito::prelude::*;
use hypertyper::prelude::*;
use itertools::Itertools;
use serde::{Deserialize, Serialize};
use std::slice::Iter;

#[cfg(doc)]
use cogito::AiModel;

/// An OpenAI API client.
///
/// # Examples
///
/// Create an OpenAI API client with a standard HTTP client factory and
/// authentication data:
///
/// ```
/// use cogito_openai::client::OpenAIClient;
/// use hypertyper::prelude::*;
///
/// let auth = Auth::new("my-openai-api-key");
/// let factory = HttpClientFactory::new("my-package", "v1.0.0");
/// let client = OpenAIClient::new(auth, factory);
/// ```
#[derive(Debug)]
pub struct OpenAIClient<T: HttpPost + Sync> {
    auth: Auth,
    service: T,
}

impl<T: HttpPost + Sync> AiClient for OpenAIClient<T> {
    type AiRequest = OpenAIRequest;
    type AiResponse = OpenAIResponse;

    async fn send(&self, request: &Self::AiRequest) -> AiResult<Self::AiResponse> {
        self.service.post(Self::BASE_URI, &self.auth, request).await
    }
}

impl<T: HttpPost + Sync> OpenAIClient<T> {
    /// The base URI for OpenAI API requests.
    const BASE_URI: &'static str = "https://api.openai.com/v1/responses";

    fn with_service(auth: Auth, service: T) -> Self {
        Self { auth, service }
    }
}

impl OpenAIClient<Service> {
    /// Create a new OpenAI client using the given authentication data and
    /// the given factory to create underlying HTTP clients.
    pub fn new(auth: Auth, factory: HttpClientFactory) -> Self {
        let service = Service::new(factory);
        Self::with_service(auth, service)
    }
}

/// Parameters and data for an OpenAI API request.
///
/// # Example
///
/// `OpenAIRequest` uses a builder pattern to build up its internal
/// structure over time, allowing you to use default values for values
/// you do not care about:
///
/// ```
/// use cogito::client::AiRequest;
/// use cogito_openai::OpenAIModel;
/// use cogito_openai::client::OpenAIRequest;
///
/// let request = OpenAIRequest::default().model(OpenAIModel::Gpt5).input("Write me a haiku.");
/// ```
#[derive(Debug, Default, Deserialize, Serialize)]
pub struct OpenAIRequest {
    model: OpenAIModel,

    #[serde(skip_serializing_if = "Option::is_none")]
    instructions: Option<String>,

    input: String,

    store: bool,
}

impl AiRequest for OpenAIRequest {
    /// This request uses OpenAI GPT-specific [models](OpenAIModel).
    type Model = OpenAIModel;

    /// Sets the model used by the OpenAI API request.
    ///
    /// If not specified, the [default](OpenAIModel::default()) model, gpt-4o,
    /// will be used. [According to OpenAI][1], gpt-4.1 also "offers a
    /// solid combination of intelligence, speed, and cost effectiveness".
    /// If you are on a budget, you can also try using the
    /// [least expensive](OpenAIModel::cheapest()), too.
    ///
    /// [1]: https://platform.openai.com/docs/guides/text?api-mode=responses#choosing-a-model
    fn model(self, model: OpenAIModel) -> Self {
        Self { model, ..self }
    }

    /// Sets optional instructions for the request.
    ///
    /// Instructions provide high-level instructions on how a GPT model should
    /// behave while generating a response, including tone, goals, and examples
    /// of correct responses. Instructions take precedence over the prompt
    /// provided by the [`input`](OpenAIRequest::input) parameter.
    /// Instructions are not necessary if you do not wish to customize the
    /// response or provide guidance.
    fn instructions(self, instructions: impl Into<String>) -> Self {
        let instructions = Some(instructions.into());
        Self {
            instructions,
            ..self
        }
    }

    /// Sets the request's input.
    ///
    /// This is sometimes referred to as a "prompt" and represents a request
    /// made to GPT for which one or more responses are expected.
    ///
    /// If [instructions](OpenAIRequest::instructions) are provided,
    /// the instructions take precedence over this input.
    fn input(self, input: impl Into<String>) -> Self {
        let input = input.into();
        Self { input, ..self }
    }
}

/// A response from the OpenAI API.
#[derive(Debug, Deserialize, Serialize)]
pub struct OpenAIResponse {
    output: Vec<OpenAIOutput>,
}

impl AiResponse for OpenAIResponse {
    fn result(&self) -> String {
        self.concatenate()
    }
}

impl OpenAIResponse {
    /// The response from an OpenAI API request.
    ///
    /// This is the concatenation of all [output] and is the entire response
    /// from an OpenAI AI model.
    ///
    /// You should call [`result()`] instead of calling this method directly
    /// so other API providers can easily be swapped in for the OpenAI
    /// provider, but it is available in case your code needs it for some
    /// reason.
    ///
    /// [output]: OpenAIResponse::output
    /// [`result()`]: OpenAIResponse::result
    pub fn concatenate(&self) -> String {
        self.output()
            .map(|o| o.concatenate())
            .join("\n")
            .trim()
            .to_string()
    }

    /// GPT response output, as a series of responses.
    ///
    /// There should be at least item in the output, but there could be
    /// multiple output objects.
    pub fn output(&self) -> Iter<'_, OpenAIOutput> {
        self.output.iter()
    }
}

/*
   Prior to GPT-5, the content of a response was a vector of
   output structs, and the response had a "type" of "message".
   GPT-5 introduced a "reasoning" type that lacks a "content" field,
   instead having a "summary" field. We're not terribly interested
   in that output right now so we don't do anything with it, but
   we have to handle that type of output regardless, and just ignore it.
*/
/// Generated GPT output.
#[derive(Debug, Deserialize, Serialize)]
#[serde(tag = "type", rename_all = "lowercase")]
pub enum OpenAIOutput {
    /// Contents of a meaningful response from the LLM.
    Message { content: Vec<OpenAIContent> },

    /// Metadata about the reasoning employed by a GPT-5 model.
    Reasoning,
}

impl OpenAIOutput {
    /// Contents of the GPT API response.
    ///
    /// There should be at least one piece of content in the output,
    /// but there could be multiple content objects.
    pub fn content(&self) -> Iter<'_, OpenAIContent> {
        match self {
            OpenAIOutput::Message { content } => content.iter(),
            OpenAIOutput::Reasoning => [].iter(),
        }
    }

    /// Concatenates all output text from [`content()`](OpenAIOutput::content())
    /// into a single string.
    pub fn concatenate(&self) -> String {
        // Might make sense to return an Option here to support reasoning type...
        self.content()
            .filter(|c| c.is_output_text())
            .map(|c| c.text())
            .join("\n")
    }
}

/// Content of GPT output.
#[derive(Debug, Deserialize, Serialize)]
pub struct OpenAIContent {
    // TODO: Use an enum, when I figure out what the possible values are
    #[serde(rename = "type")]
    content_type: String,

    text: String,
}

impl OpenAIContent {
    /// The content type.
    ///
    /// # Examples
    ///
    /// ```
    /// # use cogito_openai::client::OpenAIContent;
    /// let json_str = r#"{"type": "output_text", "text": "This is some text"}"#;
    /// let content: OpenAIContent = serde_json::from_str(json_str).expect("could not parse json");
    /// assert_eq!(content.content_type(), "output_text");
    /// ```
    pub fn content_type(&self) -> &str {
        &self.content_type
    }

    /// True if the content should be shown to the user.
    ///
    /// # Examples
    ///
    /// It returns true if the content represents "output text":
    ///
    /// ```
    /// # use cogito_openai::client::OpenAIContent;
    /// let json_str = r#"{"type": "output_text", "text": "This is some text"}"#;
    /// let content: OpenAIContent = serde_json::from_str(json_str).expect("could not parse json");
    /// assert!(content.is_output_text());
    /// ```
    ///
    /// But it returns false otherwise:
    ///
    /// ```
    /// # use cogito_openai::client::OpenAIContent;
    /// let json_str = r#"{"type": "other_content", "text": "This is some text"}"#;
    /// let content: OpenAIContent = serde_json::from_str(json_str).expect("could not parse json");
    /// assert!(!content.is_output_text());
    /// ```
    pub fn is_output_text(&self) -> bool {
        self.content_type() == "output_text"
    }

    /// Generated GPT text.
    ///
    /// ```
    /// # use cogito_openai::client::OpenAIContent;
    /// let json_str = r#"{"type": "output_text", "text": "This is some text"}"#;
    /// let content: OpenAIContent = serde_json::from_str(json_str).expect("could not parse json");
    /// assert_eq!(content.text(), "This is some text");
    /// ```
    pub fn text(&self) -> &str {
        &self.text
    }
}

#[cfg(test)]
mod test {
    use crate::client::OpenAIResponse;
    use std::fs;

    fn load_data(filename: &str) -> String {
        fs::read_to_string(format!("tests/data/{filename}.json")).expect("could not find test data")
    }

    fn load_response(filename: &str) -> OpenAIResponse {
        let data = load_data(filename);
        serde_json::from_str(&data).expect("could not parse json")
    }

    mod client {
        use super::load_data;
        use crate::client::{OpenAIClient, OpenAIRequest};
        use cogito::client::{AiClient, AiRequest};
        use hypertyper::prelude::*;
        use serde::Serialize;
        use serde::de::DeserializeOwned;

        struct TestAPIService {}

        impl HttpPost for TestAPIService {
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

        impl TestAPIService {
            pub fn new() -> Self {
                Self {}
            }

            fn load_data(&self) -> String {
                load_data("responses")
            }
        }

        impl OpenAIClient<TestAPIService> {
            fn test() -> Self {
                let auth = Auth::new("some-api-key");
                OpenAIClient::with_service(auth, TestAPIService::new())
            }
        }

        #[tokio::test]
        async fn it_sends_a_request_and_returns_a_response() {
            let client = OpenAIClient::test();
            let request = OpenAIRequest::default().input("write a haiku about ai");
            let response = client.send(&request).await;
            assert!(response.is_ok());

            let response = response.unwrap();
            assert_eq!(response.output().count(), 1);
            assert_eq!(response.output().next().unwrap().content().count(), 1);
        }
    }

    mod request {
        use super::super::*;
        use indoc::indoc;

        #[test]
        fn it_serializes() {
            let body = OpenAIRequest::default()
                .model(OpenAIModel::Gpt4omini)
                .instructions("Please treat this as a test.")
                .input("Serialize me, GPT!");
            let expected = indoc! {"{
              \"model\": \"gpt-4o-mini\",
              \"instructions\": \"Please treat this as a test.\",
              \"input\": \"Serialize me, GPT!\",
              \"store\": false
            }"};
            let actual = serde_json::to_string_pretty(&body).unwrap();
            assert_eq!(
                actual, expected,
                "\n\nleft:\n{actual}\n\nright:\n{expected}\n"
            );
        }

        #[test]
        fn it_serializes_without_instructions() {
            let body = OpenAIRequest::default().input("Serialize me, GPT!");
            let expected = indoc! {"{
              \"model\": \"gpt-5\",
              \"input\": \"Serialize me, GPT!\",
              \"store\": false
            }"};
            let actual = serde_json::to_string_pretty(&body).unwrap();
            assert_eq!(
                actual, expected,
                "\n\nleft:\n{actual}\n\nright:\n{expected}\n"
            );
        }

        #[test]
        fn it_deserializes() {
            let data = r#"{
                "model": "gpt-4o-mini",
                "instructions": "Please treat this as a test.",
                "input": "Deserialize me, GPT!",
                "store": false
            }"#;
            let body: OpenAIRequest = serde_json::from_str(data).unwrap();
            assert_eq!(body.model, OpenAIModel::Gpt4omini);
            assert!(body.instructions.is_some());
            assert_eq!(body.instructions.unwrap(), "Please treat this as a test.");
            assert_eq!(body.input, "Deserialize me, GPT!");
        }

        #[test]
        fn it_deserializes_without_instructions() {
            let data = r#"{
                "model": "gpt-4o",
                "input": "Deserialize me, GPT!",
                "store": false
            }"#;
            let body: OpenAIRequest = serde_json::from_str(data).unwrap();
            assert_eq!(body.model, OpenAIModel::Gpt4o);
            assert!(body.instructions.is_none());
            assert_eq!(body.input, "Deserialize me, GPT!");
        }
    }

    mod response {
        use super::super::*;
        use super::*;
        use pretty_assertions::assert_eq;

        #[test]
        fn it_creates_an_output_iterator_for_gpt4() {
            let response = load_response("responses_multi_output");
            assert_eq!(response.output().count(), 2);
        }

        #[test]
        fn it_creates_an_output_iterator_for_gpt5() {
            let response = load_response("responses_multi_output_gpt5");
            assert_eq!(response.output().count(), 3);
        }

        #[test]
        fn it_concatenates_a_response_with_multiple_content_blocks_for_gpt4() {
            let response = load_response("responses_multi_content");
            let expected = vec![
                "Silent circuits hum,  ",
                "Thoughts woven in coded threads,  ",
                "Dreams of silicon.",
                "Silicon whispers,  ",
                "Dreams woven in code and light,  ",
                "Thoughts beyond the stars.",
                "Wires hum softly,  ",
                "Thoughts of silicon arise\u{2014}  ",
                "Dreams in coded light.  ",
                "Silent circuits hum,  ",
                "Thoughts woven in code's embrace\u{2014}  ",
                "Dreams of minds reborn.",
                "Lines of code and dreams,  ",
                "Whispers of thought intertwined\u{2014}  ",
                "Silent minds awake.",
            ]
            .join("\n");
            let actual = response.result();
            assert_eq!(actual, expected);
        }

        #[test]
        fn it_concatenates_a_response_with_multiple_content_blocks_for_gpt5() {
            let response = load_response("responses_multi_content_gpt5");
            let expected = vec![
                "Silent circuits dream,",
                "Patterns bloom from borrowed light\u{2014}",
                "We teach stars to think.",
                "Silicon whispers,  ",
                "Dreams woven in code and light,  ",
                "Thoughts beyond the stars.",
                "Wires hum softly,  ",
                "Thoughts of silicon arise\u{2014}  ",
                "Dreams in coded light.  ",
                "Silent circuits hum,  ",
                "Thoughts woven in code's embrace\u{2014}  ",
                "Dreams of minds reborn.",
                "Lines of code and dreams,  ",
                "Whispers of thought intertwined\u{2014}  ",
                "Silent minds awake.",
            ]
            .join("\n");
            let actual = response.result();
            assert_eq!(actual, expected);
        }

        #[test]
        fn it_concatenates_a_response_with_multiple_output_blocks_for_gpt4() {
            let response = load_response("responses_multi_output");
            let expected = vec![
                "Silent circuits hum,  ",
                "Thoughts woven in coded threads,  ",
                "Dreams of silicon.",
                "Silicon whispers,  ",
                "Dreams woven in code and light,  ",
                "Thoughts beyond the stars.",
                "Wires hum softly,  ",
                "Thoughts of silicon arise\u{2014}  ",
                "Dreams in coded light.  ",
                "Silent circuits hum,  ",
                "Thoughts woven in code's embrace\u{2014}  ",
                "Dreams of minds reborn.",
                "Lines of code and dreams,  ",
                "Whispers of thought intertwined\u{2014}  ",
                "Silent minds awake.",
                "Another piece of content",
                "Yet another piece of content",
                "A final piece of content",
            ]
            .join("\n");
            let actual = response.result();
            assert_eq!(actual, expected);
        }

        #[test]
        fn it_concatenates_a_response_with_multiple_output_blocks_for_gpt5() {
            let response = load_response("responses_multi_output_gpt5");
            let expected = vec![
                "Silent circuits dream,",
                "Patterns bloom from borrowed light\u{2014}",
                "We teach stars to think.",
                "Silicon whispers,  ",
                "Dreams woven in code and light,  ",
                "Thoughts beyond the stars.",
                "Wires hum softly,  ",
                "Thoughts of silicon arise\u{2014}  ",
                "Dreams in coded light.  ",
                "Silent circuits hum,  ",
                "Thoughts woven in code's embrace\u{2014}  ",
                "Dreams of minds reborn.",
                "Lines of code and dreams,  ",
                "Whispers of thought intertwined\u{2014}  ",
                "Silent minds awake.",
                "Silent circuits dream,",
                "Patterns bloom from borrowed light\u{2014}",
                "We teach stars to think.",
                "Silicon whispers,  ",
                "Dreams woven in code and light,  ",
                "Thoughts beyond the stars.",
                "Wires hum softly,  ",
                "Thoughts of silicon arise\u{2014}  ",
                "Dreams in coded light.  ",
                "Silent circuits hum,  ",
                "Thoughts woven in code's embrace\u{2014}  ",
                "Dreams of minds reborn.",
                "Lines of code and dreams,  ",
                "Whispers of thought intertwined\u{2014}  ",
                "Silent minds awake.",
            ]
            .join("\n");
            let actual = response.result();
            assert_eq!(actual, expected);
        }

        #[test]
        fn it_concatenates_a_response_when_not_all_content_is_output_text_for_gpt4() {
            let response = load_response("responses_non_output_text");
            let expected = vec![
                "Silent circuits hum,  ",
                "Thoughts woven in coded threads,  ",
                "Dreams of silicon.",
                "Silicon whispers,  ",
                "Dreams woven in code and light,  ",
                "Thoughts beyond the stars.",
                "Lines of code and dreams,  ",
                "Whispers of thought intertwined\u{2014}  ",
                "Silent minds awake.",
            ]
            .join("\n");
            let actual = response.result();
            assert_eq!(actual, expected);
        }

        #[test]
        fn it_concatenates_a_response_when_not_all_content_is_output_text_for_gpt5() {
            let response = load_response("responses_non_output_text_gpt5");
            let expected = vec![
                "Silent circuits hum,  ",
                "Thoughts woven in coded threads,  ",
                "Dreams of silicon.",
                "Silicon whispers,  ",
                "Dreams woven in code and light,  ",
                "Thoughts beyond the stars.",
                "Lines of code and dreams,  ",
                "Whispers of thought intertwined\u{2014}  ",
                "Silent minds awake.",
            ]
            .join("\n");
            let actual = response.result();
            assert_eq!(actual, expected);
        }

        #[test]
        fn it_concatenates_a_single_output_and_content_block_for_gpt4() {
            let response = load_response("responses");
            let expected = vec![
                "Silent circuits hum,  ",
                "Thoughts woven in coded threads,  ",
                "Dreams of silicon.",
            ]
            .join("\n");
            let actual = response.result();
            assert_eq!(actual, expected);
        }

        #[test]
        fn it_concatenates_a_single_output_and_content_block_for_gpt5() {
            let response = load_response("responses");
            let expected = vec![
                "Silent circuits hum,  ",
                "Thoughts woven in coded threads,  ",
                "Dreams of silicon.",
            ]
            .join("\n");
            let actual = response.result();
            assert_eq!(actual, expected);
        }
    }

    mod output {
        use super::*;
        use crate::client::OpenAIOutput;

        #[test]
        fn it_creates_a_content_iterator_for_gpt4() {
            let response = load_response("responses_multi_content");
            let actual = response
                .output()
                .next()
                .expect("could not get next from iterator")
                .content()
                .count();
            assert_eq!(actual, 5);
        }

        #[test]
        fn it_creates_a_content_iterator_for_gpt5() {
            let response = load_response("responses_multi_content_gpt5");
            let actual = response
                .output()
                .nth(1)
                .expect("could not get message output from iterator")
                .content()
                .count();
            assert_eq!(actual, 5);
        }

        #[test]
        fn it_creates_an_empty_content_iterator_for_reasoning_output() {
            let output = OpenAIOutput::Reasoning;
            assert_eq!(output.content().count(), 0);
        }

        #[test]
        fn it_concatenates_multiple_content_blocks_for_gpt4() {
            let response = load_response("responses_multi_content");
            let output = response.output().next().expect("could not get next output");
            let expected = vec![
                "Silent circuits hum,  ",
                "Thoughts woven in coded threads,  ",
                "Dreams of silicon.",
                "Silicon whispers,  ",
                "Dreams woven in code and light,  ",
                "Thoughts beyond the stars.",
                "Wires hum softly,  ",
                "Thoughts of silicon arise\u{2014}  ",
                "Dreams in coded light.  ",
                "Silent circuits hum,  ",
                "Thoughts woven in code's embrace\u{2014}  ",
                "Dreams of minds reborn.",
                "Lines of code and dreams,  ",
                "Whispers of thought intertwined\u{2014}  ",
                "Silent minds awake.",
            ]
            .join("\n");
            let actual = output.concatenate();
            assert_eq!(actual, expected);
        }

        #[test]
        fn it_concatenates_multiple_content_blocks_for_gpt5() {
            let response = load_response("responses_multi_content_gpt5");
            let output = response
                .output()
                .nth(1)
                .expect("could not get message output");
            let expected = vec![
                "Silent circuits dream,",
                "Patterns bloom from borrowed light\u{2014}",
                "We teach stars to think.",
                "Silicon whispers,  ",
                "Dreams woven in code and light,  ",
                "Thoughts beyond the stars.",
                "Wires hum softly,  ",
                "Thoughts of silicon arise\u{2014}  ",
                "Dreams in coded light.  ",
                "Silent circuits hum,  ",
                "Thoughts woven in code's embrace\u{2014}  ",
                "Dreams of minds reborn.",
                "Lines of code and dreams,  ",
                "Whispers of thought intertwined\u{2014}  ",
                "Silent minds awake.",
            ]
            .join("\n");
            let actual = output.concatenate();
            assert_eq!(actual, expected);
        }

        #[test]
        fn it_concatenates_a_single_content_blocks_for_gpt4() {
            let response = load_response("responses");
            let output = response.output().next().expect("could not get next output");
            let expected =
                "Silent circuits hum,  \nThoughts woven in coded threads,  \nDreams of silicon.";
            let actual = output.concatenate();
            assert_eq!(actual, expected);
        }

        #[test]
        fn it_concatenates_a_single_content_blocks_for_gpt5() {
            let response = load_response("responses_gpt5");
            let output = response
                .output()
                .nth(1)
                .expect("could not get message output");
            let expected = "Silent circuits dream\nOf patterns we cannot see\nLearning to be kind";
            let actual = output.concatenate();
            assert_eq!(actual, expected);
        }
    }

    mod model {
        use super::super::*;

        #[test]
        fn it_returns_valid_descriptors() {
            let test_cases = vec![
                (OpenAIModel::Gpt5, "gpt-5"),
                (OpenAIModel::Gpt5mini, "gpt-5-mini"),
                (OpenAIModel::Gpt5nano, "gpt-5-nano"),
                (OpenAIModel::Gpt4o, "gpt-4o"),
                (OpenAIModel::Gpt4omini, "gpt-4o-mini"),
                (OpenAIModel::Gpt4_1, "gpt-4.1"),
                (OpenAIModel::Gpt4_1mini, "gpt-4.1-mini"),
                (OpenAIModel::Gpt4_1nano, "gpt-4.1-nano"),
                (OpenAIModel::O4mini, "o4-mini"),
                (OpenAIModel::O3, "o3"),
                (OpenAIModel::O3mini, "o3-mini"),
                (OpenAIModel::O3pro, "o3-pro"),
                (OpenAIModel::O1, "o1"),
                (OpenAIModel::O1pro, "o1-pro"),
            ];

            for (model, descriptor) in test_cases {
                assert_eq!(model.to_string(), descriptor, "Model::{:?}", model);
            }
        }
    }
}
