// SPDX-License-Identifier: Apache-2.0
// Copyright (C) 2025 Michael Dippery <michael@monkey-robot.com>

// TODO: Library docs

pub mod client;
pub mod service;

use std::fmt::Debug;

/// An AI model specification.
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
