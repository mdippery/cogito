use cogito_claude::prelude::*;
use hypertyper::prelude::*;

#[tokio::test]
async fn it_sends_a_post_request() {
    let auth =
        Auth::from_env("CLAUDE_API_KEY").expect("Could not create auth. Is $CLAUDE_API_KEY set?");
    let req = ClaudeRequest::default()
        .model(ClaudeModel::Haiku45)
        .input("write a haiku about ai");
    let factory = HttpClientFactory::new(env!("CARGO_PKG_NAME"), env!("CARGO_PKG_VERSION"));
    let service = ClaudeService::new(factory);
    let response: HttpResult<ClaudeResponse> = service
        .post("https://api.anthropic.com/v1/messages", &auth, &req)
        .await;
    assert!(response.is_ok());
}
