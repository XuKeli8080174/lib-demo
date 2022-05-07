use std::error::Error;

use tracing_subscriber::{prelude::__tracing_subscriber_SubscriberExt, fmt, util::SubscriberInitExt};
use tracing::{span, Level, event, instrument, Instrument};

#[test]
fn trace_log() -> Result<(), Box<dyn Error + Send + Sync + 'static>> {

    tracing_subscriber::registry()
        .with(fmt::layer())
        .init();

    log::info!("Hello world");

    let foo = 42;
    tracing::info!(foo, "Hello from tracing");

    // 在 span 的上下文之外记录一次 event 事件
    event!(Level::INFO, "something happened");
    let span = span!(Level::TRACE, "my_span");
    let _enter = span.enter();
    // 在 "my_span" 的上下文中记录一次 event
    event!(Level::DEBUG, "something happened inside my_span");
    Ok(())
}

// 将整个函数体作为一个tracing的span, 这样可以将在异步函数中不同时间点所发生的时间记录在同一个span中
#[instrument]
async fn async_in_span() {
    // 另一种对future实现绑定instrument
    let _f = async move {
        
    }.instrument(tracing::info_span!("my_future"));

}

#[test]
fn json_macro() {
    let json = serde_json::json!({
        "foo": 1,
        "bar": 2,
    });
    println!("{:?}", json)
}
