use dioxus::prelude::*;

#[derive(PartialEq, Props, Clone)]
pub struct FontAwesomeIconProps {
    /// The name of the Font Awesome icon to show. See https://fontawesome.com/icons for a full list
    /// of available icons.
    ///
    #[props(into)]
    icon: String,

    /// The style of icon. Must be either `regular` or `solid`.
    ///
    /// Default: `regular`.
    ///
    #[props(into, default = "regular".to_string())]
    style: String,

    /// The size of the icon. Must be one of `xs`, `sm`, `lg`, `2x`, `3x`, `4x`, `5x`, `6x`, `7x`,
    /// `8x`, `9x`, or `10x`.
    ///
    #[props(into)]
    size: Option<String>,
}

/// Displays a Font Awesome icon as an `<i>` tag.
///
#[component]
pub fn FontAwesomeIcon(props: FontAwesomeIconProps) -> Element {
    rsx! {
        i {
            class: "fa-{props.icon} fa-{props.style}",
            class: if let Some(ref size) = props.size { "fa-{size}" },
        }
    }
}
