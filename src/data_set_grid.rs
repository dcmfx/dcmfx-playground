use dcmfx::core::*;
use dioxus::prelude::*;

use crate::ui::FontAwesomeIcon;

#[component]
pub fn DataSetGrid(main_data_set: Signal<DataSet>) -> Element {
    rsx! {
        div {
            class: "data-set-grid",

            if !main_data_set().is_empty() {
                div {
                    class: "data-element-value-row header",

                    div { "Tag" }
                    div { "Name" }
                    div { "VR" }
                    div { "Length" }
                    div { "Value" }
                }

                DataSetGridContent { main_data_set, path_to_data_set: DataSetPath::new() }
            }
        }
    }
}

#[component]
fn DataSetGridContent(main_data_set: Signal<DataSet>, path_to_data_set: DataSetPath) -> Element {
    rsx! {
        if let Ok(data_set) = main_data_set().get_data_set_at_path(&path_to_data_set) {
            for (tag, value) in data_set.iter() {
                if let Ok(items) = value.sequence_items() {
                    DataSetSequence {
                        main_data_set,
                        tag: *tag,
                        name: data_set.tag_name(*tag),
                        item_count: items.len(),
                        path_to_sequence: {
                            let mut path = path_to_data_set.clone();
                            path.add_data_element(*tag).unwrap();
                            path
                        }
                    }
                } else if let Ok(_) = value.encapsulated_pixel_data() {
                    DataSetEncapsulatedPixelData {
                        main_data_set,
                        tag: *tag,
                        path_to_encapsulated_pixel_data: {
                            let mut path = path_to_data_set.clone();
                            path.add_data_element(*tag).unwrap();
                            path
                        }
                    }
                } else if let Ok(bytes) = value.bytes() {
                    DataElementValueRow {
                        indent: path_to_data_set.len(),
                        tag: tag.to_string(),
                        name: data_set.tag_name(*tag),
                        vr: value.value_representation().to_string(),
                        length: bytes.len().to_string(),
                        value: value.to_string(*tag, 1000),
                    }
                }
            }
        }
    }
}

#[component]
fn DataSetSequence(
    main_data_set: Signal<DataSet>,
    tag: DataElementTag,
    name: String,
    item_count: usize,
    path_to_sequence: DataSetPath,
) -> Element {
    let mut expanded = use_signal(|| false);

    let indent = path_to_sequence.len() - 1;

    rsx! {
        DataElementValueRow {
            indent,
            expanded: if item_count == 0 { None } else { Some(expanded()) },
            tag: tag.to_string(),
            name,
            vr: "SQ",
            value: format!(
                "{} item{}",
                item_count,
                if item_count == 1 { "" } else { "s" }
            ),
            onclick: move |_| *expanded.write() = !expanded(),
        }

        if expanded() {
            for i in 0..item_count {
                DataSetSequenceItem {
                    main_data_set,
                    item_index: i,
                    path_to_sequence_item: {
                        let mut path = path_to_sequence.clone();
                        path.add_sequence_item(i).unwrap();
                        path
                    }
                }
            }
        }
    }
}

#[component]
fn DataSetSequenceItem(
    main_data_set: Signal<DataSet>,
    item_index: usize,
    path_to_sequence_item: DataSetPath,
) -> Element {
    let mut expanded = use_signal(|| false);

    rsx! {
        DataElementValueRow {
            indent: path_to_sequence_item.len() - 1,
            expanded: Some(expanded()),
            tag: format!("Item {}", (item_index + 1).to_string()),
            onclick: move |_| *expanded.write() = !expanded(),
        }

        if expanded() {
            DataSetGridContent {
                main_data_set,
                path_to_data_set: path_to_sequence_item
            }
        }
    }
}

#[component]
fn DataSetEncapsulatedPixelData(
    main_data_set: Signal<DataSet>,
    tag: DataElementTag,
    path_to_encapsulated_pixel_data: DataSetPath,
) -> Element {
    let mut expanded = use_signal(|| false);

    if let Ok(items) = main_data_set()
        .get_value_at_path(&path_to_encapsulated_pixel_data)
        .unwrap()
        .encapsulated_pixel_data()
    {
        let indent = path_to_encapsulated_pixel_data.len() - 1;

        rsx! {
            DataElementValueRow {
                indent,
                expanded: if items.is_empty() { None } else { Some(expanded()) },
                tag: dictionary::tag_name(tag, None),
                vr: main_data_set()
                    .get_value_at_path(&path_to_encapsulated_pixel_data)
                    .unwrap()
                    .value_representation()
                    .to_string(),
                length: items.len().to_string(),
                onclick: move |_| *expanded.write() = !expanded()
            }

            if expanded() {
                for (i, item) in items.iter().enumerate() {
                    DataElementValueRow {
                        indent: indent + 1,
                        tag: format!("Item {}", i),
                        length: item.len().to_string(),
                    }
                }
            }
        }
    } else {
        rsx! {}
    }
}

#[component]
pub fn DataElementValueRow(
    indent: usize,
    expanded: Option<bool>,
    #[props(into, default)] tag: String,
    #[props(into, default)] name: String,
    #[props(into, default)] vr: String,
    #[props(into, default)] length: String,
    #[props(into, default)] value: String,
    onclick: Option<EventHandler<MouseEvent>>,
) -> Element {
    let is_sequence = vr == "SQ";

    let icon = if expanded == Some(true) {
        "minus"
    } else {
        "plus"
    };

    rsx! {
        div {
            class: "data-element-value-row",
            class: if onclick.is_some() { "interactive" },

            onclick: move |event| {
                if let Some(onclick) = onclick {
                    onclick.call(event);
                }
            },

            div {
                display: "flex",
                align_items: "center",

                div { min_width: format!("{}px", indent * 16) }
                div {
                    margin_right: "0.5em",
                    line_height: "1em",
                    visibility: if expanded.is_some() { "visible" } else { "hidden" },

                    FontAwesomeIcon { icon, style: "solid", size: "xs" }
                }

                {tag.to_string()}
            }
            div { {name} }
            div { {vr} }
            div { {length} }
            div {
                class: "value-cell",
                class: if is_sequence { "sequence" },

                {value}
            }
      }
    }
}
