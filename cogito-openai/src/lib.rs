// SPDX-License-Identifier: Apache-2.0
// Copyright (C) 2025 Michael Dippery <michael@monkey-robot.com>

//! An implementation of a client for the OpenAI API.
//!
//! This provider implements various traits from [cogito] to provide a uniform
//! way to accessing the OpenAI API. This makes it easy to swap out other
//! providers for OpenAI in your application, or vice versa.
//!
//! This library assumes you have set `$OPENAI_API_KEY` in your API,
//! although it is possible to configure authentication in other ways; see
//! [`OpenAIClient::new()`] for details. **This means that you are solely
//! responsible for paying the costs of API access; the Cogito developers
//! are not responsible for costs you incur while using this library.**
//! See the [`client`] module documentation for OpenAI's API pricing, or
//! visit [OpenAI's platform pricing documentation] for the latest pricing
//! information.
//!
//! [cogito]: https://docs.rs/cogito
//! [`OpenAIClient::new()`]: client::OpenAIClient::new
//! [OpenAI's platform pricing documentation]: https://platform.openai.com/docs/pricing

pub mod client;

use cogito::AIModel;
use serde::{Deserialize, Serialize};
use std::fmt;

/// Available OpenAI GPT models.
///
/// For more information on the differences between each model, see the
/// [OpenAI model documentation](https://platform.openai.com/docs/models).
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
/// naturally more powerful models cost more to use.
///
/// See the [cost breakdown](self#Cost) in the `openai` module documentation
/// for more details,  or visit OpenAI's
/// [pricing](https://platform.openai.com/docs/pricing) docs for the last prices.
///
/// [1]: https://platform.openai.com/docs/guides/text?api-mode=responses#choosing-a-model
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

impl AIModel for OpenAIModel {
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
