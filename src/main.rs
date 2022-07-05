use anyhow::Context;
use axum::Router;
use tracing::info;
use tracing_appender::{non_blocking, rolling};
use tracing_subscriber::{
    filter::EnvFilter, fmt, layer::SubscriberExt, util::SubscriberInitExt, Registry,
};

pub mod models;
pub mod services;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let env_filter = EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new("info"));
    // 输出到控制台中
    let formatting_layer = fmt::layer().pretty().with_writer(std::io::stderr);

    // 输出到文件中
    let file_appender = rolling::hourly("logs", "app.log.ndjson");
    let (non_blocking_appender, _guard) = non_blocking(file_appender);
    let file_layer = fmt::layer()
        .with_ansi(false)
        .with_writer(non_blocking_appender)
        .json();

    // 注册
    Registry::default()
        .with(env_filter)
        // ErrorLayer 可以让 color-eyre 获取到 span 的信息
        // .with(ErrorLayer::default())
        .with(formatting_layer)
        .with(file_layer)
        .init();
    color_eyre::install().map_err(|x| anyhow::anyhow!("{}", x))?;
    let app = Router::new().merge(services::router());
    let addr_str = "127.0.0.1:8888";
    let addr = &addr_str
        .parse()
        .with_context(|| format!("invalid IP address syntax: {}", addr_str))?;
    info!(message = "start serve",%addr);
    axum::Server::bind(addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
    Ok(())
}
