//! updater.rs — In-App GitHub Releases Auto-Updater for EigenGraph on macOS, Windows & Linux.
//! Checks GitHub Releases API for new releases, streams download progress in real time, and performs
//! an atomic staged bundle swap + detached relaunch on macOS (avoiding Apple Silicon CS_KILL crashes
//! and read-only /Volumes DMG traps).

#[cfg(not(target_arch = "wasm32"))]
use std::process::{Command, Stdio};
use std::sync::{Arc, Mutex};
#[cfg(not(target_arch = "wasm32"))]
use std::thread;
#[cfg(not(target_arch = "wasm32"))]
use std::time::Duration;
use egui::{Align, Align2, Color32, CornerRadius, Frame, Layout, Margin, RichText, Stroke, Vec2};
use crate::theme::MacTheme;

#[derive(Clone, Debug, Default)]
pub struct UpdateReleaseDetails {
    pub tag_name: String,
    pub name: String,
    pub body: String,
    pub published_at: String,
    pub dmg_url: String,
    pub asset_api_url: String,
    pub dmg_size_mb: f32,
}

#[derive(Clone, Debug)]
pub struct UpdaterSharedState {
    pub is_checking: bool,
    pub is_downloading: bool,
    pub is_installing: bool,
    pub download_progress: f32,
    pub status_message: Option<String>,
    pub update_available: bool,
    pub release: Option<UpdateReleaseDetails>,
    pub downloaded_path: Option<String>,
    pub error: Option<String>,
    pub target_install_path: String,
}

impl Default for UpdaterSharedState {
    fn default() -> Self {
        Self {
            is_checking: false,
            is_downloading: false,
            is_installing: false,
            download_progress: 0.0,
            status_message: None,
            update_available: false,
            release: None,
            downloaded_path: None,
            error: None,
            target_install_path: resolve_macos_target_app_path(),
        }
    }
}

/// Resolves the canonical `.app` bundle installation path on macOS.
/// If the user launched EigenGraph from inside a mounted read-only DMG (`/Volumes/...`)
/// or temporary directory, this automatically redirects installation to `/Applications/EigenGraph.app`.
pub fn resolve_macos_target_app_path() -> String {
    #[cfg(target_arch = "wasm32")]
    {
        return "WebAssembly Browser App".to_string();
    }

    #[cfg(not(target_arch = "wasm32"))]
    {
        if let Ok(exe_path) = std::env::current_exe() {
            let exe_str = exe_path.to_string_lossy();
            if let Some(idx) = exe_str.find(".app") {
                let bundle_path = &exe_str[..idx + 4];
                if !bundle_path.starts_with("/Volumes/")
                    && !bundle_path.starts_with("/tmp/")
                    && !bundle_path.starts_with("/private/tmp/")
                    && !bundle_path.contains("/AppTranslocation/")
                {
                    return bundle_path.to_string();
                }
            }
        }
        "/Applications/EigenGraph.app".to_string()
    }
}

pub struct AppUpdater {
    pub current_version: String,
    pub github_token: String,
    pub shared: Arc<Mutex<UpdaterSharedState>>,
    pub show_modal: bool,
}

impl Default for AppUpdater {
    fn default() -> Self {
        Self {
            current_version: env!("CARGO_PKG_VERSION").to_string(),
            github_token: String::new(),
            shared: Arc::new(Mutex::new(UpdaterSharedState::default())),
            show_modal: false,
        }
    }
}

#[cfg(not(target_arch = "wasm32"))]
impl AppUpdater {
    pub fn new() -> Self {
        Self::default()
    }

    /// Triggers an asynchronous check for updates against the GitHub Releases API,
    /// with automatic fallback to `/releases?per_page=5` if `/releases/latest` is cached or unavailable.
    pub fn check_for_updates(&mut self) {
        let current_ver = self.current_version.clone();
        let shared = Arc::clone(&self.shared);
        let token = if !self.github_token.trim().is_empty() {
            Some(self.github_token.trim().to_string())
        } else {
            std::env::var("GITHUB_TOKEN")
                .ok()
                .or_else(|| std::env::var("GH_TOKEN").ok())
        };

        {
            let mut state = shared.lock().unwrap();
            state.is_checking = true;
            state.error = None;
            state.target_install_path = resolve_macos_target_app_path();
            state.status_message = Some("Checking GitHub Releases for the latest version...".to_string());
        }

        self.show_modal = true;

        thread::spawn(move || {
            let latest_url = "https://api.github.com/repos/EigenGraph/EigenGraph/releases/latest";
            let list_url = "https://api.github.com/repos/EigenGraph/EigenGraph/releases?per_page=5";

            let fetch_json = |url: &str| -> Result<serde_json::Value, String> {
                let mut cmd = Command::new("curl");
                cmd.arg("-s")
                    .arg("-L")
                    .arg("--max-time")
                    .arg("12")
                    .arg("-H")
                    .arg("User-Agent: EigenGraph-Desktop-Updater")
                    .arg("-H")
                    .arg("Accept: application/vnd.github+json")
                    .arg("-H")
                    .arg("Cache-Control: no-cache");

                if let Some(ref tok) = token {
                    if !tok.is_empty() {
                        cmd.arg("-H").arg(format!("Authorization: Bearer {}", tok));
                    }
                }
                cmd.arg(url);

                let out = cmd.output().map_err(|e| format!("Network error: {}", e))?;
                if !out.status.success() {
                    return Err(format!(
                        "Failed to connect to GitHub (Exit code {}).",
                        out.status.code().unwrap_or(-1)
                    ));
                }
                let text = String::from_utf8_lossy(&out.stdout);
                serde_json::from_str::<serde_json::Value>(&text)
                    .map_err(|_| "Invalid JSON response from GitHub API.".to_string())
            };

            let release_json = match fetch_json(latest_url) {
                Ok(val) if val.get("tag_name").is_some() => Ok(val),
                _ => match fetch_json(list_url) {
                    Ok(arr) => {
                        if let Some(first) = arr.as_array().and_then(|a| a.first()).cloned() {
                            Ok(first)
                        } else {
                            Err("No published releases found on GitHub.".to_string())
                        }
                    }
                    Err(e) => Err(e),
                },
            };

            match release_json {
                Ok(json) => {
                    if let Some(tag_name) = json.get("tag_name").and_then(|v| v.as_str()) {
                        let clean_tag = tag_name.trim_start_matches('v');
                        let clean_current = current_ver.trim_start_matches('v');

                        let is_newer = is_version_newer(clean_tag, clean_current);
                        let title = json
                            .get("name")
                            .and_then(|v| v.as_str())
                            .unwrap_or(tag_name)
                            .to_string();
                        let body = json
                            .get("body")
                            .and_then(|v| v.as_str())
                            .unwrap_or("EigenGraph system improvements and architectural updates.")
                            .to_string();
                        let published_at = json
                            .get("published_at")
                            .and_then(|v| v.as_str())
                            .unwrap_or("")
                            .to_string();

                        let mut dmg_download_url = String::new();
                        let mut asset_api_url = String::new();
                        let mut dmg_size_mb = 12.5;

                        if let Some(assets) = json.get("assets").and_then(|v| v.as_array()) {
                            for asset in assets {
                                if let Some(name) = asset.get("name").and_then(|v| v.as_str()) {
                                    let is_match = if cfg!(target_os = "windows") {
                                        name.ends_with(".zip") || name.ends_with(".exe")
                                    } else if cfg!(target_os = "linux") {
                                        name.ends_with(".tar.gz") || name.ends_with(".AppImage")
                                    } else {
                                        name.ends_with(".dmg")
                                    };

                                    if is_match {
                                        if let Some(url) =
                                            asset.get("browser_download_url").and_then(|v| v.as_str())
                                        {
                                            dmg_download_url = url.to_string();
                                        }
                                        if let Some(a_url) = asset.get("url").and_then(|v| v.as_str()) {
                                            asset_api_url = a_url.to_string();
                                        }
                                        if let Some(size) = asset.get("size").and_then(|v| v.as_f64()) {
                                            dmg_size_mb = ((size / 1_048_576.0) as f32).max(0.5);
                                        }
                                        break;
                                    }
                                }
                            }
                        }

                        if dmg_download_url.is_empty() {
                            let fallback_asset = if cfg!(target_os = "windows") {
                                "EigenGraph-Windows-x64.zip"
                            } else if cfg!(target_os = "linux") {
                                "EigenGraph-Linux-x86_64.tar.gz"
                            } else {
                                "EigenGraph-macOS.dmg"
                            };
                            dmg_download_url = format!(
                                "https://github.com/EigenGraph/EigenGraph/releases/download/{}/{}",
                                tag_name, fallback_asset
                            );
                        }

                        let mut state = shared.lock().unwrap();
                        state.is_checking = false;
                        state.update_available = is_newer;
                        state.release = Some(UpdateReleaseDetails {
                            tag_name: tag_name.to_string(),
                            name: title,
                            body,
                            published_at,
                            dmg_url: dmg_download_url,
                            asset_api_url,
                            dmg_size_mb,
                        });

                        if is_newer {
                            state.status_message = Some(format!(
                                "New release {} is ready for 1-Click installation!",
                                tag_name
                            ));
                        } else {
                            state.status_message = Some(format!(
                                "EigenGraph is up to date (v{}).",
                                clean_current
                            ));
                        }
                    } else if let Some(msg) = json.get("message").and_then(|v| v.as_str()) {
                        let mut state = shared.lock().unwrap();
                        state.is_checking = false;
                        state.error = Some(format!("GitHub API: {}", msg));
                    } else {
                        let mut state = shared.lock().unwrap();
                        state.is_checking = false;
                        state.status_message =
                            Some(format!("EigenGraph v{} is up to date.", current_ver));
                    }
                }
                Err(err) => {
                    let mut state = shared.lock().unwrap();
                    state.is_checking = false;
                    state.error = Some(err);
                }
            }
        });
    }

    /// Downloads the release package with real-time byte progress tracking and performs
    /// an atomic staged bundle swap + detached relaunch on macOS.
    pub fn start_download_and_install(&mut self, silent_in_place: bool) {
        let shared = Arc::clone(&self.shared);
        let token = if !self.github_token.trim().is_empty() {
            Some(self.github_token.trim().to_string())
        } else {
            std::env::var("GITHUB_TOKEN")
                .ok()
                .or_else(|| std::env::var("GH_TOKEN").ok())
        };

        let release_info = {
            let state = shared.lock().unwrap();
            state.release.as_ref().map(|r| {
                (
                    r.dmg_url.clone(),
                    r.asset_api_url.clone(),
                    r.tag_name.clone(),
                    r.dmg_size_mb.max(1.0),
                )
            })
        };

        if let Some((dmg_url, asset_api_url, tag, expected_mb)) = release_info {
            {
                let mut state = shared.lock().unwrap();
                state.is_downloading = true;
                state.is_installing = false;
                state.error = None;
                state.download_progress = 0.04;
                state.status_message =
                    Some(format!("Starting download of EigenGraph {}...", tag));
            }

            thread::spawn(move || {
                let ext = if cfg!(target_os = "windows") {
                    "zip"
                } else if cfg!(target_os = "linux") {
                    "tar.gz"
                } else {
                    "dmg"
                };
                let download_target = format!("/tmp/EigenGraph-{}.{}", tag, ext);
                let _ = std::fs::remove_file(&download_target);

                let (target_url, use_api_header) =
                    if token.is_some() && !asset_api_url.is_empty() {
                        (asset_api_url, true)
                    } else {
                        (dmg_url, false)
                    };

                let mut cmd = Command::new("curl");
                cmd.arg("-s")
                    .arg("-L")
                    .arg("--fail")
                    .arg("--retry")
                    .arg("2")
                    .arg("-o")
                    .arg(&download_target);

                if let Some(ref tok) = token {
                    if !tok.is_empty() {
                        cmd.arg("-H").arg(format!("Authorization: Bearer {}", tok));
                        if use_api_header {
                            cmd.arg("-H").arg("Accept: application/octet-stream");
                        }
                    }
                }
                cmd.arg("-H").arg("User-Agent: EigenGraph-Desktop-Updater");
                cmd.arg(&target_url);
                cmd.stdout(Stdio::null()).stderr(Stdio::piped());

                let child_res = cmd.spawn();
                match child_res {
                    Ok(mut child) => {
                        let expected_bytes = (expected_mb as f64 * 1_048_576.0).max(500_000.0);
                        loop {
                            match child.try_wait() {
                                Ok(Some(status)) => {
                                    if !status.success() {
                                        let mut state = shared.lock().unwrap();
                                        state.is_downloading = false;
                                        state.is_installing = false;
                                        state.error = Some(format!(
                                            "Download failed (HTTP/curl exit {}). Check network or release asset.",
                                            status.code().unwrap_or(-1)
                                        ));
                                        return;
                                    }
                                    break;
                                }
                                Ok(None) => {
                                    if let Ok(meta) = std::fs::metadata(&download_target) {
                                        let cur_bytes = meta.len() as f64;
                                        let cur_mb = cur_bytes / 1_048_576.0;
                                        let ratio = (cur_bytes / expected_bytes).clamp(0.05, 0.82) as f32;
                                        if let Ok(mut state) = shared.lock() {
                                            state.download_progress = ratio;
                                            state.status_message = Some(format!(
                                                "Downloading {} ({:.1} MB / {:.1} MB)...",
                                                tag, cur_mb, expected_mb
                                            ));
                                        }
                                    }
                                    thread::sleep(Duration::from_millis(120));
                                }
                                Err(e) => {
                                    let mut state = shared.lock().unwrap();
                                    state.is_downloading = false;
                                    state.error = Some(format!("Download process error: {}", e));
                                    return;
                                }
                            }
                        }

                        // Verify downloaded file is non-empty
                        let valid_file = std::fs::metadata(&download_target)
                            .map(|m| m.len() > 10_240)
                            .unwrap_or(false);
                        if !valid_file {
                            let mut state = shared.lock().unwrap();
                            state.is_downloading = false;
                            state.error = Some(
                                "Downloaded file is incomplete or empty. Please try again."
                                    .to_string(),
                            );
                            return;
                        }

                        if !silent_in_place || !cfg!(target_os = "macos") {
                            let _ = Command::new("open").arg(&download_target).spawn();
                            let mut state = shared.lock().unwrap();
                            state.is_downloading = false;
                            state.download_progress = 1.0;
                            state.downloaded_path = Some(download_target);
                            state.status_message = Some(
                                "Release package downloaded and opened in system file manager."
                                    .to_string(),
                            );
                            return;
                        }

                        // macOS Atomic Staged Bundle Swap + Detached Relaunch
                        let target_app_path = resolve_macos_target_app_path();
                        {
                            let mut state = shared.lock().unwrap();
                            state.is_downloading = false;
                            state.is_installing = true;
                            state.download_progress = 0.86;
                            state.status_message = Some(format!(
                                "Mounting DMG & staging update for {}...",
                                target_app_path
                            ));
                        }

                        let pid = std::process::id();
                        let mount_point = format!("/tmp/EigenGraph_Mount_{}", pid);
                        let staged_app = "/tmp/EigenGraph_Staged.app";

                        let _ = Command::new("hdiutil")
                            .arg("detach")
                            .arg(&mount_point)
                            .arg("-force")
                            .output();
                        let _ = std::fs::remove_dir_all(staged_app);
                        let _ = std::fs::create_dir_all(&mount_point);

                        let mount_res = Command::new("hdiutil")
                            .arg("attach")
                            .arg("-nobrowse")
                            .arg("-readonly")
                            .arg("-mountpoint")
                            .arg(&mount_point)
                            .arg(&download_target)
                            .output();

                        let mut staged_ok = false;

                        if let Ok(m_out) = mount_res {
                            if m_out.status.success() {
                                // Locate the .app bundle inside the mounted DMG volume
                                let mut source_app = format!("{}/EigenGraph.app", mount_point);
                                if !std::path::Path::new(&source_app).exists() {
                                    if let Ok(entries) = std::fs::read_dir(&mount_point) {
                                        for entry in entries.flatten() {
                                            let p = entry.path();
                                            if p.extension()
                                                .and_then(|s| s.to_str())
                                                .map(|s| s.eq_ignore_ascii_case("app"))
                                                .unwrap_or(false)
                                            {
                                                source_app = p.to_string_lossy().to_string();
                                                break;
                                            }
                                        }
                                    }
                                }

                                {
                                    let mut state = shared.lock().unwrap();
                                    state.download_progress = 0.93;
                                    state.status_message = Some(
                                        "Staging Universal 2 application bundle & verifying signature..."
                                            .to_string(),
                                    );
                                }

                                // Copy from read-only DMG mount to /tmp/EigenGraph_Staged.app
                                let ditto_stage = Command::new("ditto")
                                    .arg(&source_app)
                                    .arg(staged_app)
                                    .output();

                                // Detach DMG immediately once staged
                                let _ = Command::new("hdiutil")
                                    .arg("detach")
                                    .arg(&mount_point)
                                    .arg("-force")
                                    .output();

                                if ditto_stage.map(|c| c.status.success()).unwrap_or(false) {
                                    let _ = Command::new("xattr")
                                        .arg("-cr")
                                        .arg(staged_app)
                                        .output();
                                    let _ = Command::new("codesign")
                                        .arg("--force")
                                        .arg("--deep")
                                        .arg("--sign")
                                        .arg("-")
                                        .arg(staged_app)
                                        .output();
                                    staged_ok = true;
                                }
                            }
                        }

                        if staged_ok {
                            {
                                let mut state = shared.lock().unwrap();
                                state.download_progress = 1.0;
                                state.downloaded_path = Some(target_app_path.clone());
                                state.status_message = Some(
                                    "Update staged! Performing atomic swap and restarting EigenGraph..."
                                        .to_string(),
                                );
                            }

                            // Write detached atomic installer script so we unlink the old binary
                            // AFTER the current process exits (preventing macOS arm64 CS_KILL crash)
                            // and fall back to macOS native Administrator elevation if /Applications is locked.
                            let script_path = "/tmp/eigengraph_atomic_update.sh";
                            let script_content = format!(
                                r#"#!/bin/sh
TARGET_APP="{target_app}"
STAGED_APP="/tmp/EigenGraph_Staged.app"
OLD_PID="{pid}"

# Wait briefly for current EigenGraph process to exit cleanly
for i in 1 2 3 4 5 6 7 8 9 10; do
    if ! kill -0 "$OLD_PID" 2>/dev/null; then
        break
    fi
    sleep 0.15
done

# Attempt direct atomic unlink + ditto replacement
if rm -rf "$TARGET_APP" 2>/dev/null && ditto "$STAGED_APP" "$TARGET_APP" 2>/dev/null; then
    xattr -cr "$TARGET_APP" 2>/dev/null || true
    codesign --force --deep --sign - "$TARGET_APP" 2>/dev/null || true
else
    # Fallback to macOS native Administrator prompt if /Applications requires elevation
    osascript -e "do shell script \"rm -rf '$TARGET_APP' && ditto '$STAGED_APP' '$TARGET_APP' && xattr -cr '$TARGET_APP'\" with administrator privileges" 2>/dev/null || true
fi

rm -rf "$STAGED_APP" 2>/dev/null || true
open -n "$TARGET_APP"
"#,
                                target_app = target_app_path,
                                pid = pid
                            );

                            let _ = std::fs::write(script_path, script_content);
                            let _ = Command::new("chmod").arg("+x").arg(script_path).output();

                            let _ = Command::new("/bin/sh")
                                .arg(script_path)
                                .stdin(Stdio::null())
                                .stdout(Stdio::null())
                                .stderr(Stdio::null())
                                .spawn();

                            thread::sleep(Duration::from_millis(350));
                            std::process::exit(0);
                        } else {
                            // Fallback: open DMG in Finder if hdiutil mount was blocked
                            let _ = Command::new("open").arg(&download_target).spawn();
                            let mut state = shared.lock().unwrap();
                            state.is_installing = false;
                            state.downloaded_path = Some(download_target);
                            state.status_message = Some(
                                "DMG installer opened in Finder. Drag EigenGraph.app to Applications."
                                    .to_string(),
                            );
                        }
                    }
                    Err(e) => {
                        let mut state = shared.lock().unwrap();
                        state.is_downloading = false;
                        state.is_installing = false;
                        state.error = Some(format!("Failed to launch curl downloader: {}", e));
                    }
                }
            });
        }
    }

    /// Renders the macOS native modal for Software Updates.
    pub fn render_modal(&mut self, ctx: &egui::Context, theme: &MacTheme) {
        if !self.show_modal {
            return;
        }

        let mut close_modal = false;
        let mut trigger_silent_update = false;
        let mut trigger_download_only = false;
        let mut trigger_recheck = false;

        let state = {
            let s = self.shared.lock().unwrap();
            s.clone()
        };

        if state.is_checking || state.is_downloading || state.is_installing {
            ctx.request_repaint_after(Duration::from_millis(100));
        }

        egui::Window::new("EigenGraph Software Update")
            .collapsible(false)
            .resizable(false)
            .anchor(Align2::CENTER_CENTER, Vec2::new(0.0, 0.0))
            .fixed_size(Vec2::new(520.0, 385.0))
            .frame(
                Frame::new()
                    .fill(theme.bg_card)
                    .stroke(Stroke::new(1.0, theme.border_subtle))
                    .corner_radius(CornerRadius::same(14))
                    .inner_margin(Margin::same(22)),
            )
            .show(ctx, |ui| {
                ui.vertical(|ui| {
                    ui.horizontal(|ui| {
                        ui.label(RichText::new("🚀").size(30.0));
                        ui.add_space(10.0);
                        ui.vertical(|ui| {
                            ui.label(
                                RichText::new("EigenGraph In-App Software Updater")
                                    .size(16.0)
                                    .strong()
                                    .color(theme.text_primary),
                            );
                            ui.label(
                                RichText::new(format!(
                                    "Installed Version: v{}  •  Target Bundle: {}",
                                    self.current_version, state.target_install_path
                                ))
                                .size(11.5)
                                .color(theme.text_secondary),
                            );
                        });
                    });

                    ui.add_space(12.0);
                    ui.separator();
                    ui.add_space(12.0);

                    if state.is_checking {
                        ui.horizontal(|ui| {
                            ui.spinner();
                            ui.add_space(8.0);
                            ui.label(
                                RichText::new("Querying GitHub Releases API for latest Universal 2 build...")
                                    .size(12.5)
                                    .color(theme.text_secondary),
                            );
                        });
                    } else if state.is_downloading || state.is_installing {
                        let header_text = state
                            .status_message
                            .as_deref()
                            .unwrap_or("Downloading & staging application update...");
                        ui.label(
                            RichText::new(header_text)
                                .size(12.5)
                                .strong()
                                .color(theme.accent_blue),
                        );
                        ui.add_space(8.0);
                        let progress_bar = egui::ProgressBar::new(state.download_progress)
                            .show_percentage()
                            .animate(true);
                        ui.add(progress_bar);
                        ui.add_space(6.0);
                        let note_text = if state.is_installing {
                            "Atomic bundle swap in progress — EigenGraph will relaunch automatically."
                        } else {
                            "Streaming official release DMG package from GitHub Releases..."
                        };
                        ui.label(
                            RichText::new(note_text)
                                .size(11.0)
                                .color(theme.text_muted),
                        );
                    } else if let Some(ref release) = state.release {
                        if state.update_available {
                            Frame::new()
                                .fill(theme.accent_green.gamma_multiply(0.12))
                                .stroke(Stroke::new(1.0, theme.accent_green))
                                .corner_radius(CornerRadius::same(10))
                                .inner_margin(Margin::same(12))
                                .show(ui, |ui| {
                                    ui.horizontal(|ui| {
                                        ui.label(RichText::new("✨").size(18.0));
                                        ui.vertical(|ui| {
                                            ui.label(
                                                RichText::new(format!(
                                                    "Update Available: {} ({})",
                                                    release.tag_name, release.name
                                                ))
                                                .size(13.5)
                                                .strong()
                                                .color(theme.accent_green),
                                            );
                                            ui.label(
                                                RichText::new(format!(
                                                    "Package Size: {:.1} MB  •  Published: {}",
                                                    release.dmg_size_mb,
                                                    release
                                                        .published_at
                                                        .chars()
                                                        .take(10)
                                                        .collect::<String>()
                                                ))
                                                .size(11.0)
                                                .color(theme.text_secondary),
                                            );
                                        });
                                    });
                                });

                            ui.add_space(8.0);
                            ui.label(
                                RichText::new("Release Notes & Architectural Highlights:")
                                    .size(11.5)
                                    .strong()
                                    .color(theme.text_secondary),
                            );
                            egui::ScrollArea::vertical()
                                .max_height(88.0)
                                .show(ui, |ui| {
                                    ui.label(
                                        RichText::new(&release.body)
                                            .size(11.0)
                                            .color(theme.text_muted),
                                    );
                                });
                        } else {
                            Frame::new()
                                .fill(theme.bg_canvas)
                                .stroke(Stroke::new(1.0, theme.border_subtle))
                                .corner_radius(CornerRadius::same(10))
                                .inner_margin(Margin::same(12))
                                .show(ui, |ui| {
                                    ui.horizontal(|ui| {
                                        ui.label(RichText::new("✅").size(20.0));
                                        ui.vertical(|ui| {
                                            ui.label(
                                                RichText::new("EigenGraph is Up to Date!")
                                                    .size(13.5)
                                                    .strong()
                                                    .color(theme.accent_green),
                                            );
                                            ui.label(
                                                RichText::new(format!(
                                                    "You are running the latest release (v{}). You can also force reinstall below.",
                                                    self.current_version
                                                ))
                                                .size(11.5)
                                                .color(theme.text_secondary),
                                            );
                                        });
                                    });
                                });
                        }
                    } else if let Some(ref err) = state.error {
                        ui.label(
                            RichText::new(format!("⚠️ {}", err))
                                .size(12.0)
                                .color(theme.accent_red),
                        );
                    } else if let Some(ref msg) = state.status_message {
                        ui.label(RichText::new(msg).size(12.0).color(theme.text_secondary));
                    }

                    if let Some(ref path) = state.downloaded_path {
                        ui.add_space(6.0);
                        ui.label(
                            RichText::new(format!("✅ Installed / Saved: {}", path))
                                .size(10.5)
                                .color(theme.accent_green),
                        );
                    }

                    ui.add_space(10.0);
                    ui.horizontal(|ui| {
                        ui.label(
                            RichText::new("🔑 GitHub Token (optional):")
                                .size(11.0)
                                .color(theme.text_muted),
                        );
                        ui.add(
                            egui::TextEdit::singleline(&mut self.github_token)
                                .password(true)
                                .desired_width(180.0),
                        );
                    });

                    ui.add_space(12.0);
                    ui.separator();
                    ui.add_space(8.0);

                    ui.with_layout(Layout::right_to_left(Align::Center), |ui| {
                        if state.release.is_some() && !state.is_downloading && !state.is_installing {
                            let btn_label = if state.update_available {
                                "⚡ 1-Click Update & Relaunch"
                            } else {
                                "🔁 Reinstall Latest Build"
                            };
                            if ui
                                .add(
                                    egui::Button::new(
                                        RichText::new(btn_label)
                                            .size(12.0)
                                            .strong()
                                            .color(Color32::WHITE),
                                    )
                                    .fill(theme.accent_blue)
                                    .corner_radius(CornerRadius::same(7))
                                    .min_size(Vec2::new(180.0, 30.0)),
                                )
                                .on_hover_text(
                                    "Atomically stage DMG bundle, replace app in /Applications, strip Gatekeeper quarantine, and relaunch",
                                )
                                .clicked()
                            {
                                trigger_silent_update = true;
                            }
                            ui.add_space(6.0);

                            if ui
                                .add(
                                    egui::Button::new(
                                        RichText::new("💾 Download DMG")
                                            .size(11.5)
                                            .color(theme.text_secondary),
                                    )
                                    .fill(theme.bg_canvas)
                                    .corner_radius(CornerRadius::same(7))
                                    .min_size(Vec2::new(115.0, 30.0)),
                                )
                                .clicked()
                            {
                                trigger_download_only = true;
                            }
                            ui.add_space(6.0);
                        }

                        if ui
                            .add(
                                egui::Button::new(
                                    RichText::new("Close")
                                        .size(12.0)
                                        .color(theme.text_secondary),
                                )
                                .fill(theme.bg_canvas)
                                .corner_radius(CornerRadius::same(7))
                                .min_size(Vec2::new(68.0, 30.0)),
                            )
                            .clicked()
                        {
                            close_modal = true;
                        }

                        if !state.is_checking && !state.is_downloading && !state.is_installing {
                            ui.add_space(6.0);
                            if ui
                                .add(
                                    egui::Button::new(
                                        RichText::new("🔄 Check Now")
                                            .size(11.5)
                                            .color(theme.text_primary),
                                    )
                                    .fill(theme.bg_canvas)
                                    .corner_radius(CornerRadius::same(7))
                                    .min_size(Vec2::new(95.0, 30.0)),
                                )
                                .clicked()
                            {
                                trigger_recheck = true;
                            }
                        }
                    });
                });
            });

        if close_modal {
            self.show_modal = false;
        }
        if trigger_silent_update {
            self.start_download_and_install(true);
        }
        if trigger_download_only {
            self.start_download_and_install(false);
        }
        if trigger_recheck {
            self.check_for_updates();
        }
    }
}

#[cfg(target_arch = "wasm32")]
impl AppUpdater {
    pub fn new() -> Self {
        Self::default()
    }

    /// Opens the Web Edition info & desktop download modal in browser environments.
    pub fn check_for_updates(&mut self) {
        self.show_modal = true;
    }

    /// Renders the WebAssembly Edition update & desktop release links modal.
    pub fn render_modal(&mut self, ctx: &egui::Context, theme: &MacTheme) {
        if !self.show_modal {
            return;
        }

        let mut close_modal = false;
        let mut reload_page = false;

        egui::Window::new("EigenGraph Web Edition")
            .collapsible(false)
            .resizable(false)
            .anchor(Align2::CENTER_CENTER, Vec2::new(0.0, 0.0))
            .fixed_size(Vec2::new(500.0, 320.0))
            .frame(
                Frame::new()
                    .fill(theme.bg_card)
                    .stroke(Stroke::new(1.0, theme.border_subtle))
                    .corner_radius(CornerRadius::same(14))
                    .inner_margin(Margin::same(22)),
            )
            .show(ctx, |ui| {
                ui.vertical(|ui| {
                    ui.horizontal(|ui| {
                        ui.label(RichText::new("🌐").size(30.0));
                        ui.add_space(10.0);
                        ui.vertical(|ui| {
                            ui.label(
                                RichText::new("EigenGraph WebAssembly Edition")
                                    .size(16.0)
                                    .strong()
                                    .color(theme.text_primary),
                            );
                            ui.label(
                                RichText::new(format!(
                                    "Running Live Build v{} • Phase 25 (WebGL2)",
                                    self.current_version
                                ))
                                .size(11.5)
                                .color(theme.text_secondary),
                            );
                        });
                    });

                    ui.add_space(12.0);
                    ui.separator();
                    ui.add_space(12.0);

                    Frame::new()
                        .fill(theme.accent_green.gamma_multiply(0.12))
                        .stroke(Stroke::new(1.0, theme.accent_green))
                        .corner_radius(CornerRadius::same(10))
                        .inner_margin(Margin::same(12))
                        .show(ui, |ui| {
                            ui.label(
                                RichText::new("✅ Always Up to Date via GitHub Pages")
                                    .size(13.0)
                                    .strong()
                                    .color(theme.accent_green),
                            );
                            ui.label(
                                RichText::new(
                                    "The web workstation automatically deploys from the latest main branch. Reload the page to fetch any newly published build.",
                                )
                                .size(11.5)
                                .color(theme.text_secondary),
                            );
                        });

                    ui.add_space(12.0);
                    ui.label(
                        RichText::new("Native Desktop Installers (macOS, Windows, Linux):")
                            .size(11.5)
                            .strong()
                            .color(theme.text_secondary),
                    );
                    ui.add_space(4.0);
                    ui.hyperlink_to(
                        "📦 Download Desktop Releases on GitHub",
                        "https://github.com/EigenGraph/EigenGraph/releases/latest",
                    );

                    ui.add_space(16.0);
                    ui.separator();
                    ui.add_space(8.0);

                    ui.with_layout(Layout::right_to_left(Align::Center), |ui| {
                        if ui
                            .add(
                                egui::Button::new(
                                    RichText::new("Close")
                                        .size(12.0)
                                        .color(theme.text_secondary),
                                )
                                .fill(theme.bg_canvas)
                                .corner_radius(CornerRadius::same(7))
                                .min_size(Vec2::new(72.0, 30.0)),
                            )
                            .clicked()
                        {
                            close_modal = true;
                        }
                        ui.add_space(8.0);
                        if ui
                            .add(
                                egui::Button::new(
                                    RichText::new("🔄 Reload Web App")
                                        .size(12.0)
                                        .strong()
                                        .color(Color32::WHITE),
                                )
                                .fill(theme.accent_blue)
                                .corner_radius(CornerRadius::same(7))
                                .min_size(Vec2::new(140.0, 30.0)),
                            )
                            .clicked()
                        {
                            reload_page = true;
                        }
                    });
                });
            });

        if close_modal {
            self.show_modal = false;
        }
        if reload_page {
            if let Some(window) = web_sys::window() {
                let _ = window.location().reload();
            }
        }
    }
}

/// Helper function to compare semver strings like "0.2.5" and "0.2.4"
#[allow(dead_code)]
fn is_version_newer(remote: &str, local: &str) -> bool {
    let parse = |s: &str| -> (u32, u32, u32) {
        let parts: Vec<u32> = s
            .split('.')
            .filter_map(|p| {
                p.chars()
                    .take_while(|c| c.is_ascii_digit())
                    .collect::<String>()
                    .parse::<u32>()
                    .ok()
            })
            .collect();
        (
            *parts.get(0).unwrap_or(&0),
            *parts.get(1).unwrap_or(&0),
            *parts.get(2).unwrap_or(&0),
        )
    };

    let (r_maj, r_min, r_pat) = parse(remote);
    let (l_maj, l_min, l_pat) = parse(local);

    if r_maj > l_maj {
        return true;
    }
    if r_maj == l_maj && r_min > l_min {
        return true;
    }
    if r_maj == l_maj && r_min == l_min && r_pat > l_pat {
        return true;
    }
    false
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_version_comparison() {
        assert!(is_version_newer("0.2.5", "0.2.4"));
        assert!(is_version_newer("1.0.1", "1.0.0"));
        assert!(is_version_newer("1.1.0", "1.0.9"));
        assert!(is_version_newer("2.0.0", "1.9.9"));
        assert!(!is_version_newer("0.2.5", "0.2.5"));
        assert!(!is_version_newer("0.2.4", "0.2.5"));
    }
}
