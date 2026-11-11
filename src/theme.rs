//! theme.rs — Modern macOS Sequoia / Pro Dark Engineering Visual System.

use egui::{Color32, CornerRadius, Frame, Margin, Stroke};

#[derive(Debug, Clone)]
pub struct MacTheme {
    pub bg_canvas: Color32,
    pub bg_sidebar: Color32,
    pub bg_card: Color32,
    pub bg_card_hover: Color32,
    pub bg_hero: Color32,
    pub accent_blue: Color32,
    pub accent_green: Color32,
    pub accent_orange: Color32,
    pub accent_red: Color32,
    pub text_primary: Color32,
    pub text_secondary: Color32,
    pub text_muted: Color32,
    pub border_subtle: Color32,
    pub border_card: Color32,
}

impl Default for MacTheme {
    fn default() -> Self {
        Self {
            // Deep obsidian & elevated slate glassmorphic surfaces
            bg_canvas: Color32::from_rgb(15, 17, 23),
            bg_sidebar: Color32::from_rgb(20, 23, 31),
            bg_card: Color32::from_rgb(26, 30, 40),
            bg_card_hover: Color32::from_rgb(35, 41, 56),
            bg_hero: Color32::from_rgb(22, 34, 58),
            // High-clarity engineering semantic accents
            accent_blue: Color32::from_rgb(45, 140, 255),
            accent_green: Color32::from_rgb(16, 185, 129),
            accent_orange: Color32::from_rgb(245, 158, 11),
            accent_red: Color32::from_rgb(239, 68, 68),
            // Crisp typographic hierarchy
            text_primary: Color32::from_rgb(248, 250, 252),
            text_secondary: Color32::from_rgb(165, 180, 202),
            text_muted: Color32::from_rgb(108, 122, 145),
            // Subtle architectural hairline borders
            border_subtle: Color32::from_rgb(38, 44, 58),
            border_card: Color32::from_rgb(48, 56, 74),
        }
    }
}

impl MacTheme {
    /// Card frame with modern rounded corners and subtle architectural border
    pub fn card_frame(&self) -> Frame {
        Frame::new()
            .fill(self.bg_card)
            .corner_radius(CornerRadius::same(12))
            .stroke(Stroke::new(1.0, self.border_card))
            .inner_margin(Margin::same(16))
    }

    /// Hero Featured Banner Frame
    pub fn hero_frame(&self) -> Frame {
        Frame::new()
            .fill(self.bg_hero)
            .corner_radius(CornerRadius::same(14))
            .stroke(Stroke::new(1.2, self.accent_blue.gamma_multiply(0.55)))
            .inner_margin(Margin::same(20))
    }

    /// Sidebar Translucent Frame
    pub fn sidebar_frame(&self) -> Frame {
        Frame::new()
            .fill(self.bg_sidebar)
            .stroke(Stroke::new(1.0, self.border_subtle))
            .inner_margin(Margin::symmetric(12, 14))
    }

    /// Full-height Apple macOS Left Navigation Sidebar Frame
    pub fn mac_sidebar_frame(&self) -> Frame {
        Frame::new()
            .fill(self.bg_sidebar)
            .stroke(Stroke::new(1.0, self.border_subtle))
            .inner_margin(Margin::symmetric(12, 12))
    }

    /// Content Top Header Bar Frame (Clean macOS Toolbar)
    pub fn content_top_frame(&self) -> Frame {
        Frame::new()
            .fill(self.bg_sidebar)
            .stroke(Stroke::new(1.0, self.border_subtle))
            .inner_margin(Margin::symmetric(18, 10))
    }

    /// Bottom Status Bar Frame
    pub fn status_bar_frame(&self) -> Frame {
        Frame::new()
            .fill(self.bg_sidebar)
            .stroke(Stroke::new(1.0, self.border_subtle))
            .inner_margin(Margin::symmetric(16, 7))
    }

    /// Top Navigation Bar Frame
    pub fn top_bar_frame(&self) -> Frame {
        Frame::new()
            .fill(self.bg_sidebar)
            .stroke(Stroke::new(1.0, self.border_subtle))
            .inner_margin(Margin::symmetric(16, 10))
    }

    /// Floating Dynamic Island / Pill Frame (Collapsed Capsule)
    pub fn floating_pill_frame(&self) -> Frame {
        Frame::new()
            .fill(Color32::from_rgba_premultiplied(22, 26, 36, 240))
            .corner_radius(CornerRadius::same(22))
            .stroke(Stroke::new(1.2, self.accent_blue.gamma_multiply(0.6)))
            .inner_margin(Margin::symmetric(14, 7))
    }

    /// Floating Dynamic Island Expanded Console Frame
    pub fn floating_island_expanded_frame(&self) -> Frame {
        Frame::new()
            .fill(Color32::from_rgba_premultiplied(18, 22, 31, 250))
            .corner_radius(CornerRadius::same(16))
            .stroke(Stroke::new(1.5, self.accent_blue.gamma_multiply(0.75)))
            .inner_margin(Margin::same(14))
    }

    /// Apply modern Pro Dark visuals to the global egui Context
    pub fn apply_to_ctx(&self, ctx: &egui::Context) {
        let mut visuals = egui::Visuals::dark();
        visuals.panel_fill = self.bg_canvas;
        visuals.window_fill = self.bg_card;
        visuals.extreme_bg_color = Color32::from_rgb(11, 13, 18);
        visuals.faint_bg_color = Color32::from_rgb(22, 26, 35);

        visuals.selection.bg_fill = self.accent_blue.gamma_multiply(0.35);
        visuals.selection.stroke = Stroke::new(1.0, self.accent_blue);

        visuals.widgets.noninteractive.bg_fill = self.bg_card;
        visuals.widgets.noninteractive.weak_bg_fill = self.bg_card;
        visuals.widgets.noninteractive.bg_stroke = Stroke::new(1.0, self.border_subtle);
        visuals.widgets.noninteractive.fg_stroke = Stroke::new(1.0, self.text_secondary);
        visuals.widgets.noninteractive.corner_radius = CornerRadius::same(8);

        visuals.widgets.inactive.bg_fill = Color32::from_rgb(32, 38, 52);
        visuals.widgets.inactive.weak_bg_fill = Color32::from_rgb(28, 33, 45);
        visuals.widgets.inactive.bg_stroke = Stroke::new(1.0, self.border_card);
        visuals.widgets.inactive.fg_stroke = Stroke::new(1.0, self.text_primary);
        visuals.widgets.inactive.corner_radius = CornerRadius::same(8);

        visuals.widgets.hovered.bg_fill = self.bg_card_hover;
        visuals.widgets.hovered.weak_bg_fill = self.bg_card_hover;
        visuals.widgets.hovered.bg_stroke = Stroke::new(1.0, self.accent_blue);
        visuals.widgets.hovered.fg_stroke = Stroke::new(1.2, Color32::WHITE);
        visuals.widgets.hovered.corner_radius = CornerRadius::same(8);

        visuals.widgets.active.bg_fill = self.accent_blue;
        visuals.widgets.active.weak_bg_fill = self.accent_blue;
        visuals.widgets.active.bg_stroke = Stroke::new(1.0, Color32::WHITE);
        visuals.widgets.active.fg_stroke = Stroke::new(1.5, Color32::WHITE);
        visuals.widgets.active.corner_radius = CornerRadius::same(8);

        visuals.widgets.open.bg_fill = self.bg_card_hover;
        visuals.widgets.open.weak_bg_fill = self.bg_card_hover;
        visuals.widgets.open.bg_stroke = Stroke::new(1.0, self.accent_blue);
        visuals.widgets.open.fg_stroke = Stroke::new(1.2, Color32::WHITE);
        visuals.widgets.open.corner_radius = CornerRadius::same(8);

        ctx.set_visuals(visuals);
    }
}
