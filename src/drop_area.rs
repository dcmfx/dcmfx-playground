use dioxus::prelude::*;
use dioxus_elements::FileData;

#[component]
pub fn DropArea(
    is_file_dragged_over: Signal<bool>,
    on_select_input_file: EventHandler<Option<FileData>>,
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
                    onchange: move |event| on_select_input_file(event.files().into_iter().next()),
                }
            }
        }
    }
}
