use std::sync::atomic::{AtomicUsize, Ordering};
use std::time::Duration;

use dioxus::prelude::*;
use indexmap::IndexMap;

static TOASTS: GlobalSignal<IndexMap<usize, Toast>> = Signal::global(IndexMap::new);

static NEXT_TOAST_ID: AtomicUsize = AtomicUsize::new(0);

/// Adds a new info toast.
///
pub fn add_info(message: String) {
    add(ToastType::Info, message, Duration::from_secs(2));
}

/// Adds a new error toast.
///
pub fn add_error(message: String) {
    add(ToastType::Error, message, Duration::from_secs(10));
}

fn add(r#type: ToastType, message: String, duration: Duration) {
    // Allocate unique ID for the toast
    let id = NEXT_TOAST_ID.fetch_add(1, Ordering::Relaxed);

    // Add the new toast
    TOASTS.write().insert(id, Toast { r#type, message });

    // Remove the toast after the specified duration
    dioxus_core::spawn_forever(async move {
        gloo_timers::future::sleep(duration).await;
        TOASTS.write().shift_remove(&id);
    });
}

#[derive(Clone)]
struct Toast {
    r#type: ToastType,
    message: String,
}

#[derive(Clone)]
enum ToastType {
    Info,
    Error,
}

impl ToastType {
    fn css_class(&self) -> &'static str {
        match self {
            Self::Info => "info",
            Self::Error => "error",
        }
    }
}

#[component]
pub fn ToastUi() -> Element {
    rsx! {
        div {
            class: "toast-list",

            for (_id, toast) in TOASTS.read().iter() {
                div {
                    class: "toast",
                    class: "{toast.r#type.css_class()}",

                    {toast.message.clone()}
                }
            }
        }
    }
}
