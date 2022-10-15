#![warn(clippy::all, rust_2018_idioms)]

mod app;
pub use app::MetadazioApp;

fn main() {
    // Make sure panics are logged using `console.error`.
    console_error_panic_hook::set_once();

    // Redirect tracing to console.log and friends:
    tracing_wasm::set_as_global_default();

    let web_options = eframe::WebOptions::default();
    eframe::start_web(
        "metadazio-wasm",
        web_options,
        Box::new(|cc| Box::new(MetadazioApp::new(cc))),
    )
    .expect("failed to start eframe");
}
