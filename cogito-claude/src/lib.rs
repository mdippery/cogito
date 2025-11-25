// SPDX-License-Identifier: Apache-2.0
// Copyright (C) 2025 Michael Dippery <michael@monkey-robot.com>

//! An implementation of a client for Anthropic's Claude API.
//!
//! This provider implements traits from [cogito] to provide a uniform way
//! to access Anthropic's Claude API. This makes it easy to swap out other
//! providers for Anthropic's Claude in your application, or vice versa.
//!
//! This library assumes you pass authentication tokens for the Anthropic API
//! using [`cogito::service::Auth`]. **This means that you are solely
//! responsible for paying the costs of API access; the Cogito developers
//! are not responsible for costs you incur while using this library.**
//!
//! # Cost
//!
//! There's no such thing as a free lunch, and there's no such thing as free
//! Claude access. When you create a Claude API client, you will need to
//! select a [`ClaudeModel`]. Models are billed on a per-token basis, where
//! a token is the smallest unit of text that the model reads and processes.
//! There are two types of tokens: input tokens and output tokens.
//!
//! - **Input tokens** are the token used in any _requests_ made to the
//!   Claude AI. This is the "prompt" that users of this library send to
//!   Claude for summarization.
//! - **Output tokens** are tokens generated in the output that is sent back
//!   to a client in response to a request.
//!
//! Prices are expressed in US dollars per $1 million tokens. As of
//! 25 November 2025, the prices for each model are as follows.
//!
//! For the latest pricing, see the [pricing documentation] in the Claude
//! platform documentation.
//!
//! | Model      | Designation       | Input | Output |
//! |------------|-------------------|------:|-------:|
//! | Sonnet 4.5 | claude-sonnet-4-5 | $3    | $15    |
//! | Haiku 4.5  | claude-haiku-4-5  | $1    | $5     |
//! | Opus 4.5   | claude-opus-4-5   | $5    | $25    |
//! | Opus 4.1   | claude-opus-4-1   | $15   | $75    |
//!
//! [`cogito::service::Auth`]: https://docs.rs/cogito/latest/cogito/service/struct.Auth.html
//! [pricing documentation]: https://platform.claude.com/docs/en/about-claude/pricing

use cogito::AiModel;
use serde::{Deserialize, Serialize};
use std::fmt;

/// Available Claude AI models.
///
/// For more information on the differences between each model, see the
/// [model overview] in the Claude platform documentation.
///
/// The [default model] is [Sonnet 4.5], which Anthropic describes as "our
/// smartest model for complex agents and coding". The cheapest and fastest
/// model, [Haiku 4.5], is the "fastest model with near-frontier intelligence".
/// [Opus 4.5] is a "premium model combining maximum intelligence with
/// practical performance", and [Opus 4.1] is an "exceptional model for
/// specialized reasoning tasks" (and also the most expensive!).
///
/// # Cost
///
/// Claude API usage has a cost, and the cost of each model differs; naturally,
/// the more powerful models cost more to use. See the [cost breakdown] in the
/// `cogito_claude` module documentation for more details, or visit Anthropic's
/// [pricing] documentation for the latest prices.
///
/// [cost breakdown]: self#Cost
/// [default model]: ClaudeModel::default()
/// [model overview]: https://platform.claude.com/docs/en/about-claude/models/overview
/// [pricing]: https://platform.claude.com/docs/en/about-claude/pricing
/// [Haiku 4.5]: ClaudeModel::Haiku45
/// [Opus 4.1]: ClaudeModel::Opus41
/// [Opus 4.5]: ClaudeModel::Opus45
/// [Sonnet 4.5]: ClaudeModel::Sonnet45
#[derive(Clone, Copy, Debug, Default, PartialEq, Deserialize, Serialize)]
pub enum ClaudeModel {
    /// Anthropic's flagship model.
    ///
    /// According to Anthropic, this model is "our smartest model for
    /// complex agents and coding".
    #[default]
    #[serde(rename = "claude-sonnet-4-5")]
    Sonnet45,

    /// Anthropic's fastest and cheapest model.
    ///
    /// According to Anthropic, this model is "our fastest model with
    /// near-frontier intelligence".
    #[serde(rename = "claude-haiku-4-5")]
    Haiku45,

    /// A premium model.
    ///
    /// This model "combines maximum intelligence with practical performance".
    #[serde(rename = "claude-opus-4-5")]
    Opus45,

    /// An expensive, premium model.
    ///
    /// According to Anthropic, an "exceptional model for specialize
    /// reasoning tasks".
    #[serde(rename = "claude-opus-4-1")]
    Opus41,
}

impl AiModel for ClaudeModel {
    /// Anthropic's standard model.
    fn flagship() -> Self {
        ClaudeModel::default()
    }

    /// The "best" model as defined by Anthropic.
    fn best() -> Self {
        ClaudeModel::default()
    }

    /// Anthropic's cheapest model.
    fn cheapest() -> Self {
        ClaudeModel::Haiku45
    }

    /// Anthropic's fastest model.
    fn fastest() -> Self {
        ClaudeModel::Haiku45
    }
}

impl fmt::Display for ClaudeModel {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = serde_json::to_string(&self)
            .unwrap_or_else(|_| panic!("could not serialize {:?}", self));
        let s = s.trim_matches('"');
        f.write_fmt(format_args!("{}", s))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_returns_a_valid_display_string() {
        let test_cases = vec![
            (ClaudeModel::Sonnet45, "claude-sonnet-4-5"),
            (ClaudeModel::Haiku45, "claude-haiku-4-5"),
            (ClaudeModel::Opus45, "claude-opus-4-5"),
            (ClaudeModel::Opus41, "claude-opus-4-1"),
        ];

        for (model, descriptor) in test_cases {
            assert_eq!(model.to_string(), descriptor, "ClaudeModel::{:?}", model);
        }
    }
}
