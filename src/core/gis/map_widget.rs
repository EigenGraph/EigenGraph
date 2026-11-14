//! map_widget.rs — Slippy Map & Web Mercator Quadtree Engine for GIS Site Inception (Phase 23 / v0.2.3).
//!
//! Provides mathematically rigorous Web Mercator (EPSG:3857) projection, quadtree tile indexing,
//! smooth continuous panning/zooming, geodetic site pinning, and interactive cartographic HUD telemetry.

use std::f64::consts::PI;
use egui::{Color32, CornerRadius, FontId, Pos2, Rect, Stroke, Ui, Vec2};

/// WGS84 Earth semi-major axis radius in meters (EPSG:3857 / WGS84 ellipsoid spherical approximation).
pub const EARTH_RADIUS_METERS: f64 = 6378137.0;

/// Half circumference of the Earth in Web Mercator projection (meters).
pub const EARTH_HALF_CIRCUMFERENCE: f64 = PI * EARTH_RADIUS_METERS; // ≈ 20,037,508.34 m

/// Maximum valid latitude for Web Mercator projection to avoid asymptotic infinity: ~85.05112878°.
pub const MAX_LATITUDE: f64 = 85.0511287798066;
pub const MIN_LATITUDE: f64 = -85.0511287798066;

/// Supported tile provider themes and raster/vector sources.
#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum TileSource {
    OpenStreetMap,
    OpenTopoMap,
    EsriWorldImagery,
    CartoDbDarkMatter,
    SyntheticVectorTerrain,
}

impl TileSource {
    pub fn display_name(&self) -> &'static str {
        match self {
            Self::OpenStreetMap => "🗺️ OpenStreetMap Standard",
            Self::OpenTopoMap => "⛰️ OpenTopoMap Elevation",
            Self::EsriWorldImagery => "🛰️ Esri World Imagery (Satellite)",
            Self::CartoDbDarkMatter => "🌌 CartoDB Dark Matter",
            Self::SyntheticVectorTerrain => "📐 Synthetic Vector Terrain (Offline)",
        }
    }

    pub fn url_template(&self) -> &'static str {
        match self {
            Self::OpenStreetMap => "https://tile.openstreetmap.org/{z}/{x}/{y}.png",
            Self::OpenTopoMap => "https://tile.opentopomap.org/{z}/{x}/{y}.png",
            Self::EsriWorldImagery => "https://server.arcgisonline.com/ArcGIS/rest/services/World_Imagery/MapServer/tile/{z}/{y}/{x}",
            Self::CartoDbDarkMatter => "https://basemaps.cartocdn.com/rastertiles/dark_all/{z}/{x}/{y}.png",
            Self::SyntheticVectorTerrain => "internal://procedural/vector/{z}/{x}/{y}",
        }
    }
}

/// A pinned geodetic site location for BIM project placement, soil investigation, and seismic zoning.
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct SiteGeodeticPin {
    pub id: usize,
    pub label: String,
    pub lat: f64,
    pub lon: f64,
    pub elevation_m: f32,
    pub project_name: String,
    pub soil_class: String,
    pub ss_g: f32,
    pub s1_g: f32,
    pub notes: String,
}

/// Mathematical Web Mercator (EPSG:3857) and Slippy Map Quadtree Projection Routines.
pub struct WebMercator;

impl WebMercator {
    /// Clamps latitude to valid Web Mercator bounds [-85.051129°, +85.051129°].
    #[inline]
    pub fn clamp_lat(lat: f64) -> f64 {
        lat.clamp(MIN_LATITUDE, MAX_LATITUDE)
    }

    /// Normalizes longitude to [-180.0°, +180.0°].
    #[inline]
    pub fn normalize_lon(lon: f64) -> f64 {
        let mut l = lon % 360.0;
        if l > 180.0 {
            l -= 360.0;
        } else if l < -180.0 {
            l += 360.0;
        }
        l
    }

    /// Converts WGS84 Geodetic coordinates (lat, lon) in degrees to Projected Web Mercator (x, y) in meters.
    pub fn lat_lon_to_meters(lat: f64, lon: f64) -> (f64, f64) {
        let lat = Self::clamp_lat(lat);
        let lon = Self::normalize_lon(lon);

        let x = lon.to_radians() * EARTH_RADIUS_METERS;
        let lat_rad = lat.to_radians();
        let y = EARTH_RADIUS_METERS * ((PI / 4.0 + lat_rad / 2.0).tan().ln());
        (x, y)
    }

    /// Converts Projected Web Mercator meters (x, y) back to WGS84 Geodetic (lat, lon) in degrees.
    pub fn meters_to_lat_lon(x: f64, y: f64) -> (f64, f64) {
        let lon = (x / EARTH_RADIUS_METERS).to_degrees();
        let lat = (2.0 * (y / EARTH_RADIUS_METERS).exp().atan() - PI / 2.0).to_degrees();
        (Self::clamp_lat(lat), Self::normalize_lon(lon))
    }

    /// Converts WGS84 Geodetic (lat, lon) to continuous floating-point tile coordinates (X, Y) at zoom level z.
    pub fn lat_lon_to_tile_coords(lat: f64, lon: f64, zoom: f32) -> (f64, f64) {
        let lat = Self::clamp_lat(lat);
        let lon = Self::normalize_lon(lon);
        let n = 2.0_f64.powf(zoom as f64);

        let x = (lon + 180.0) / 360.0 * n;
        let lat_rad = lat.to_radians();
        let y = (1.0 - ((lat_rad.tan() + 1.0 / lat_rad.cos()).ln() / PI)) / 2.0 * n;
        (x, y)
    }

    /// Converts floating-point tile coordinates (X, Y) at zoom level z back to WGS84 Geodetic (lat, lon).
    pub fn tile_coords_to_lat_lon(x: f64, y: f64, zoom: f32) -> (f64, f64) {
        let n = 2.0_f64.powf(zoom as f64);
        let lon = x / n * 360.0 - 180.0;
        let sinh_val = (PI * (1.0 - 2.0 * y / n)).sinh();
        let lat = sinh_val.atan().to_degrees();
        (Self::clamp_lat(lat), Self::normalize_lon(lon))
    }

    /// Calculates ground resolution in meters per pixel at a given latitude and zoom level (assuming 256px tiles).
    pub fn ground_resolution_m_px(lat: f64, zoom: f32) -> f64 {
        let lat = Self::clamp_lat(lat);
        (156543.03392 * lat.to_radians().cos()) / 2.0_f64.powf(zoom as f64)
    }

    /// Calculates the integer discrete tile index (tile_x, tile_y, zoom_int) for tile fetching.
    pub fn get_tile_index(lat: f64, lon: f64, zoom_int: u8) -> (u32, u32, u8) {
        let (tx, ty) = Self::lat_lon_to_tile_coords(lat, lon, zoom_int as f32);
        let max_tile = (1u64 << zoom_int).saturating_sub(1) as u32;
        let x = (tx.floor() as i64).clamp(0, max_tile as i64) as u32;
        let y = (ty.floor() as i64).clamp(0, max_tile as i64) as u32;
        (x, y, zoom_int)
    }
}

/// Real-time HUD Telemetry information for inspection.
#[derive(Debug, Clone)]
pub struct SlippyMapTelemetry {
    pub lat: f64,
    pub lon: f64,
    pub mercator_x: f64,
    pub mercator_y: f64,
    pub zoom: f32,
    pub tile_x: u32,
    pub tile_y: u32,
    pub tile_z: u8,
    pub ground_resolution_m_px: f64,
    pub pins_count: usize,
}

/// Interactive Slippy Map State Container for Pillar 1D (Research & GIS Site Inception).
#[derive(Debug, Clone)]
pub struct SlippyMapState {
    pub center_lat_lon: (f64, f64),
    pub zoom_level: f32, // Continuous zoom 1.0 to 19.0
    pub tile_provider: TileSource,
    pub active_pin: Option<(f64, f64)>,
    pub site_pins: Vec<SiteGeodeticPin>,
    pub crosshair_active: bool,
    pub show_grid: bool,
    pub show_scale_bar: bool,
    pub show_elevation_contours: bool,
    pub is_dragging: bool,
    pub drag_start_mouse: Option<Pos2>,
    pub drag_start_center: (f64, f64),
    pub next_pin_id: usize,
}

impl Default for SlippyMapState {
    fn default() -> Self {
        let mut state = Self {
            // Default center: Monas National Monument, Jakarta, Indonesia (-6.1754°, 106.8272°)
            center_lat_lon: (-6.175392, 106.827153),
            zoom_level: 14.5,
            tile_provider: TileSource::SyntheticVectorTerrain,
            active_pin: Some((-6.175392, 106.827153)),
            site_pins: Vec::new(),
            crosshair_active: true,
            show_grid: true,
            show_scale_bar: true,
            show_elevation_contours: true,
            is_dragging: false,
            drag_start_mouse: None,
            drag_start_center: (-6.175392, 106.827153),
            next_pin_id: 1,
        };

        // Preload standard engineering benchmark site pins
        state.site_pins.push(SiteGeodeticPin {
            id: 1,
            label: "Monas Station Benchmark Lot 01".to_string(),
            lat: -6.175392,
            lon: 106.827153,
            elevation_m: 14.2,
            project_name: "Jakarta North-South Transit Hub".to_string(),
            soil_class: "Site Class D (Stiff Soil)".to_string(),
            ss_g: 0.850,
            s1_g: 0.420,
            notes: "Design Ground Acceleration Ss=0.85g per SNI 1726:2019 / PusGeN 2017".to_string(),
        });
        state.next_pin_id = 2;

        state
    }
}

impl SlippyMapState {
    pub fn new() -> Self {
        Self::default()
    }

    /// Pans the map center by delta pixel offset on screen.
    pub fn pan_by_pixels(&mut self, dx: f32, dy: f32) {
        let (center_x, center_y) = WebMercator::lat_lon_to_tile_coords(
            self.center_lat_lon.0,
            self.center_lat_lon.1,
            self.zoom_level,
        );
        // 256 pixels per tile
        let new_tile_x = center_x - (dx as f64) / 256.0;
        let new_tile_y = center_y - (dy as f64) / 256.0;
        self.center_lat_lon = WebMercator::tile_coords_to_lat_lon(new_tile_x, new_tile_y, self.zoom_level);
    }

    /// Zooms in smoothly by delta.
    pub fn zoom_in(&mut self, delta: f32) {
        self.zoom_level = (self.zoom_level + delta).clamp(1.0, 19.0);
    }

    /// Zooms out smoothly by delta.
    pub fn zoom_out(&mut self, delta: f32) {
        self.zoom_level = (self.zoom_level - delta).clamp(1.0, 19.0);
    }

    /// Sets the map center to specific geodetic coordinates.
    pub fn set_center(&mut self, lat: f64, lon: f64) {
        self.center_lat_lon = (WebMercator::clamp_lat(lat), WebMercator::normalize_lon(lon));
    }

    /// Converts screen position within viewport rect to WGS84 Geodetic coordinates (lat, lon).
    pub fn screen_to_lat_lon(&self, pos: Pos2, rect: Rect) -> (f64, f64) {
        let center_pos = rect.center();
        let dx = pos.x - center_pos.x;
        let dy = pos.y - center_pos.y;

        let (center_x, center_y) = WebMercator::lat_lon_to_tile_coords(
            self.center_lat_lon.0,
            self.center_lat_lon.1,
            self.zoom_level,
        );
        let tile_x = center_x + (dx as f64) / 256.0;
        let tile_y = center_y + (dy as f64) / 256.0;

        WebMercator::tile_coords_to_lat_lon(tile_x, tile_y, self.zoom_level)
    }

    /// Converts WGS84 Geodetic coordinates (lat, lon) to screen pixel position within viewport rect.
    pub fn lat_lon_to_screen(&self, lat: f64, lon: f64, rect: Rect) -> Pos2 {
        let (center_x, center_y) = WebMercator::lat_lon_to_tile_coords(
            self.center_lat_lon.0,
            self.center_lat_lon.1,
            self.zoom_level,
        );
        let (target_x, target_y) = WebMercator::lat_lon_to_tile_coords(lat, lon, self.zoom_level);

        let dx = ((target_x - center_x) * 256.0) as f32;
        let dy = ((target_y - center_y) * 256.0) as f32;

        Pos2::new(rect.center().x + dx, rect.center().y + dy)
    }

    /// Adds a new site pin at current active coordinates.
    pub fn add_pin(&mut self, lat: f64, lon: f64, label: &str) -> usize {
        let id = self.next_pin_id;
        self.next_pin_id += 1;

        let elevation_est = 10.0 + ((lat.abs() * 100.0).sin() * 25.0).abs() as f32;
        self.site_pins.push(SiteGeodeticPin {
            id,
            label: label.to_string(),
            lat,
            lon,
            elevation_m: elevation_est,
            project_name: format!("Project Lot {:03}", id),
            soil_class: "Site Class D (Standard Stiff Clay)".to_string(),
            ss_g: 0.825,
            s1_g: 0.395,
            notes: "Direct geodetic site pin dropped via EigenGraph GIS Inception Canvas".to_string(),
        });
        self.active_pin = Some((lat, lon));
        id
    }

    /// Removes a site pin by ID.
    pub fn remove_pin(&mut self, id: usize) {
        self.site_pins.retain(|p| p.id != id);
        if let Some(active) = self.active_pin {
            if !self.site_pins.iter().any(|p| (p.lat - active.0).abs() < 1e-6 && (p.lon - active.1).abs() < 1e-6) {
                self.active_pin = self.site_pins.first().map(|p| (p.lat, p.lon));
            }
        }
    }

    /// Extracts real-time telemetry HUD summary.
    pub fn telemetry(&self) -> SlippyMapTelemetry {
        let (mx, my) = WebMercator::lat_lon_to_meters(self.center_lat_lon.0, self.center_lat_lon.1);
        let z_int = self.zoom_level.floor() as u8;
        let (tx, ty, tz) = WebMercator::get_tile_index(self.center_lat_lon.0, self.center_lat_lon.1, z_int);
        let res = WebMercator::ground_resolution_m_px(self.center_lat_lon.0, self.zoom_level);

        SlippyMapTelemetry {
            lat: self.center_lat_lon.0,
            lon: self.center_lat_lon.1,
            mercator_x: mx,
            mercator_y: my,
            zoom: self.zoom_level,
            tile_x: tx,
            tile_y: ty,
            tile_z: tz,
            ground_resolution_m_px: res,
            pins_count: self.site_pins.len(),
        }
    }

    /// Renders the complete interactive slippy map canvas widget with high-performance vector cartography.
    pub fn render(&mut self, ui: &mut Ui, rect: Rect, theme: &crate::theme::MacTheme) {
        let painter = ui.painter_at(rect);
        let response = ui.interact(rect, ui.id().with("slippy_map_canvas"), egui::Sense::click_and_drag());

        // 1. Mouse Drag Panning
        if response.drag_started() {
            self.is_dragging = true;
            self.drag_start_mouse = response.interact_pointer_pos();
            self.drag_start_center = self.center_lat_lon;
        }

        if self.is_dragging {
            if response.dragged() {
                let delta = response.drag_delta();
                self.pan_by_pixels(delta.x, delta.y);
            }
            if response.drag_stopped() {
                self.is_dragging = false;
                self.drag_start_mouse = None;
            }
        }

        // 2. Mouse Wheel Smooth Zooming (Centering on Cursor)
        let scroll_delta = ui.input(|i| i.smooth_scroll_delta.y);
        if response.hovered()
            && scroll_delta.abs() > 0.5
            && !crate::ui_kit::is_canvas_frozen(ui.ctx())
        {
            let zoom_delta = scroll_delta * 0.003;
            if let Some(mouse_pos) = response.hover_pos() {
                let (mouse_lat, mouse_lon) = self.screen_to_lat_lon(mouse_pos, rect);
                self.zoom_level = (self.zoom_level + zoom_delta).clamp(1.0, 19.0);
                // Adjust center so mouse position stays anchored
                let new_mouse_screen = self.lat_lon_to_screen(mouse_lat, mouse_lon, rect);
                let corr_x = new_mouse_screen.x - mouse_pos.x;
                let corr_y = new_mouse_screen.y - mouse_pos.y;
                self.pan_by_pixels(corr_x, corr_y);
            } else {
                self.zoom_level = (self.zoom_level + zoom_delta).clamp(1.0, 19.0);
            }
        }

        // 3. Double-Click or Click to Drop Pin
        if response.double_clicked() {
            if let Some(pos) = response.interact_pointer_pos() {
                let (lat, lon) = self.screen_to_lat_lon(pos, rect);
                self.add_pin(lat, lon, &format!("Site Pin #{:02}", self.site_pins.len() + 1));
            }
        } else if response.clicked() {
            if let Some(pos) = response.interact_pointer_pos() {
                let (lat, lon) = self.screen_to_lat_lon(pos, rect);
                self.active_pin = Some((lat, lon));
            }
        }

        // 4. Cartographic Background Rendering (Water & Landmasses)
        self.render_cartographic_basemap(&painter, rect, theme);

        // 5. Geodetic Coordinate Grid (Equator, Meridians, Parallels)
        if self.show_grid {
            self.render_geodetic_grid(&painter, rect, theme);
        }

        // 6. Topographic Elevation Iso-Contours (TIN preview simulation)
        if self.show_elevation_contours && self.zoom_level >= 10.0 {
            self.render_elevation_contours(&painter, rect, theme);
        }

        // 7. Render Geodetic Site Pins & Active Pin Highlight
        self.render_site_pins(&painter, rect, theme);

        // 8. Viewport Crosshair HUD
        if self.crosshair_active {
            self.render_crosshair(&painter, rect, theme);
        }

        // 9. Metric Dynamic Scale Bar
        if self.show_scale_bar {
            self.render_scale_bar(&painter, rect, theme);
        }
    }

    fn render_cartographic_basemap(&self, painter: &egui::Painter, rect: Rect, theme: &crate::theme::MacTheme) {
        // Deep oceanic background fill
        let ocean_color = match self.tile_provider {
            TileSource::CartoDbDarkMatter => Color32::from_rgb(10, 14, 20),
            TileSource::EsriWorldImagery => Color32::from_rgb(18, 28, 42),
            TileSource::OpenTopoMap => Color32::from_rgb(212, 230, 241),
            _ => Color32::from_rgb(20, 28, 38),
        };
        painter.rect_filled(rect, CornerRadius::same(8), ocean_color);

        // Landmass color
        let land_color = match self.tile_provider {
            TileSource::CartoDbDarkMatter => Color32::from_rgb(24, 30, 40),
            TileSource::EsriWorldImagery => Color32::from_rgb(38, 54, 40),
            TileSource::OpenTopoMap => Color32::from_rgb(245, 238, 220),
            _ => Color32::from_rgb(32, 42, 54),
        };

        // Procedural continental & major landmass polygons (World & Indonesian Archipelago High-Fidelity)
        let landmasses: &[&[(f64, f64)]] = &[
            // Java Island
            &[(-6.0, 105.2), (-5.9, 106.8), (-6.2, 108.5), (-6.8, 111.5), (-7.0, 114.6), (-7.8, 114.5), (-8.3, 113.2), (-8.4, 110.5), (-7.7, 108.5), (-7.0, 106.5), (-6.8, 105.1)],
            // Sumatra Island
            &[(5.5, 95.3), (4.0, 98.5), (2.0, 100.5), (-0.5, 103.5), (-3.0, 106.0), (-5.8, 105.8), (-5.0, 103.5), (-3.5, 102.0), (-0.5, 99.5), (2.5, 96.5), (5.5, 95.3)],
            // Kalimantan / Borneo
            &[(7.0, 116.5), (4.5, 118.5), (1.5, 119.0), (-1.2, 117.2), (-3.5, 116.0), (-4.2, 114.5), (-3.5, 110.5), (-1.0, 109.0), (1.5, 109.5), (4.0, 114.0), (7.0, 116.5)],
            // Sulawesi
            &[(1.8, 125.0), (0.5, 120.0), (-1.0, 119.5), (-5.5, 119.5), (-5.0, 120.5), (-3.0, 121.5), (-5.0, 123.0), (-3.0, 123.0), (-1.0, 121.5), (0.8, 123.5), (1.8, 125.0)],
            // Bali & Nusa Tenggara
            &[(-8.1, 114.4), (-8.1, 115.7), (-8.8, 115.6), (-8.7, 114.5)],
            &[(-8.2, 115.9), (-8.2, 119.2), (-8.9, 119.0), (-8.9, 116.0)],
            // Papua / New Guinea
            &[(-0.8, 131.0), (-2.5, 140.0), (-8.5, 141.0), (-8.0, 137.5), (-4.5, 135.0), (-3.0, 131.5), (-0.8, 131.0)],
            // Eurasia & Asia Mainland
            &[(70.0, 30.0), (70.0, 170.0), (50.0, 140.0), (30.0, 120.0), (15.0, 105.0), (1.0, 103.5), (10.0, 75.0), (25.0, 60.0), (35.0, 40.0), (50.0, 30.0)],
            // Australia
            &[(-11.0, 130.0), (-12.0, 142.0), (-25.0, 153.0), (-38.0, 148.0), (-35.0, 115.0), (-22.0, 114.0), (-15.0, 125.0)],
        ];

        for land in landmasses {
            let pts: Vec<Pos2> = land.iter().map(|&(lat, lon)| self.lat_lon_to_screen(lat, lon, rect)).collect();
            if pts.len() >= 3 {
                painter.add(egui::Shape::convex_polygon(pts, land_color, Stroke::new(1.0, theme.border_subtle)));
            }
        }
    }

    fn render_geodetic_grid(&self, painter: &egui::Painter, rect: Rect, theme: &crate::theme::MacTheme) {
        let grid_stroke = Stroke::new(0.8, Color32::from_white_alpha(35));
        let equator_stroke = Stroke::new(1.4, theme.accent_blue.gamma_multiply(0.7));

        // Parallels (Equator, ±20°, ±40°, ±60°)
        for lat in [-60.0, -40.0, -20.0, 0.0, 20.0, 40.0, 60.0] {
            let p1 = self.lat_lon_to_screen(lat, -180.0, rect);
            let p2 = self.lat_lon_to_screen(lat, 180.0, rect);
            let stroke = if lat.abs() < 1e-3 { equator_stroke } else { grid_stroke };
            if p1.y >= rect.min.y && p1.y <= rect.max.y {
                painter.line_segment([Pos2::new(rect.min.x, p1.y), Pos2::new(rect.max.x, p1.y)], stroke);
                painter.text(
                    Pos2::new(rect.min.x + 8.0, p1.y - 10.0),
                    egui::Align2::LEFT_BOTTOM,
                    format!("{:+.0}°", lat),
                    FontId::monospace(9.0),
                    theme.text_muted,
                );
            }
        }

        // Meridians (every 30°)
        for lon in (-180..=180).step_by(30) {
            let lon_f = lon as f64;
            let p = self.lat_lon_to_screen(0.0, lon_f, rect);
            if p.x >= rect.min.x && p.x <= rect.max.x {
                painter.line_segment([Pos2::new(p.x, rect.min.y), Pos2::new(p.x, rect.max.y)], grid_stroke);
                painter.text(
                    Pos2::new(p.x + 4.0, rect.max.y - 12.0),
                    egui::Align2::LEFT_BOTTOM,
                    format!("{:+.0}°", lon_f),
                    FontId::monospace(9.0),
                    theme.text_muted,
                );
            }
        }
    }

    fn render_elevation_contours(&self, painter: &egui::Painter, rect: Rect, theme: &crate::theme::MacTheme) {
        // High zoom: draw simulated elevation contours around current center
        let (clat, clon) = self.center_lat_lon;
        let contour_stroke = Stroke::new(1.0, theme.accent_green.gamma_multiply(0.4));

        for r in 1..=4 {
            let radius_km = (r as f64) * 0.8;
            let deg_delta = radius_km / 111.0;
            let mut contour_pts = Vec::new();
            for step in 0..=36 {
                let theta = (step as f64) * (2.0 * PI / 36.0);
                // Perturb with harmonic pseudo-terrain ripple
                let perturbation = 1.0 + 0.15 * (4.0 * theta).sin() + 0.08 * (7.0 * theta).cos();
                let lat = clat + (deg_delta * perturbation) * theta.sin();
                let lon = clon + (deg_delta * perturbation) * theta.cos() / clat.to_radians().cos();
                contour_pts.push(self.lat_lon_to_screen(lat, lon, rect));
            }
            if contour_pts.len() >= 2 {
                for window in contour_pts.windows(2) {
                    painter.line_segment([window[0], window[1]], contour_stroke);
                }
            }
        }
    }

    fn render_site_pins(&self, painter: &egui::Painter, rect: Rect, theme: &crate::theme::MacTheme) {
        for pin in &self.site_pins {
            let pos = self.lat_lon_to_screen(pin.lat, pin.lon, rect);
            if rect.contains(pos) {
                let is_active = self.active_pin.map(|p| (p.0 - pin.lat).abs() < 1e-6 && (p.1 - pin.lon).abs() < 1e-6).unwrap_or(false);

                // Glow ring
                if is_active {
                    painter.circle_stroke(pos, 16.0, Stroke::new(2.0, theme.accent_blue.gamma_multiply(0.8)));
                    painter.circle_filled(pos, 14.0, theme.accent_blue.gamma_multiply(0.2));
                }

                // Pin circle and icon
                painter.circle_filled(pos, 8.0, theme.accent_blue);
                painter.circle_stroke(pos, 8.0, Stroke::new(2.0, Color32::WHITE));

                // Pin label card
                let label_pos = Pos2::new(pos.x + 12.0, pos.y - 8.0);
                let label_text = format!("📍 #{}: {} ({:.1}m)", pin.id, pin.label, pin.elevation_m);
                let text_color = if is_active { theme.accent_blue } else { theme.text_primary };

                painter.rect_filled(
                    Rect::from_min_size(label_pos, Vec2::new(label_text.len() as f32 * 6.5 + 8.0, 18.0)),
                    CornerRadius::same(4),
                    theme.bg_card.gamma_multiply(0.85),
                );
                painter.text(
                    Pos2::new(label_pos.x + 4.0, label_pos.y + 2.0),
                    egui::Align2::LEFT_TOP,
                    label_text,
                    FontId::proportional(10.5),
                    text_color,
                );
            }
        }
    }

    fn render_crosshair(&self, painter: &egui::Painter, rect: Rect, theme: &crate::theme::MacTheme) {
        let c = rect.center();
        let crosshair_stroke = Stroke::new(1.2, theme.accent_blue.gamma_multiply(0.85));
        let len = 12.0;

        // Central cross lines
        painter.line_segment([Pos2::new(c.x - len, c.y), Pos2::new(c.x - 3.0, c.y)], crosshair_stroke);
        painter.line_segment([Pos2::new(c.x + 3.0, c.y), Pos2::new(c.x + len, c.y)], crosshair_stroke);
        painter.line_segment([Pos2::new(c.x, c.y - len), Pos2::new(c.x, c.y - 3.0)], crosshair_stroke);
        painter.line_segment([Pos2::new(c.x, c.y + 3.0), Pos2::new(c.x, c.y + len)], crosshair_stroke);

        // Center target ring
        painter.circle_stroke(c, 4.0, crosshair_stroke);
    }

    fn render_scale_bar(&self, painter: &egui::Painter, rect: Rect, theme: &crate::theme::MacTheme) {
        let res_m_px = WebMercator::ground_resolution_m_px(self.center_lat_lon.0, self.zoom_level);
        let target_bar_width_px = 120.0;
        let ground_meters = target_bar_width_px * res_m_px;

        // Round to clean metric increments (10m, 50m, 100m, 500m, 1km, 5km, etc.)
        let nice_meters = if ground_meters >= 5000.0 {
            (ground_meters / 5000.0).round() * 5000.0
        } else if ground_meters >= 1000.0 {
            (ground_meters / 1000.0).round() * 1000.0
        } else if ground_meters >= 100.0 {
            (ground_meters / 100.0).round() * 100.0
        } else {
            (ground_meters / 10.0).max(1.0).round() * 10.0
        };

        let actual_bar_width_px = (nice_meters / res_m_px) as f32;
        let bar_origin = Pos2::new(rect.min.x + 16.0, rect.max.y - 28.0);
        let bar_end = Pos2::new(bar_origin.x + actual_bar_width_px, bar_origin.y);

        // Background backing
        painter.rect_filled(
            Rect::from_min_max(Pos2::new(bar_origin.x - 6.0, bar_origin.y - 14.0), Pos2::new(bar_end.x + 6.0, bar_origin.y + 12.0)),
            CornerRadius::same(4),
            theme.bg_card.gamma_multiply(0.85),
        );

        // Scale ticks & line
        let bar_stroke = Stroke::new(1.8, theme.text_primary);
        painter.line_segment([bar_origin, bar_end], bar_stroke);
        painter.line_segment([Pos2::new(bar_origin.x, bar_origin.y - 4.0), Pos2::new(bar_origin.x, bar_origin.y + 4.0)], bar_stroke);
        painter.line_segment([Pos2::new(bar_end.x, bar_end.y - 4.0), Pos2::new(bar_end.x, bar_end.y + 4.0)], bar_stroke);

        let scale_label = if nice_meters >= 1000.0 {
            format!("{:.1} km", nice_meters / 1000.0)
        } else {
            format!("{:.0} m", nice_meters)
        };

        painter.text(
            Pos2::new((bar_origin.x + bar_end.x) / 2.0, bar_origin.y - 6.0),
            egui::Align2::CENTER_BOTTOM,
            scale_label,
            FontId::monospace(10.0),
            theme.text_primary,
        );
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_web_mercator_projection_roundtrip() {
        // Monas Jakarta (-6.1754°, 106.8272°)
        let lat = -6.175392;
        let lon = 106.827153;

        let (x, y) = WebMercator::lat_lon_to_meters(lat, lon);
        let (lat_back, lon_back) = WebMercator::meters_to_lat_lon(x, y);

        assert!((lat - lat_back).abs() < 1e-6, "Latitude roundtrip mismatch: {} vs {}", lat, lat_back);
        assert!((lon - lon_back).abs() < 1e-6, "Longitude roundtrip mismatch: {} vs {}", lon, lon_back);
    }

    #[test]
    fn test_tile_coordinates_conversion() {
        let lat = 0.0;
        let lon = 0.0;
        let zoom = 1.0;

        let (tx, ty) = WebMercator::lat_lon_to_tile_coords(lat, lon, zoom);
        assert!((tx - 1.0).abs() < 1e-5);
        assert!((ty - 1.0).abs() < 1e-5);

        let (lat_back, lon_back) = WebMercator::tile_coords_to_lat_lon(tx, ty, zoom);
        assert!((lat_back - 0.0).abs() < 1e-5);
        assert!((lon_back - 0.0).abs() < 1e-5);
    }

    #[test]
    fn test_ground_resolution_and_discrete_tile() {
        let lat = -6.1754;
        let lon = 106.8272;
        let zoom = 14.0;

        let res = WebMercator::ground_resolution_m_px(lat, zoom);
        assert!(res > 5.0 && res < 15.0, "Resolution at zoom 14 near equator should be ~9.5 m/px: got {}", res);

        let (tx, ty, tz) = WebMercator::get_tile_index(lat, lon, 14);
        assert_eq!(tz, 14);
        assert!(tx > 12000 && tx < 14000);
        assert!(ty > 7000 && ty < 9000);
    }

    #[test]
    fn test_slippy_map_state_pan_zoom_and_pins() {
        let mut map = SlippyMapState::new();
        assert_eq!(map.site_pins.len(), 1);

        let initial_lat = map.center_lat_lon.0;
        map.pan_by_pixels(100.0, 50.0);
        assert_ne!(map.center_lat_lon.0, initial_lat);

        let initial_zoom = map.zoom_level;
        map.zoom_in(1.0);
        assert_eq!(map.zoom_level, initial_zoom + 1.0);

        map.zoom_out(2.0);
        assert_eq!(map.zoom_level, initial_zoom - 1.0);

        let pin_id = map.add_pin(-6.9175, 107.6191, "Bandung Project Lot");
        assert_eq!(map.site_pins.len(), 2);
        assert_eq!(map.active_pin, Some((-6.9175, 107.6191)));

        map.remove_pin(pin_id);
        assert_eq!(map.site_pins.len(), 1);

        let telem = map.telemetry();
        assert_eq!(telem.pins_count, 1);
        assert_eq!(telem.zoom, initial_zoom - 1.0);
    }
}
