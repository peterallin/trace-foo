use tracing::{instrument, span, Level, event};
use tracing_subscriber::fmt::format;
use std::error::Error;
use std::io;

fn main() -> Result<(), Box<dyn Error>> {
    let formatter = format::debug_fn(|writer, field, value| {
        write!(writer, ">>> {}: {:?}", field, value)
    });

    let subscriber = tracing_subscriber::fmt()
        .with_level(true)
        .with_writer(io::stderr)
        .fmt_fields(formatter)
        .finish();
    tracing::subscriber::set_global_default(subscriber)?;

    event!(Level::ERROR, "My first event");
    foo();
    bar();
    Ok(())
}

#[instrument]
fn foo() {
    event!(Level::ERROR, "A foo event");
    bar();
}

#[instrument]
fn bar() {
    event!(Level::ERROR, "A bar event");
}