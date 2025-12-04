use dioxus::prelude::*;
use dioxus::web::WebEventExt;
use js_sys::wasm_bindgen::JsCast;

pub mod canvas;
pub mod download;

/// Returns the global document object.
///
pub fn document() -> web_sys::Document {
    web_sys::window().unwrap().document().unwrap()
}

/// Returns the underlying web event contained in an `onmounted`` event. This helper is used when
/// storing HTML elements into signals.
///
pub fn get_element<T>(ev: Event<MountedData>) -> Option<T>
where
    T: JsCast,
{
    ev.as_web_event().dyn_into::<T>().ok()
}
