use image::RgbImage;
use js_sys::wasm_bindgen::{JsCast, JsValue};
use wasm_bindgen::Clamped;
use web_sys::{CanvasRenderingContext2d, HtmlCanvasElement, ImageData};

/// Clears the content of the specified canvas.
///
pub fn clear(canvas: &HtmlCanvasElement) -> Result<(), JsValue> {
    let context = canvas
        .get_context("2d")?
        .unwrap()
        .dyn_into::<CanvasRenderingContext2d>()?;

    context.clear_rect(0.0, 0.0, canvas.width() as f64, canvas.height() as f64);

    Ok(())
}

/// Creates an HTML canvas element containing the specified RGB image content.
///
pub fn from_rgb_image(rgb_image: &RgbImage) -> Result<HtmlCanvasElement, JsValue> {
    let document = super::document();

    // Create a new canvas
    let canvas = document
        .create_element("canvas")?
        .dyn_into::<HtmlCanvasElement>()?;
    canvas.set_width(rgb_image.width());
    canvas.set_height(rgb_image.height());

    // Get 2D context
    let context = canvas
        .get_context("2d")?
        .unwrap()
        .dyn_into::<CanvasRenderingContext2d>()?;

    // Convert image data to RGBA
    let mut rgba_data = Vec::with_capacity(rgb_image.as_raw().len() / 3 * 4);
    for chunk in rgb_image.as_raw().chunks_exact(3) {
        rgba_data.extend_from_slice(chunk);
        rgba_data.push(0xFF);
    }

    // Create ImageData object containing the image
    let image_data = ImageData::new_with_u8_clamped_array_and_sh(
        Clamped(&rgba_data),
        rgb_image.width(),
        rgb_image.height(),
    )?;

    // Put the image data into the canvas
    context.put_image_data(&image_data, 0.0, 0.0)?;

    Ok(canvas)
}
