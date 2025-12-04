#![allow(non_snake_case)]

use dcmfx::{core::*, json::*, p10::*};
use dioxus::{document::Title, prelude::*};
use dioxus_elements::{FileData, HasFileData};

mod data_set_grid;
mod drop_area;
mod pixel_data_frame_view;
mod ui;
mod utils;

use data_set_grid::*;
use drop_area::*;
use pixel_data_frame_view::*;

const LOGO_SVG: Asset = asset!("/assets/logo.svg");
const MAIN_CSS: Asset = asset!("/assets/main.scss");

fn main() {
    launch(App);
}

#[derive(Clone)]
enum DataSetSourceType {
    P10,
    Json,
}

#[derive(Clone, PartialEq)]
enum ViewMode {
    DataSet,
    PixelData,
}

#[component]
fn App() -> Element {
    let mut dicom_filename = use_signal(String::new);
    let mut data_set = use_signal(DataSet::new);
    let mut data_set_source_type = use_signal(|| DataSetSourceType::P10);
    let mut error_lines = use_signal::<Vec<String>>(Vec::new);
    let mut is_file_dragged_over = use_signal(|| false);

    let mut view_mode = use_signal(|| ViewMode::DataSet);

    let mut clear_dicom = move || {
        dicom_filename.set("".to_string());
        data_set.set(DataSet::new());
        error_lines.set(vec![]);
    };

    let mut on_select_input_file = move |file_data: Option<FileData>| {
        dicom_filename.set("".to_string());
        error_lines.set(vec![]);

        spawn(async move {
            let Some(file_data) = file_data else {
                return;
            };

            let Ok(bytes) = file_data.read_bytes().await else {
                return;
            };

            dicom_filename.set(file_data.name());

            // If the file has a .json extension then load it as a DICOM JSON file, otherwise load
            // it as a DICOM P10
            if dicom_filename().to_lowercase().ends_with(".json") {
                let Ok(json) = std::str::from_utf8(&bytes) else {
                    return;
                };

                match DataSet::from_json(json) {
                    Ok(ds) => {
                        data_set.set(ds);
                        data_set_source_type.set(DataSetSourceType::Json);
                    }
                    Err(e) => error_lines.set(e.to_lines("")),
                };
            } else {
                match dcmfx::p10::read_bytes(bytes.to_vec().into()) {
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
        });
    };

    let on_download_p10 = move |_: MouseEvent| {
        let filename = match data_set_source_type() {
            DataSetSourceType::P10 => dicom_filename(),
            DataSetSourceType::Json => {
                format!("{}.dcm", &dicom_filename()[0..dicom_filename().len() - 5])
            }
        };

        let mut writer = utils::download::BlobPartWriter::new(1024 * 1024);

        match data_set().write_p10_stream(&mut writer, None) {
            Ok(()) => {
                utils::download::trigger(writer.into_js_array(), &filename, "application/json")
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

        let mut writer = utils::download::BlobPartWriter::new(1024 * 1024);

        let config = DicomJsonConfig {
            store_encapsulated_pixel_data: true,
            pretty_print: true,
        };

        match data_set().to_json_stream(config, &mut writer) {
            Ok(()) => {
                utils::download::trigger(writer.into_js_array(), &filename, "application/dicom")
                    .unwrap();

                ui::toasts::add_info("Generated DICOM JSON file for download".into());
            }
            Err(e) => ui::toasts::add_error(e.to_string()),
        }
    };

    rsx! {
        document::Stylesheet { href: MAIN_CSS }

        Title { "DCMfx Playground" }

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
                on_select_input_file(event.files().into_iter().next());
                is_file_dragged_over.set(false);
            },

            div {
                class: "app-header",

                img { src: LOGO_SVG, height: "48px" }
                h1 {
                    "DCMfx Playground"

                    if !dicom_filename().is_empty() {
                        " — "
                        code { i { "\"{dicom_filename}\"" } }
                    }
                }
                a {
                    href: "https://github.com/dcmfx/dcmfx-playground",
                    target: "_blank",
                    margin_left: "auto",

                    ui::FontAwesomeIcon { icon: "github", style: "brands", size: "2x" }
                }
            }

            div {
                class: "name-divider",

                div { class: "divider-line" }
                if !dicom_filename().is_empty() {
                    div {
                        class: "file-details",

                        div {
                            class: "details-text",
                            class: if view_mode() == ViewMode::DataSet { "selected" },

                            onclick: move |_| view_mode.set(ViewMode::DataSet),
                            "Data set"
                        }
                        div { class: "vertical-divider" }
                        div {
                            class: "details-text",
                            class: if view_mode() == ViewMode::PixelData { "selected" },

                            onclick: move |_| view_mode.set(ViewMode::PixelData),
                            "Pixel data"
                        }
                        div { class: "vertical-divider" }
                        div {
                            class: "close-icon",
                            onclick: move |_| clear_dicom(),

                            ui::FontAwesomeIcon { icon: "close", style: "solid", size: "lg" }
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
                if view_mode() == ViewMode::DataSet {
                    DataSetGrid { main_data_set: data_set }
                } else {
                    PixelDataFrameView { data_set, frame_index: 0 }
                }
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
