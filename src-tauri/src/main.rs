#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

mod circle_plot;
pub mod sequences;

use tauri::{WindowBuilder, TitleBarStyle};
use url::Url;

#[tauri::command]
fn save_plot(plot: circle_plot::Plot) {
    let filename = format!("circle-plot {}.svg", &plot.modulus);
    let doc = plot.render();

    svg::save(filename, &doc).unwrap();
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![save_plot])
        .setup(|app| {
            WindowBuilder::new(app, "label", tauri::WindowUrl::App("index.html".into()))
                .inner_size(1200.0, 800.0)
                .visible(false)
                .title("")
                .hidden_title(true)
                .title_bar_style(TitleBarStyle::Overlay)
                .build()?;

            Ok(())
        })
        .register_uri_scheme_protocol("circleplot", |_app, req| {
            let url: Url = req.uri().parse().unwrap();

            let doc = circle_plot::Plot::try_from(url)?.render();

            let mut buf = Vec::new();
            svg::write(&mut buf, &doc).unwrap();

            tauri::http::ResponseBuilder::new()
                .header("Origin", "*")
                .mimetype("image/svg+xml")
                .header("Content-Length", buf.len())
                .status(200)
                .body(buf)
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
