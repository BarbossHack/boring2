use diesel::{Connection, connection::SimpleConnection};
use wreq::Client;
use wreq_util::Emulation;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // Initialize both libraries to ensure that the prefixed symbols are used
    boring_sys2::init();
    openssl_sys::init();

    // sqlcipher calls openssl functions, so this ensures that the prefixed symbols are used
    let mut conn = diesel::sqlite::SqliteConnection::establish("target/db.sqlite")?;
    conn.batch_execute(r#"PRAGMA key="key""#)?;
    conn.batch_execute(r#"CREATE TABLE IF NOT EXISTS openssl (id INT)"#)?;

    // wreq calls boringssl functions, so this ensures that the prefixed symbols are used
    // and check that wreq is working correctly
    let client = Client::builder().emulation(Emulation::Chrome142).build()?;
    let res = client
        .get("https://tls.browserleaks.com/")
        .send()
        .await?
        .text()
        .await?;
    assert!(
        res.contains("t13d1516h2_8daaf6152771_d8a2da3f94cd"),
        "{}",
        res
    );

    Ok(())
}
