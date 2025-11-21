// These tests aren't particularly interesting and mostly serve to ensure
// that we can actually connect to the OpenAI service. Somewhat redundant
// with openai_client_https, but it ensures we are testing the integration
// of each individual component.

use cogito::client::AIRequest;
use cogito::service::Service;
use cogito_openai::OpenAIModel;
use cogito_openai::client::{OpenAIRequest, OpenAIResponse};
use hypertyper::prelude::*;

#[tokio::test]
async fn it_sends_a_post_request_using_gpt4o() {
    let auth =
        Auth::from_env("OPENAI_API_KEY").expect("Could not create auth. Is $OPENAI_API_KEY set?");
    let request = OpenAIRequest::default()
        .model(OpenAIModel::Gpt4o)
        .input("write a haiku about ai");
    let factory = HttpClientFactory::new(env!("CARGO_PKG_NAME"), env!("CARGO_PKG_VERSION"));
    let service = Service::new(factory);
    let response: HttpResult<OpenAIResponse> = service
        .post("https://api.openai.com/v1/responses", &auth, &request)
        .await;
    let resp = response.expect("could not make OpenAI API request");
    assert_eq!(resp.output().count(), 1);
    assert_eq!(resp.output().next().unwrap().content().count(), 1);
}

#[tokio::test]
async fn it_sends_a_post_request_using_gpt5nano() {
    let auth =
        Auth::from_env("OPENAI_API_KEY").expect("Could not create auth. Is $OPENAI_API_KEY set?");
    let request = OpenAIRequest::default()
        .model(OpenAIModel::Gpt5nano)
        .input("write a haiku about ai");
    let factory = HttpClientFactory::new(env!("CARGO_PKG_NAME"), env!("CARGO_PKG_VERSION"));
    let service = Service::new(factory);
    let response: HttpResult<OpenAIResponse> = service
        .post("https://api.openai.com/v1/responses", &auth, &request)
        .await;
    let resp = response.expect("could not make OpenAI API request");
    assert_eq!(resp.output().count(), 2);

    let output = resp.output().nth(1).expect(&format!(
        "could not get second element of output: {:?}",
        resp
    ));
    assert_eq!(output.content().count(), 1);
}
