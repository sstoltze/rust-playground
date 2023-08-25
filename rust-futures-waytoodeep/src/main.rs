// mod dumb;

use color_eyre::Report;
use reqwest::Client;
use tracing::info;
use tracing_subscriber::EnvFilter;

pub const URL_1: &str = "https://fasterthanli.me/articles/whats-in-the-box";
pub const URL_2: &str = "https://fasterthanli.me/series/advent-of-code-2020/part-13";

#[tokio::main]
async fn main() -> Result<(), Report> {
    setup()?;

    info!("This does nothing.");

    let client = Client::new();

    // info!("Building dumb future");
    // let dumb = dumb::DumbFuture {};
    // info!("Awaiting dumb future");
    // dumb.await;
    // info!("Done awaiting");

    fetch_thing(&client, URL_1).await?;
    fetch_thing(&client, URL_2).await?;

    Ok(())
}

async fn fetch_thing(client: &Client, url: &str) -> Result<(), Report> {
    let res = client.get(url).send().await?.error_for_status()?;
    info!(%url, content_type=?res.headers().get("content-type"), "Got a response");
    Ok(())
}

fn setup() -> Result<(), Report> {
    if std::env::var("RUST_BACKTRACE").is_err() {
        std::env::set_var("RUST_BACKTRACE", "1")
    }
    color_eyre::install()?;

    if std::env::var("RUST_LOG").is_err() {
        std::env::set_var("RUST_LOG", "info")
    }
    tracing_subscriber::fmt::fmt()
        .with_env_filter(EnvFilter::from_default_env())
        .init();

    Ok(())
}
