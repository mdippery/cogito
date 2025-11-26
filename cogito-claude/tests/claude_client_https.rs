use cogito::prelude::*;
use cogito_claude::prelude::*;
use hypertyper::prelude::*;

// These tests aren't particularly interesting and mostly serve to ensure
// that we can actually connect to the Claude API service.

#[tokio::test]
async fn it_sends_a_request() {
    let auth =
        Auth::from_env("CLAUDE_API_KEY").expect("Could not create auth. Is $CLAUDE_API_KEY set?");
    let factory = HttpClientFactory::new(env!("CARGO_PKG_NAME"), env!("CARGO_PKG_VERSION"));
    let client = ClaudeClient::new(auth, factory);
    let req = ClaudeRequest::default()
        .model(ClaudeModel::Haiku45)
        .input("write a haiku about ai");
    let resp = client
        .send(&req)
        .await
        .expect("could not make Claude API request");
    let result = resp.result();
    let line_count = resp.result().split('\n').count();
    assert!(line_count >= 3, "line count is {line_count}:\n{result}");
}
