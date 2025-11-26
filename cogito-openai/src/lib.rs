// SPDX-License-Identifier: Apache-2.0
// Copyright (C) 2025 Michael Dippery <michael@monkey-robot.com>

//! An implementation of a client for the OpenAI API.
//!
//! This provider implements various traits from [cogito] to provide a uniform
//! way to access the OpenAI API. This makes it easy to swap out other
//! providers for OpenAI in your application, or vice versa.
//!
//! This library assumes you pass authentication tokens for the OpenAI API
//! using [`cogito::service::Auth`]. **This means that you are solely
//! responsible for paying the costs of API access; the Cogito developers
//! are not responsible for costs you incur while using this library.**
//!
//! # Cost
//!
//! There's no such thing as a free lunch, and there's no such thing as free OpenAI access,
//! even if OpenAI is a "non-profit" that is building its technology for the betterment of
//! humanity (and not Sam Altman's bank account). When you create an OpenAI API client,
//! you will need to select an [`OpenAIModel`]. Models are billed on a per-token basis, where
//! a token is the smallest unit of text that the model reads and processes. There are three
//! types of tokens: input tokens, cached input tokens, and output tokens.
//!
//! - **Input tokens** are the token used in any _requests_ made to the OpenAPI AI. This is
//!   the "prompt" that users of this library send to OpenAI for summarization.
//! - **Cached input tokens** are input tokens that have been reused by GPT. Input tokens are
//!   reused by prompts that have a common prefix, as described
//!   [here](https://openai.com/index/api-prompt-caching/).
//! - **Output tokens** are tokens generated in the output that is sent back to a client in
//!   response to a request.
//!
//! Prices are expressed in US dollars per $1 million tokens. As of 17 July 2025, the prices
//! for each model are as follows.
//!
//! For the latest pricing, see OpenAI's [pricing][OpenAI's platform pricing documentation]
//! docs.
//!
//! | Model      | Designation        | Input    | Cached Input | Output  |
//! |------------|--------------------|---------:|-------------:|--------:|
//! | Gpt5       | gpt-5              | $1.25    | $0.125       | $10.00  |
//! | Gpt5mini   | gpt-5-mini         | $0.25    | $0.025       | $2.00   |
//! | Gpt5nano   | gpt-5-nano         | $0.05    | $0.005       | $0.40   |
//! | Gpt4_1nano | gpt-4.1-nano       | $0.10    | $0.025       | $0.40   |
//! | Gpt4omini  | gpt-4o-mini        | $0.15    | $0.075       | $0.60   |
//! | Gpt4_1mini | gpt-4.1-mini       | $0.40    | $0.10        | $1.60   |
//! | O4mini     | o4-mini            | $1.10    | $0.275       | $4.40   |
//! | O3mini     | o3-mini            | $1.10    | $0.55        | $4.40   |
//! | Gpt4_1     | gpt-4.1            | $2.00    | $0.50        | $8.00   |
//! | O3         | o3                 | $2.00    | $0.50        | $8.00   |
//! | Gpt4o      | gpt-4o             | $2.50    | $1.25        | $10.00  |
//! | ChatGpt4o  | chatgpt-4o-latest  | $5.00    | -            | $15.00  |
//! | O1         | o1                 | $15.00   | $7.50        | $60.00  |
//! | O3pro      | o3-pro             | $20.00   | -            | $80.00  |
//! | 01pro      | o1-pro             | $150.00  | -            | $600.00 |
//!
//! [cogito]: https://docs.rs/cogito
//! [`OpenAIClient::new()`]: client::OpenAIClient::new
//! [`cogito::service::Auth`]: https://docs.rs/cogito/latest/cogito/service/struct.Auth.html
//! [OpenAI's platform pricing documentation]: https://platform.openai.com/docs/pricing

pub mod client;

use cogito::AiModel;
use serde::{Deserialize, Serialize};
use std::fmt;

/// Available OpenAI GPT models.
///
/// For more information on the differences between each model, see the
/// [OpenAI model documentation].
///
/// The [default](OpenAIModel::default()) is [gpt-4o](OpenAIModel::Gpt4o),
/// which OpenAI describes as "the best model to use for most tasks".
/// [According to its docs][1], [gpt-4.1](OpenAIModel::Gpt4_1) "offers a solid
/// combination of intelligence, speed, and cost effectiveness". If you are on
/// a budget, consider using [gpt-4.1-nano](OpenAIModel::Gpt4_1nano), the
/// [least expensive](OpenAIModel::cheapest()) model.
///
/// # Cost
///
/// OpenAI API usage has a cost, and the cost of each model differs;
/// naturally, the more powerful models cost more to use. See the
/// [cost breakdown] in the `cogito_openai` module documentation
/// for more details, or visit OpenAI's [pricing] docs for the latest prices.
///
/// [1]: https://platform.openai.com/docs/guides/text?api-mode=responses#choosing-a-model
/// [cost breakdown]: self#Cost
/// [OpenAI model documentation]: https://platform.openai.com/docs/models
/// [pricing]: https://platform.openai.com/docs/pricing
#[derive(Clone, Copy, Debug, Default, PartialEq, Deserialize, Serialize)]
pub enum OpenAIModel {
    /// OpenAI's flagship model for coding, reasoning, and agentic tasks
    /// across domains.
    #[default]
    #[serde(rename = "gpt-5")]
    Gpt5,

    /// A faster, more cost-efficient version of [`GPT-5`](OpenAIModel::Gpt5).
    ///
    /// It's great for well-defined tasks and precise prompts.
    #[serde(rename = "gpt-5-mini")]
    Gpt5mini,

    /// Fastest, cheapest version of [`GPT-5`](OpenAIModel::Gpt5).
    ///
    /// It's great for summarization and classification tasks.
    #[serde(rename = "gpt-5-nano")]
    Gpt5nano,

    /// Versatile, high-intelligence flagship model.
    #[serde(rename = "gpt-4o")]
    Gpt4o,

    /// A fast, affordable model for focused tasks.
    #[serde(rename = "gpt-4o-mini")]
    Gpt4omini,

    /// The flagship model for complex tasks.
    ///
    /// It is well-suited for problem-solving across domains.
    #[serde(rename = "gpt-4.1")]
    Gpt4_1,

    /// Provides a balance between intelligence, speed, and cost.
    ///
    /// An attractive model for many use cases.
    #[serde(rename = "gpt-4.1-mini")]
    Gpt4_1mini,

    /// The fastest, most cost-effective 4.1 model.
    #[serde(rename = "gpt-4.1-nano")]
    Gpt4_1nano,

    /// Optimized for fast, effective reasoning with exceptionally efficient
    /// performance in coding and visual tasks.
    #[serde(rename = "o4-mini")]
    O4mini,

    /// A well-rounded and powerful reasoning model across domains.
    ///
    /// It sets a new standard for math, science, coding, and visual
    /// reasoning tasks, and excels at technical writing and following
    /// instructions.
    #[serde(rename = "o3")]
    O3,

    /// A mini version of the o3 model, providing high intelligence with
    /// the same cost and latency targets of o1-mini.
    #[serde(rename = "o3-mini")]
    O3mini,

    /// Like the o3 model, but it uses more compute to think even harder.
    #[serde(rename = "o3-pro")]
    O3pro,

    /// A model trained with reinforcement learning that thinks before it
    /// answers and produces a long chain of thought before responding to
    /// the user.
    #[serde(rename = "o1")]
    O1,

    /// A version of the [`o1`](OpenAIModel::O1) model that thinks even harder
    /// before responding.
    #[serde(rename = "o1-pro")]
    O1pro,
}

impl AiModel for OpenAIModel {
    /// OpenAI's standard model.
    fn flagship() -> Self {
        OpenAIModel::default()
    }

    /// The "best" GPT model as defined by OpenAI.
    fn best() -> Self {
        OpenAIModel::default()
    }

    fn cheapest() -> Self {
        OpenAIModel::Gpt5nano
    }

    fn fastest() -> Self {
        // GPT 4.1-nano is noticeably faster than GPT 5-nano.
        OpenAIModel::Gpt4_1nano
    }
}

impl fmt::Display for OpenAIModel {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = serde_json::to_string(&self)
            .unwrap_or_else(|_| panic!("could not serialize {:?}", self));
        let s = s.trim_matches('"');
        f.write_fmt(format_args!("{}", s))
    }
}

/// Convenience module for splat imports.
///
/// You can import the most common traits and data structures into your
/// project using
///
/// ```
/// use cogito_openai::prelude::*;
/// ```
pub mod prelude {
    pub use crate::OpenAIModel;
    pub use crate::client::{OpenAIClient, OpenAIRequest, OpenAIResponse};
    pub use cogito::AiModel;
    pub use cogito::client::{AiClient, AiRequest, AiResponse};
    pub use cogito::service::Service;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_returns_a_valid_display_string() {
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
            assert_eq!(model.to_string(), descriptor, "OpenAIModel::{:?}", model);
        }
    }
}
