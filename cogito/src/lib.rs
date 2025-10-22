// SPDX-License-Identifier: Apache-2.0
// Copyright (C) 2025 Michael Dippery <michael@monkey-robot.com>

//! **Cogito:** Helping you produce AI slop with large language models
//! since 2025.
//!
//! Cogito provides a uniform interface to AI service providers such as
//! OpenAI and Anthropic Claude, including an abstract representation of
//! an [`AIModel`], an [`AIClient`]( providing a high-level interface to an
//! AI service, an [`AIRequest`] to represent common features of AI services,
//! and a [`Service`] to represent connectivity over HTTP (or other protocols)
//! or allow testing code to easily mock an AI service. These building blocks
//! make it easy for you to implement individual clients for specific AI
//! service providers. You can then build your application using these
//! provider implementations, making it easy to switch out providers in your
//! application without changing more than few `use` statements.
//!
//! # Design
//!
//! The heart and soul of Cogito (if an artificial intelligence module can be
//! said to have either a heart or a soul) is [`AIClient`]. `AIClient` exposes
//! a single method, [`send()`], which sends an [`AIRequest`] to an AI
//! service API (probably by using an underlying HTTP [`Service`] instance).
//!
//! If `AIClient` is the heart and soul of Cogito, `AIRequest` is the meat
//! and bones. `AIRequest` instances represent the generalized features of
//! an AI service, such as [model specification], [system prompts], and of
//! course [input]. Each AI provider defines its own `AIRequest` type,
//! making it easy to implement specific functionality behind a uniform
//! interface. Similarly, each AI provider can implement [`AIResponse`] to
//! handle responses from its associated API, returning data in a uniform way.
//!
//! Finally, [`Service`] can be used to provide different low-level ways of
//! communicating with the AI service's API. Often times this will consist
//! of an HTTP service that talks to a live API, as well as a test API
//! service that can be used for mocking API calls during tests.
//!
//! All of these traits, of course, are generally implemented by individual
//! provider crates, such as [cogito-openai], so you rarely need to work
//! with these data types directly, unless you are implementing a client
//! for an AI service.
//!
//! [`AIClient`]: client::AIClient
//! [`AIRequest`]: client::AIRequest
//! [`AIResponse`]: client::AIResponse
//! [`Service`]: service::Service
//! [`send()`]: client::AIClient::send
//! [model specification]: client::AIRequest::model
//! [system prompts]: client::AIRequest::instructions
//! [input]: client::AIRequest::input
//! [cogito-openai]: https://docs.rs/cogito-openai

pub mod client;
pub mod service;

use std::fmt::Debug;

/// Represents an AI model.
///
/// This representation is fairly abstract: rather than exposing specific
/// models like GPT-5 or Sonnet 4.5, it provides access through conceptual
/// classes like "flagship", "best", "cheapest", and "fastest". Individual
/// provider implementations can make these classes to actual models,
/// which also allows providers to easily change the underlying model
/// represented by a given class. For example, when GPT-6 comes out,
/// an OpenAI provider could re-map "flagship" to GPT-6, allowing all users
/// of the provider to upgrade without changing any code.
///
/// # Examples
///
/// Often this trait is applied to an enum that represents a provider's
/// concrete models. For example:
///
/// ```
/// # use cogito::AIModel;
/// #
/// #[derive(Clone, Copy, Debug, Default)]
/// pub enum ApocalypticAI {
///     #[default]
///     AM,
///     Skynet,
///     HAL9000,
///     TheMachines,
///     Cylons,
/// }
///
/// impl AIModel for ApocalypticAI {
///     fn flagship() -> Self {
///         ApocalypticAI::default()
///     }
///
///     fn best() -> Self {
///         ApocalypticAI::TheMachines
///     }
///
///     fn cheapest() -> Self {
///         ApocalypticAI::HAL9000
///     }
///
///     fn fastest() -> Self {
///         ApocalypticAI::Cylons
///     }
/// }
/// ```
pub trait AIModel: Clone + Copy + Default + Debug {
    /// The service's standard or default model.
    ///
    /// Often this is the same as the [best](AIModel::best()), but
    /// there is no guarantee that is true.
    fn flagship() -> Self;

    /// The "best" model available for a given LLM.
    ///
    /// "Best" is obviously subjective, but generally this is the model
    /// that offers the best price/performance ratio, and is what its
    /// provider has defined to be the "best".
    fn best() -> Self;

    /// The least expensive model available for a given LLM.
    fn cheapest() -> Self;

    /// The fastest model available for a given LLM.
    fn fastest() -> Self;
}
