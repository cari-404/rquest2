use rquest::tls::Impersonate;
use std::error::Error;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    // Build a client to mimic Edge127
    let client = rquest::Client::builder()
        .impersonate(Impersonate::Edge127)
        .enable_ech_grease()
        .permute_extensions()
        .tls_info(true)
        .build()?;

    // Use the API you're already familiar with
    let resp = client.get("https://tls.peet.ws/api/all").send().await?;
    if let Some(val) = resp.extensions().get::<rquest::tls::TlsInfo>() {
        if let Some(peer_cert_der) = val.peer_certificate() {
            assert!(!peer_cert_der.is_empty());
        }
    }

    Ok(())
}
