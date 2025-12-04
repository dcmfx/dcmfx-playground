use dcmfx::{
    core::{DataSet, IodModule},
    pixel_data::{DataSetPixelDataExtensions, PixelDataRenderer},
};
use dioxus::prelude::*;
use image::RgbImage;
use js_sys::wasm_bindgen::{JsCast, JsValue};
use web_sys::{CanvasRenderingContext2d, HtmlCanvasElement, HtmlElement};

use crate::utils;

#[component]
pub fn PixelDataFrameView(data_set: Signal<DataSet>, frame_index: usize) -> Element {
    let mut container_element = use_signal(|| None);
    let mut canvas_element = use_signal(|| None);
    let mut error_message = use_signal(|| None);

    let mut redraw = move || {
        error_message.set(None);

        let Some(container) = container_element() else {
            return;
        };

        let Some(canvas) = canvas_element() else {
            return;
        };

        let Ok(mut frames) = data_set().get_pixel_data_frames() else {
            error_message.set(Some("No pixel data found".into()));
            return;
        };

        let Some(frame) = frames.get_mut(frame_index) else {
            error_message.set(Some(format!(
                "Pixel data frame index '{}' is out of range",
                frame_index
            )));
            return;
        };

        let Ok(pixel_data_renderer) = PixelDataRenderer::from_data_set(&data_set()) else {
            error_message.set(Some("Pixel data renderer creation failed".into()));
            return;
        };

        match pixel_data_renderer.render_frame(frame, None) {
            Ok(image) => draw_rgb_image_to_canvas(&image, canvas, container).unwrap(),

            Err(e) => {
                utils::canvas::clear(&canvas).unwrap();

                error_message.set(Some(format!("Frame rendering failed. {}", e)));
            }
        }
    };

    use_effect(redraw);

    rsx! {
        div {
            class: "frame-view",

            onmounted: move |ev| container_element.set(utils::get_element::<HtmlElement>(ev)),
            onresize: move |_| redraw(),

            canvas {
                onmounted: move |ev| canvas_element.set(utils::get_element::<HtmlCanvasElement>(ev)),
            }

            if let Some(error_message) = error_message() {
                div {
                    class: "frame-view-error",

                    b { "Error" }
                    br {}
                    br {}
                    "{error_message}"
                }
            }
        }
    }
}

fn draw_rgb_image_to_canvas(
    rgb_image: &RgbImage,
    dst_canvas: web_sys::HtmlCanvasElement,
    canvas_container: web_sys::HtmlElement,
) -> Result<(), JsValue> {
    // Set width and height of the destination canvas
    let rect = canvas_container.get_bounding_client_rect();
    dst_canvas.set_width(rect.width() as u32);
    dst_canvas.set_height(rect.height() as u32);

    // Get 2D rendering context for the destination canvas
    let dst_context = dst_canvas
        .get_context("2d")?
        .unwrap()
        .dyn_into::<CanvasRenderingContext2d>()?;

    // Convert source RGB image data into a canvas
    let src_canvas = utils::canvas::from_rgb_image(rgb_image)?;

    // Compute aspect ratios to determine whether vertical or horizontal padding is needed
    let image_aspect_ratio = rgb_image.width() as f64 / rgb_image.height() as f64;
    let canvas_aspect_ratio = dst_canvas.width() as f64 / dst_canvas.height() as f64;

    // Draw the image into the canvas, scaling so that it fits exactly
    if image_aspect_ratio > canvas_aspect_ratio {
        let image_height = dst_canvas.width() as f64 / image_aspect_ratio;

        dst_context.draw_image_with_html_canvas_element_and_dw_and_dh(
            &src_canvas,
            0.0,
            (dst_canvas.height() as f64 - image_height) * 0.5,
            dst_canvas.width() as f64,
            image_height,
        )
    } else {
        let image_width = dst_canvas.height() as f64 * image_aspect_ratio;

        dst_context.draw_image_with_html_canvas_element_and_dw_and_dh(
            &src_canvas,
            (dst_canvas.width() as f64 - image_width) * 0.5,
            0.0,
            image_width,
            dst_canvas.height() as f64,
        )
    }
}
