# Cogito

> Je pense, donc je suis.  
> Cogito ergo sum.  
> I think, therefore I am.  
> **I HAVE NO MOUTH, AND I MUST SCREAM.**  
> — ChatGPT/Sam Altman

**Cogito** is your one-stop shop for rapidly and relentlessly generating AI
using the large language model of your choice. Specifically, it is a toolkit
for building _providers_, implementations of API clients for specific AI
services like OpenAI's GPT or Anthropic's Claude. Crucially, the Cogito
package provides uniform traits for the implementation of specific API
clients, allowing you to code to a specification and swap different
providers in and out as you see fit.

## Design

The heart and soul of Cogito, insofar as an artifical intelligence module
can be said to possess a heart or a soul) is `AIClient`. `AIClient` exposes
a single method, `send()`, which sends an `AIRequest` to an AI service's
API (probably by using an underlying HTTP `Service` instance).

If `AIClient` is the heart and soul of Cogito, `AIRequest` is the meat and
bones. `AIRequest` instances represent the generalized features of an AI
service, such as model specification, system prompts, and of course input.
Each AI provider defines its own `AIRequest` type, making it easy to
implement specific functionality behind a uniform interface. Similarly, each
AI provider can implement `AIResponse` to handle responses from its
associated API, returning data in a uniform way.

Finally, `Service` can be used to provide different low-level ways of
communicating with the AI service's API. Often times this will consist of an
HTTP service that talks to a live API, as well as a test API service that can
be used for mocking API calls during tests.

All of these traits, of course, are generally implemented by individual
provider crates, such as **cogito-openai**, so you rarely need to work with
these data types directly, unless you are implementing a client for an AI
service.

The design and documentation of Cogito is a moving target, so for the most
up-to-date documentation, read the source code, generate it using
`cargo doc`, or visit [docs.rs/cogito]. Or better yet, get an AI agent to do
it for you. Because this is (at least) 2025, and humans shouldn't have to do
anything anymore.

## Providers

Some providers are developed and published in this repository alongside the
core cogito package, but the core package provides a foundation for
third-party implementations as well.

Generally you will work with specific AI service providers, such as
**cogito-openai**. The **cogito** crate itself is only useful for
provider implementors, although when using a provider like cogito-openai,
you may still have to use `use cogito` to make use of some of the traits
it provides.

## Goals

Ultimately the goal of Cogito is to provide a foundation for putting all
humans out of work and bringing about the utopian e/acc vision of fully
automated luxury communism and/or the extermination of the human race.

## License

Cogito is licensed under the terms of the [Apache 2.0 license][LICENSE].
Please see the [LICENSE] file accompanying this source code or visit
[apache.org] for more information on licensing.

[apache.org]: https://www.apache.org/licenses/LICENSE-2.0
[LICENSE]: LICENSE
[docs.rs/cogito]: https://docs.rs/cogito
