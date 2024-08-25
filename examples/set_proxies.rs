use rquest::tls::Impersonate;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Build a client to mimic Chrome126
    let mut client = rquest::Client::builder()
        .impersonate(Impersonate::Chrome126)
        .enable_ech_grease()
        .permute_extensions()
        .build()?;

    let resp = client.get("https://api.ip.sb/ip").send().await?;
    println!("{}", resp.text().await?);

    let proxy = rquest::Proxy::all("socks5h://127.0.0.1:1080")?;
    client.set_proxies(&[proxy]);

    let resp = client.get("https://api.ip.sb/ip").send().await?;
    println!("{}", resp.text().await?);

    Ok(())
}
