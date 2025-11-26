use cogito_openai::prelude::*;
use hypertyper::prelude::*;

// These tests aren't particularly interesting and mostly serve to ensure
// that we can actually connect to the OpenAI service.

#[tokio::test]
async fn it_sends_a_request_using_gpt_4o() {
    let auth =
        Auth::from_env("OPENAI_API_KEY").expect("Could not create auth. Is $OPENAI_API_KEY set?");
    let factory = HttpClientFactory::new(env!("CARGO_PKG_NAME"), env!("CARGO_PKG_VERSION"));
    let client = OpenAIClient::new(auth, factory);
    let req = OpenAIRequest::default()
        .model(OpenAIModel::Gpt4o)
        .input("write a haiku about ai");
    let resp = client.send(&req).await;
    let resp = resp.expect("could not make OpenAI API request");
    let line_count = resp.result().split('\n').count();
    assert!(line_count >= 3, "line count is {line_count}");
}

#[tokio::test]
async fn it_sends_a_request_using_gpt_5nano() {
    let auth =
        Auth::from_env("OPENAI_API_KEY").expect("Could not create auth. Is $OPENAI_API_KEY set?");
    let factory = HttpClientFactory::new(env!("CARGO_PKG_NAME"), env!("CARGO_PKG_VERSION"));
    let client = OpenAIClient::new(auth, factory);
    let req = OpenAIRequest::default()
        .model(OpenAIModel::Gpt5nano)
        .input("write a haiku about ai");
    let resp = client.send(&req).await;
    let resp = resp.expect("could not make OpenAI API request");
    let line_count = resp.result().split('\n').count();
    assert!(line_count >= 3, "line count is {line_count}");
}
