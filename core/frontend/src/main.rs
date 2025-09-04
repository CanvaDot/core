#![deny(clippy::pedantic)]
#![deny(clippy::unwrap_used)]
#![deny(clippy::match_like_matches_macro)]

use app::App;
use tracing_subscriber::filter::Targets;
use tracing_subscriber::prelude::*;
use tracing_web::MakeWebConsoleWriter;

mod app;

#[cfg(test)]
mod tests;

fn main() {
    // Configure tracing to output to browser console
    let fmt_layer = tracing_subscriber::fmt::layer()
        .with_ansi(true) // This might not work in all browsers
        .without_time() // std::time is not available in browsers
        .with_writer(MakeWebConsoleWriter::new())
        .with_filter(
            Targets::new()
                .with_target("yew", tracing::Level::DEBUG) // yew trace can be verbose
                .with_default(tracing::Level::TRACE),
        );

    tracing_subscriber::registry()
        .with(fmt_layer)
        .init();

    tracing::info!("Starting Yew application");
    yew::Renderer::<App>::new().render();
}
