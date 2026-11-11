//! web_bridge.rs — Cross-platform browser/WASM integration helpers for timestamps and file downloads.

/// Returns a human-readable UNIX epoch timestamp string without panicking on `wasm32`.
pub fn unix_timestamp_label() -> String {
    #[cfg(not(target_arch = "wasm32"))]
    {
        std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .map(|d| format!("Epoch +{}s", d.as_secs()))
            .unwrap_or_else(|_| "Live".to_string())
    }

    #[cfg(target_arch = "wasm32")]
    {
        let secs = (js_sys::Date::now() / 1000.0).max(0.0) as u64;
        format!("Epoch +{}s", secs)
    }
}

/// Triggers an immediate browser file download when running on WebAssembly (`wasm32`).
#[cfg(target_arch = "wasm32")]
pub fn trigger_browser_download(
    filename: &str,
    bytes: &[u8],
    mime_type: &str,
) -> Result<String, String> {
    use wasm_bindgen::JsCast as _;

    let window = web_sys::window().ok_or_else(|| "Browser window unavailable".to_string())?;
    let document = window
        .document()
        .ok_or_else(|| "Browser document unavailable".to_string())?;

    let uint8_array = js_sys::Uint8Array::from(bytes);
    let array = js_sys::Array::new();
    array.push(&uint8_array.buffer());

    let props = web_sys::BlobPropertyBag::new();
    props.set_type(mime_type);

    let blob = web_sys::Blob::new_with_buffer_source_sequence_and_options(&array, &props)
        .map_err(|e| format!("Failed to create download blob: {e:?}"))?;

    let url = web_sys::Url::create_object_url_with_blob(&blob)
        .map_err(|e| format!("Failed to create object URL: {e:?}"))?;

    let anchor = document
        .create_element("a")
        .map_err(|e| format!("Failed to create anchor element: {e:?}"))?
        .dyn_into::<web_sys::HtmlAnchorElement>()
        .map_err(|_| "Created element is not an HtmlAnchorElement".to_string())?;

    anchor.set_href(&url);
    anchor.set_download(filename);
    anchor.set_attribute("style", "display:none").ok();

    if let Some(body) = document.body() {
        let _ = body.append_child(&anchor);
        anchor.click();
        let _ = body.remove_child(&anchor);
    } else {
        anchor.click();
    }

    let _ = web_sys::Url::revoke_object_url(&url);
    Ok(format!("Browser Download ({filename})"))
}
