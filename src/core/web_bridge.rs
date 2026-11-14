//! web_bridge.rs — Cross-platform browser/WASM integration helpers for timestamps, downloads, and persistent Web Vault I/O.

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct WebVaultProjectMeta {
    #[serde(default)]
    pub name: String,
    #[serde(default)]
    pub filename: String,
    #[serde(default)]
    pub rel_path: String,
    #[serde(default)]
    pub category: String,
    #[serde(default)]
    pub node_count: usize,
    #[serde(default)]
    pub member_count: usize,
    #[serde(default)]
    pub size_bytes: usize,
    #[serde(default)]
    pub updated_at: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct WebVaultStateSnapshot {
    #[serde(default)]
    pub connected: bool,
    #[serde(default)]
    pub guest_mode: bool,
    #[serde(default)]
    pub vault_name: String,
    #[serde(default)]
    pub backend: String,
    #[serde(default)]
    pub fs_supported: bool,
    #[serde(default)]
    pub needs_permission: bool,
    #[serde(default)]
    pub busy: bool,
    #[serde(default)]
    pub status_msg: String,
    #[serde(default)]
    pub error_msg: String,
    #[serde(default)]
    pub version: u64,
    #[serde(default)]
    pub projects: Vec<WebVaultProjectMeta>,
}

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

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ImportedFilePayload {
    pub filename: String,
    pub content: String,
}

#[cfg(not(target_arch = "wasm32"))]
static PENDING_NATIVE_IMPORTED_FILE: std::sync::Mutex<Option<ImportedFilePayload>> =
    std::sync::Mutex::new(None);

#[cfg(target_arch = "wasm32")]
mod js_vault {
    use wasm_bindgen::prelude::*;

    #[wasm_bindgen]
    extern "C" {
        #[wasm_bindgen(js_namespace = EigenGraphVaultBridge, js_name = getVersion)]
        pub fn get_version() -> u32;

        #[wasm_bindgen(js_namespace = EigenGraphVaultBridge, js_name = getStateJson)]
        pub fn get_state_json() -> String;

        #[wasm_bindgen(js_namespace = EigenGraphVaultBridge, js_name = connectExistingVault)]
        pub fn connect_existing_vault();

        #[wasm_bindgen(js_namespace = EigenGraphVaultBridge, js_name = connectIndexedDbVault)]
        pub fn connect_indexeddb_vault(name: &str);

        #[wasm_bindgen(js_namespace = EigenGraphVaultBridge, js_name = createNewVault)]
        pub fn create_new_vault(name: &str, force_indexeddb: bool);

        #[wasm_bindgen(js_namespace = EigenGraphVaultBridge, js_name = reconnectVault)]
        pub fn reconnect_vault();

        #[wasm_bindgen(js_namespace = EigenGraphVaultBridge, js_name = refreshVault)]
        pub fn refresh_vault();

        #[wasm_bindgen(js_namespace = EigenGraphVaultBridge, js_name = disconnectVault)]
        pub fn disconnect_vault();

        #[wasm_bindgen(js_namespace = EigenGraphVaultBridge, js_name = enterGuestMode)]
        pub fn enter_guest_mode();

        #[wasm_bindgen(js_namespace = EigenGraphVaultBridge, js_name = readFileSync)]
        pub fn read_file_sync(rel_path: &str) -> String;

        #[wasm_bindgen(js_namespace = EigenGraphVaultBridge, js_name = writeFileAsync)]
        pub fn write_file_async(rel_path: &str, content: &str);

        #[wasm_bindgen(js_namespace = EigenGraphVaultBridge, js_name = deleteFileAsync)]
        pub fn delete_file_async(rel_path: &str);

        #[wasm_bindgen(js_namespace = EigenGraphVaultBridge, js_name = saveAutosaveArchive)]
        pub fn save_autosave_archive(json: &str);

        #[wasm_bindgen(js_namespace = EigenGraphVaultBridge, js_name = loadAutosaveArchive)]
        pub fn load_autosave_archive() -> String;

        #[wasm_bindgen(js_namespace = EigenGraphVaultBridge, js_name = openFilePicker)]
        pub fn open_file_picker(accept: &str);

        #[wasm_bindgen(js_namespace = EigenGraphVaultBridge, js_name = pollPickedFileJson)]
        pub fn poll_picked_file_json() -> String;
    }
}

pub fn vault_bridge_version() -> u64 {
    #[cfg(target_arch = "wasm32")]
    {
        js_vault::get_version() as u64
    }
    #[cfg(not(target_arch = "wasm32"))]
    {
        0
    }
}

pub fn vault_state_snapshot() -> WebVaultStateSnapshot {
    #[cfg(target_arch = "wasm32")]
    {
        let raw = js_vault::get_state_json();
        serde_json::from_str::<WebVaultStateSnapshot>(&raw).unwrap_or_default()
    }
    #[cfg(not(target_arch = "wasm32"))]
    {
        WebVaultStateSnapshot::default()
    }
}

pub fn vault_connect_existing() {
    #[cfg(target_arch = "wasm32")]
    js_vault::connect_existing_vault();
}

pub fn vault_connect_indexeddb(name: &str) {
    #[cfg(target_arch = "wasm32")]
    js_vault::connect_indexeddb_vault(name);
    #[cfg(not(target_arch = "wasm32"))]
    let _ = name;
}

pub fn vault_create_new(name: &str, force_indexeddb: bool) {
    #[cfg(target_arch = "wasm32")]
    js_vault::create_new_vault(name, force_indexeddb);
    #[cfg(not(target_arch = "wasm32"))]
    let _ = (name, force_indexeddb);
}

pub fn vault_reconnect() {
    #[cfg(target_arch = "wasm32")]
    js_vault::reconnect_vault();
}

pub fn vault_refresh() {
    #[cfg(target_arch = "wasm32")]
    js_vault::refresh_vault();
}

pub fn vault_disconnect() {
    #[cfg(target_arch = "wasm32")]
    js_vault::disconnect_vault();
}

pub fn vault_enter_guest_mode() {
    #[cfg(target_arch = "wasm32")]
    js_vault::enter_guest_mode();
}

pub fn vault_read_file(rel_path: &str) -> Option<String> {
    #[cfg(target_arch = "wasm32")]
    {
        let content = js_vault::read_file_sync(rel_path);
        if content.is_empty() {
            None
        } else {
            Some(content)
        }
    }
    #[cfg(not(target_arch = "wasm32"))]
    {
        let _ = rel_path;
        None
    }
}

pub fn vault_write_file(rel_path: &str, content: &str) {
    #[cfg(target_arch = "wasm32")]
    js_vault::write_file_async(rel_path, content);
    #[cfg(not(target_arch = "wasm32"))]
    let _ = (rel_path, content);
}

pub fn vault_delete_file(rel_path: &str) {
    #[cfg(target_arch = "wasm32")]
    js_vault::delete_file_async(rel_path);
    #[cfg(not(target_arch = "wasm32"))]
    let _ = rel_path;
}

pub fn workspace_autosave_save(json: &str) {
    #[cfg(target_arch = "wasm32")]
    js_vault::save_autosave_archive(json);
    #[cfg(not(target_arch = "wasm32"))]
    let _ = json;
}

pub fn workspace_autosave_load() -> Option<String> {
    #[cfg(target_arch = "wasm32")]
    {
        let raw = js_vault::load_autosave_archive();
        if raw.trim().is_empty() {
            None
        } else {
            Some(raw)
        }
    }
    #[cfg(not(target_arch = "wasm32"))]
    {
        None
    }
}

pub fn trigger_file_picker(accept: &str) {
    #[cfg(target_arch = "wasm32")]
    {
        js_vault::open_file_picker(accept);
    }
    #[cfg(not(target_arch = "wasm32"))]
    {
        let _ = accept;
        if let Some((filename, content)) = pick_file_native("Open EigenGraph File (.ifc, .eigen, .json, .csv)") {
            if let Ok(mut guard) = PENDING_NATIVE_IMPORTED_FILE.lock() {
                *guard = Some(ImportedFilePayload { filename, content });
            }
        }
    }
}

pub fn poll_imported_file() -> Option<ImportedFilePayload> {
    #[cfg(target_arch = "wasm32")]
    {
        let raw = js_vault::poll_picked_file_json();
        if raw.trim().is_empty() {
            return None;
        }
        serde_json::from_str::<ImportedFilePayload>(&raw).ok()
    }
    #[cfg(not(target_arch = "wasm32"))]
    {
        if let Ok(mut guard) = PENDING_NATIVE_IMPORTED_FILE.lock() {
            guard.take()
        } else {
            None
        }
    }
}

/// Opens a native OS file picker dialog and reads the selected file into UTF-8 text.
#[cfg(not(target_arch = "wasm32"))]
pub fn pick_file_native(prompt_title: &str) -> Option<(String, String)> {
    #[cfg(target_os = "macos")]
    {
        let script = format!(
            "POSIX path of (choose file with prompt \"{}\")",
            prompt_title.replace('"', "")
        );
        if let Ok(out) = std::process::Command::new("osascript")
            .args(["-e", &script])
            .output()
        {
            if out.status.success() {
                let path_str = String::from_utf8_lossy(&out.stdout).trim().to_string();
                if !path_str.is_empty() {
                    let path = std::path::PathBuf::from(&path_str);
                    if let Ok(content) = std::fs::read_to_string(&path) {
                        let fname = path
                            .file_name()
                            .and_then(|s| s.to_str())
                            .unwrap_or("imported_file")
                            .to_string();
                        return Some((fname, content));
                    }
                }
            }
        }
        None
    }
    #[cfg(target_os = "linux")]
    {
        if let Ok(out) = std::process::Command::new("zenity")
            .args(["--file-selection", &format!("--title={prompt_title}")])
            .output()
        {
            if out.status.success() {
                let path_str = String::from_utf8_lossy(&out.stdout).trim().to_string();
                if !path_str.is_empty() {
                    let path = std::path::PathBuf::from(&path_str);
                    if let Ok(content) = std::fs::read_to_string(&path) {
                        let fname = path
                            .file_name()
                            .and_then(|s| s.to_str())
                            .unwrap_or("imported_file")
                            .to_string();
                        return Some((fname, content));
                    }
                }
            }
        }
        None
    }
    #[cfg(target_os = "windows")]
    {
        let ps_cmd = format!(
            "Add-Type -AssemblyName System.Windows.Forms; $f = New-Object System.Windows.Forms.OpenFileDialog; $f.Title = '{}'; if($f.ShowDialog() -eq 'OK'){{ $f.FileName }}",
            prompt_title.replace('\'', "")
        );
        if let Ok(out) = std::process::Command::new("powershell")
            .args(["-NoProfile", "-Command", &ps_cmd])
            .output()
        {
            if out.status.success() {
                let path_str = String::from_utf8_lossy(&out.stdout).trim().to_string();
                if !path_str.is_empty() {
                    let path = std::path::PathBuf::from(&path_str);
                    if let Ok(content) = std::fs::read_to_string(&path) {
                        let fname = path
                            .file_name()
                            .and_then(|s| s.to_str())
                            .unwrap_or("imported_file")
                            .to_string();
                        return Some((fname, content));
                    }
                }
            }
        }
        None
    }
    #[cfg(not(any(target_os = "macos", target_os = "linux", target_os = "windows")))]
    {
        let _ = prompt_title;
        None
    }
}

/// Opens a native OS folder selection dialog when running natively on desktop.
#[cfg(not(target_arch = "wasm32"))]
pub fn pick_folder_native(prompt_title: &str) -> Option<String> {
    #[cfg(target_os = "macos")]
    {
        let script = format!(
            "POSIX path of (choose folder with prompt \"{}\")",
            prompt_title.replace('"', "")
        );
        if let Ok(out) = std::process::Command::new("osascript")
            .args(["-e", &script])
            .output()
        {
            if out.status.success() {
                let path = String::from_utf8_lossy(&out.stdout).trim().to_string();
                if !path.is_empty() {
                    return Some(path.trim_end_matches('/').to_string());
                }
            }
        }
        None
    }
    #[cfg(target_os = "linux")]
    {
        if let Ok(out) = std::process::Command::new("zenity")
            .args(["--file-selection", "--directory", &format!("--title={prompt_title}")])
            .output()
        {
            if out.status.success() {
                let path = String::from_utf8_lossy(&out.stdout).trim().to_string();
                if !path.is_empty() {
                    return Some(path);
                }
            }
        }
        None
    }
    #[cfg(target_os = "windows")]
    {
        let ps_cmd = format!(
            "Add-Type -AssemblyName System.Windows.Forms; $f = New-Object System.Windows.Forms.FolderBrowserDialog; $f.Description = '{}'; if($f.ShowDialog() -eq 'OK'){{ $f.SelectedPath }}",
            prompt_title.replace('\'', "")
        );
        if let Ok(out) = std::process::Command::new("powershell")
            .args(["-NoProfile", "-Command", &ps_cmd])
            .output()
        {
            if out.status.success() {
                let path = String::from_utf8_lossy(&out.stdout).trim().to_string();
                if !path.is_empty() {
                    return Some(path);
                }
            }
        }
        None
    }
    #[cfg(not(any(target_os = "macos", target_os = "linux", target_os = "windows")))]
    {
        let _ = prompt_title;
        None
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

/// Saves a UTF-8 text deliverable via browser download on `wasm32` or the local Vault `Exports` folder natively.
pub fn save_text_file(filename: &str, content: &str, mime_type: &str) -> Result<String, String> {
    #[cfg(target_arch = "wasm32")]
    {
        trigger_browser_download(filename, content.as_bytes(), mime_type)
    }
    #[cfg(not(target_arch = "wasm32"))]
    {
        let _ = mime_type;
        let exports_dir = crate::core::InternalVault::vault_root_dir().join("Exports");
        std::fs::create_dir_all(&exports_dir)
            .map_err(|e| format!("Failed to create Exports dir: {e}"))?;
        let out_path = exports_dir.join(filename);
        std::fs::write(&out_path, content.as_bytes())
            .map_err(|e| format!("Failed to write {filename}: {e}"))?;
        Ok(format!("Saved {}", out_path.display()))
    }
}
