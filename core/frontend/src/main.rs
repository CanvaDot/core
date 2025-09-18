#![deny(clippy::pedantic)]
#![deny(clippy::unwrap_used)]
#![deny(clippy::match_like_matches_macro)]

#[cfg(not(feature = "coverage"))]
use ::{
    tracing::Level as TracingLevel,
    tracing_subscriber::filter::Targets,
    tracing_subscriber::fmt::layer as ts_layer,
    tracing_subscriber::{prelude::*, registry as ts_registry},
    tracing_web::MakeWebConsoleWriter,
    yew::Renderer,
};

#[cfg(not(feature = "coverage"))]
use crate::app::App;

#[cfg(not(feature = "coverage"))]
mod app;
#[cfg(not(feature = "coverage"))]
mod components;
mod utils;

#[cfg(not(feature = "coverage"))]
fn main() {
    let fmt_layer = ts_layer()
        .with_ansi(false)
        .without_time()
        .with_writer(MakeWebConsoleWriter::new())
        .with_filter(
            Targets::new()
                .with_target("yew", TracingLevel::DEBUG)
                .with_default(TracingLevel::TRACE),
        );

    ts_registry()
        .with(fmt_layer)
        .init();

    Renderer::<App>::new().render();
}

#[cfg(feature = "coverage")]
fn main() {}

#[cfg(feature = "coverage")]
#[cfg(test)]
mod tests {
    use crate::main;

    #[test]
    fn main_coverage() {
        main()
    }
}
