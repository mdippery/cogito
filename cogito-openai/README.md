# cogito-openai

**cogito-openai** is an OpenAI provider that adheres to the [Cogito]
specification.

## Usage

```rust
use cogito::client::{AIClient, AIRequest, AIResponse};
use cogito_openai::OpenAIModel;
use cogito_openai::client::{OpenAIClient, OpenAIRequest};
use hypertyper::{Auth, HTTPClientFactory};

let auth = Auth::from_env("OPENAI_API_KEY")?;
let factory = HTTPClientFactory::new(env!("CARGO_PKG_NAME"), env!("CARGO_PKG_VERSION"));
let client = OpenAIClient::new(auth, factory);
let request = OpenAIRequest::default().model(OpenAIModel::Gpt5).input("Write me a haiku.");
let response = client.send(&request).await?.result();
```

See the [Cogito docs] and [Hypertyper docs] for more information.

## License

Cogito is licensed under the terms of the [Apache 2.0 license][LICENSE].
Please see the [LICENSE] file accompanying this source code or visit
[apache.org] for more information on licensing.

[Cogito]: https://crates.io/crates/cogito
[Cogito docs]: https://docs.rs/cogito
[Hypertyper docs]: https://docs.rs/hypertyper
[apache.org]: https://www.apache.org/licenses/LICENSE-2.0
[LICENSE]: LICENSE
