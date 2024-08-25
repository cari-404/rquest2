use rquest::tls::Impersonate;
use rquest::{ClientBuilder, header::HeaderMap, Version, StatusCode};
use rquest::header::HeaderValue;
use std::error::Error;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let mut headers = rquest::header::HeaderMap::new();
    headers.insert("User-Agent", HeaderValue::from_static("Android app Shopee appver=29333 app_type=13"));
    headers.insert("Connection", HeaderValue::from_static("keep-alive"));
    headers.insert("x-shopee-language", HeaderValue::from_static("application/json"));
    headers.insert("if-none-match-", HeaderValue::from_static("8001"));
    headers.insert("x-api-source", HeaderValue::from_static("rn"));
    headers.insert("origin", HeaderValue::from_static("https://shopee.co.id"));
    //headers.insert("referer", rquest::header::HeaderValue::from_str(&refe)?);
    headers.insert("accept-language", HeaderValue::from_static("id-ID,id;q=0.9,en-US;q=0.8,en;q=0.7,fr;q=0.6,es;q=0.5"));
    //headers.insert("x-csrftoken", rquest::header::HeaderValue::from_str(csrftoken)?);
    //headers.insert("cookie", rquest::header::HeaderValue::from_str(cookie_content)?);
    // Build a client to mimic Edge127 with headers
    let client = rquest::Client::builder()
        .danger_accept_invalid_certs(true)
        .impersonate_with_headers(Impersonate::Chrome127, false)
        .enable_ech_grease()
        .permute_extensions()
        //.gzip(true)
        .build()?;

    // Use the API you're already familiar with
    let resp = client
        .get("https://tls.peet.ws/api/all")
        .headers(headers)
        .version(Version::HTTP_2) 
        .send()
        .await?;
    println!("{}", resp.text().await?);

    Ok(())
}
