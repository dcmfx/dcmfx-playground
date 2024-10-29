#![allow(non_snake_case)]

use data_set_grid::*;
use dcmfx::{core::*, json::*, p10::*};
use dioxus::prelude::*;
use dioxus_elements::{FileEngine, HasFileData};
use dioxus_logger::tracing::Level;
use std::sync::Arc;

mod data_set_grid;
mod ui;
mod utils;

use crate::ui::FontAwesomeIcon;

fn main() {
    dioxus_logger::init(Level::INFO).expect("Logger init failed");

    launch(App);
}

#[derive(Clone)]
enum DataSetSourceType {
    P10,
    Json,
}

#[component]
fn App() -> Element {
    let mut dicom_filename = use_signal(String::new);
    let mut data_set = use_signal(DataSet::new);
    let mut data_set_source_type = use_signal(|| DataSetSourceType::P10);
    let mut error_lines = use_signal::<Vec<String>>(Vec::new);
    let mut is_file_dragged_over = use_signal(|| false);

    let mut clear_dicom = move || {
        dicom_filename.set("".to_string());
        data_set.set(DataSet::new());
        error_lines.set(vec![]);
    };

    let mut on_select_input_file = move |files: Option<Arc<dyn FileEngine>>| {
        dicom_filename.set("".to_string());
        error_lines.set(vec![]);

        spawn(async move {
            if let Some(file_engine) = files {
                if let Some(filename) = file_engine.files().first() {
                    if let Some(bytes) = file_engine.read_file(filename).await {
                        dicom_filename.set(filename.clone());

                        // If the file has a .json extension then load it as a DICOM JSON file,
                        // otherwise load it as a DICOM P10
                        if dicom_filename().to_lowercase().ends_with(".json") {
                            if let Ok(json) = std::str::from_utf8(&bytes) {
                                match DataSet::from_json(json) {
                                    Ok(ds) => {
                                        data_set.set(ds);
                                        data_set_source_type.set(DataSetSourceType::Json);
                                    }
                                    Err(e) => error_lines.set(e.to_lines("")),
                                };
                            }
                        } else {
                            match dcmfx::p10::read_bytes(bytes) {
                                Ok(ds) => {
                                    data_set.set(ds);
                                    data_set_source_type.set(DataSetSourceType::P10);
                                }
                                Err((e, mut data_set_builder)) => {
                                    data_set_builder.force_end();
                                    data_set.set(data_set_builder.final_data_set().unwrap());
                                    error_lines.set(e.to_lines("reading file"));
                                }
                            };
                        }
                    }
                }
            }
        });
    };

    let on_download_p10 = move |_: MouseEvent| {
        let filename = match data_set_source_type() {
            DataSetSourceType::P10 => dicom_filename(),
            DataSetSourceType::Json => {
                format!("{}.dcm", &dicom_filename()[0..dicom_filename().len() - 5])
            }
        };

        let mut writer = utils::BlobPartWriter::new(1024 * 1024);

        match data_set().write_p10_stream(&mut writer, None) {
            Ok(()) => {
                utils::trigger_download(writer.into_js_array(), &filename, "application/json")
                    .unwrap();

                ui::toasts::add_info("Generated DICOM P10 file for download".into());
            }

            Err(e) => {
                ui::toasts::add_info(e.to_lines("writing P10 file").join(", "));
            }
        }
    };

    let on_download_json = move |_: MouseEvent| {
        let filename = match data_set_source_type() {
            DataSetSourceType::P10 => format!("{}.json", dicom_filename()),
            DataSetSourceType::Json => dicom_filename(),
        };

        let mut writer = utils::BlobPartWriter::new(1024 * 1024);

        let config = DicomJsonConfig {
            store_encapsulated_pixel_data: true,
        };

        match data_set().to_json_stream(config, &mut writer) {
            Ok(()) => {
                utils::trigger_download(writer.into_js_array(), &filename, "application/dicom")
                    .unwrap();

                ui::toasts::add_info("Generated DICOM JSON file for download".into());
            }
            Err(e) => ui::toasts::add_error(e.to_string()),
        }
    };

    rsx! {
        link { rel: "stylesheet", href: "main.css" }
        script {
            src: "https://kit.fontawesome.com/4e8968f74f.js",
            crossorigin: "anonymous"
        }

        section {
            class: "main",

            ondragover: move |event| {
                event.prevent_default();
                event.stop_propagation();
                is_file_dragged_over.set(true);
            },
            ondragleave: move |_| is_file_dragged_over.set(false),
            ondrop: move |event| {
                event.prevent_default();
                on_select_input_file(event.files());
                is_file_dragged_over.set(false);
            },

            div {
                class: "app-header",

                img { src: "logo.svg", height: "64px" }
                h1 { "DCMfx Playground" }
                a {
                    href: "https://github.com/dcmfx/dcmfx-playground",
                    target: "_blank",
                    margin_left: "auto",

                    FontAwesomeIcon { icon: "github", style: "brands", size: "2x" }
                }
            }

            div {
                class: "name-divider",

                div { class: "divider-line" }
                div {
                    class: "file-details",

                    div {
                        class: "details-text",

                        if dicom_filename().is_empty() {
                            "No DICOM selected"
                        } else {
                            {dicom_filename}
                        }
                    }

                    if !dicom_filename().is_empty() {
                        div { class: "vertical-divider" }
                        div {
                            class: "close-icon",
                            onclick: move |_| clear_dicom(),

                            FontAwesomeIcon { icon: "close", style: "solid", size: "lg" }
                        }
                    }
                }
                div { class: "divider-line" }
            }

            if !error_lines().is_empty() {
                div {
                    class: "error-container",

                    for line in error_lines() {
                        if line.is_empty() {
                            br {}
                        } else {
                            pre { {line} }
                        }
                    }
                }
            }

            if data_set().is_empty() && error_lines().is_empty() {
                DropArea { is_file_dragged_over, on_select_input_file }
            } else {
                DataSetGrid { main_data_set: data_set }
            }

            div {
                class: "bottom-toolbar",
                class: if data_set().size() == 0 { "disabled" },

                button { onclick: on_download_p10, "Download as .dcm" }
                button { onclick: on_download_json, "Download as .json" }
            }

            ui::ToastUi {}
        }
    }
}

#[component]
fn DropArea(
    is_file_dragged_over: Signal<bool>,
    on_select_input_file: EventHandler<Option<Arc<dyn FileEngine>>>,
) -> Element {
    rsx! {
        div {
            class: "drop-area-container",

            div {
                class: "drop-area",
                class: if is_file_dragged_over() { "active-drag-over" },

                i { class: "fa-solid fa-file-medical fa-3x" }

                span { "Drop a DICOM or DICOM JSON file here, or click to browse for a file." }
                span {
                    font_size: "0.75em",
                    color: "#AAA",
                    margin: "0 2em",

                    "Opened DICOM files never leave your device."
                }

                input {
                    id: "dicom-file-input",
                    r#type: "file",
                    onchange: move |event| on_select_input_file(event.files()),
                }
            }
        }
    }
}
