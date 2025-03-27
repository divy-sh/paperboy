use serde_json::json;
pub struct RestRequest {}

pub async fn load_curl_request() {
    let input = r#"curl \
    -X PATCH \
    -d '{"visibility":"private"}' \
    -H "Accept: application/vnd.github+json" \
    -H "Authorization: Bearer {{ token }}"\
    -H "X-GitHub-Api-Version: 2022-11-28" \
    https://api.github.com/user/email/visibility "#;
    let context = json!({ "token": "abcd1234" });
    let parsed = curl_parser::ParsedRequest::load(input, Some(context)).unwrap();
    println!("{:#?}", parsed);
    let request = reqwest::RequestBuilder::try_from(&parsed).unwrap();
    println!("{:#?}", request);
}
