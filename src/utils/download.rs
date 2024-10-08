use js_sys::wasm_bindgen::{JsCast, JsValue};

/// Triggers a browser download of a file with the contents specified by the given blob parts, which
/// must be an array of [`js_sys::Uint8Array`]s.
///
pub fn trigger_download(
    blob_parts: js_sys::Array,
    filename: &str,
    mime_type: &str,
) -> Result<(), JsValue> {
    // Create property bag specifying the MIME type
    let options = web_sys::BlobPropertyBag::new();
    options.set_type(mime_type);

    // Create a blob from the provided parts and convert to an object URL
    let blob = web_sys::Blob::new_with_u8_array_sequence_and_options(&blob_parts, &options)?;
    let url = web_sys::Url::create_object_url_with_blob(&blob)?;

    // Ensure object URL is cleaned up
    scopeguard::defer! {
      let _ = web_sys::Url::revoke_object_url(&url);
    }

    // Create anchor tag for triggering the download
    let document = web_sys::window().unwrap().document().unwrap();
    let a = document
        .create_element("a")?
        .dyn_into::<web_sys::HtmlElement>()?;

    a.set_attribute("href", &url)?;
    a.set_attribute("download", filename)?;
    document.body().unwrap().append_child(&a)?;

    // Ensure anchor tag is cleaned up
    scopeguard::defer! {
      let _ = document.body().unwrap().remove_child(&a);
    }

    // Click the anchor tag to trigger the download
    a.click();

    Ok(())
}

/// Takes written bytes and chunks them into [`js_sys::Uint8Array`]s of the specified size. These
/// chunks can then be turned into a Blob and offered for download. This approach reduces memory
/// pressure in WASM by shifting the allocations to JavaScript.
///
pub struct BlobPartWriter {
    part_size: usize,
    buffer: Vec<u8>,
    js_array: js_sys::Array,
}

impl BlobPartWriter {
    /// Creates a new blob part writer which accumulates written bytes in [`js_sys::Uint8Array`]s of
    /// the given size.
    ///
    pub fn new(part_size: usize) -> Self {
        Self {
            part_size,
            buffer: Vec::with_capacity(part_size),
            js_array: js_sys::Array::new(),
        }
    }

    /// Consumes this writer and returns its internal JavaScript array.
    ///
    pub fn into_js_array(self) -> js_sys::Array {
        self.js_array
    }

    fn flush_buffer(&mut self) {
        let uint8_array = js_sys::Uint8Array::new_with_length(self.buffer.len() as u32);
        uint8_array.copy_from(&self.buffer);
        self.js_array.push(&uint8_array);

        self.buffer.clear();
    }
}

impl std::io::Write for BlobPartWriter {
    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        let mut remaining = buf;

        while !remaining.is_empty() {
            let space_left = self.part_size - self.buffer.len();
            let to_write = remaining.len().min(space_left);

            self.buffer.extend_from_slice(&remaining[..to_write]);
            remaining = &remaining[to_write..];

            if self.buffer.len() == self.part_size {
                self.flush_buffer();
            }
        }

        Ok(buf.len())
    }

    fn flush(&mut self) -> std::io::Result<()> {
        self.flush_buffer();
        Ok(())
    }
}
