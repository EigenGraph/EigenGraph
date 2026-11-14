//! theme.rs — Modern macOS Sequoia / Pro Dark & Engineering Daylight Visual System.

use egui::{Color32, CornerRadius, Frame, Margin, Stroke, Vec2};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
pub enum ThemeMode {
    #[default]
    ProDark,
    EngineeringLight,
}

impl ThemeMode {
    pub fn label(self) -> &'static str {
        match self {
            Self::ProDark => "Pro Obsidian Dark",
            Self::EngineeringLight => "Engineering Daylight",
        }
    }

    pub fn app_icon(self) -> crate::icons::AppIcon {
        match self {
            Self::ProDark => crate::icons::AppIcon::Moon,
            Self::EngineeringLight => crate::icons::AppIcon::Sun,
        }
    }
}

#[derive(Debug, Clone)]
pub struct MacTheme {
    pub mode: ThemeMode,
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
        Self::for_mode(ThemeMode::ProDark)
    }
}

impl MacTheme {
    /// Builds a theme palette for the requested [`ThemeMode`].
    pub fn for_mode(mode: ThemeMode) -> Self {
        match mode {
            ThemeMode::ProDark => Self {
                mode,
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
            },
            ThemeMode::EngineeringLight => Self {
                mode,
                // Crisp CAD paper & elevated slate daylight surfaces
                bg_canvas: Color32::from_rgb(244, 246, 250),
                bg_sidebar: Color32::from_rgb(233, 238, 246),
                bg_card: Color32::from_rgb(255, 255, 255),
                bg_card_hover: Color32::from_rgb(226, 233, 245),
                bg_hero: Color32::from_rgb(228, 239, 255),
                // High-contrast engineering accents on light surfaces
                accent_blue: Color32::from_rgb(24, 108, 224),
                accent_green: Color32::from_rgb(10, 146, 100),
                accent_orange: Color32::from_rgb(206, 120, 6),
                accent_red: Color32::from_rgb(212, 42, 42),
                // Deep ink typography
                text_primary: Color32::from_rgb(15, 23, 42),
                text_secondary: Color32::from_rgb(51, 65, 85),
                text_muted: Color32::from_rgb(100, 116, 139),
                // Clean technical drafting hairlines
                border_subtle: Color32::from_rgb(203, 213, 225),
                border_card: Color32::from_rgb(184, 196, 212),
            },
        }
    }

    pub fn is_dark(&self) -> bool {
        self.mode == ThemeMode::ProDark
    }
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
        let fill = if self.is_dark() {
            Color32::from_rgba_premultiplied(22, 26, 36, 240)
        } else {
            Color32::from_rgba_premultiplied(248, 250, 252, 245)
        };
        Frame::new()
            .fill(fill)
            .corner_radius(CornerRadius::same(22))
            .stroke(Stroke::new(1.2, self.accent_blue.gamma_multiply(0.6)))
            .inner_margin(Margin::symmetric(14, 7))
    }

    /// Floating Dynamic Island Expanded Console Frame
    pub fn floating_island_expanded_frame(&self) -> Frame {
        let fill = if self.is_dark() {
            Color32::from_rgba_premultiplied(18, 22, 31, 250)
        } else {
            Color32::from_rgba_premultiplied(255, 255, 255, 252)
        };
        Frame::new()
            .fill(fill)
            .corner_radius(CornerRadius::same(16))
            .stroke(Stroke::new(1.5, self.accent_blue.gamma_multiply(0.75)))
            .inner_margin(Margin::same(14))
    }

    /// Apply modern Pro Dark or Engineering Daylight visuals to the global egui Context
    pub fn apply_to_ctx(&self, ctx: &egui::Context) {
        let is_dark = self.is_dark();
        let mut visuals = if is_dark {
            egui::Visuals::dark()
        } else {
            egui::Visuals::light()
        };
        visuals.panel_fill = self.bg_canvas;
        visuals.window_fill = self.bg_card;
        visuals.extreme_bg_color = if is_dark {
            Color32::from_rgb(11, 13, 18)
        } else {
            Color32::from_rgb(232, 236, 244)
        };
        visuals.faint_bg_color = if is_dark {
            Color32::from_rgb(22, 26, 35)
        } else {
            Color32::from_rgb(238, 242, 248)
        };

        visuals.selection.bg_fill = self.accent_blue.gamma_multiply(0.30);
        visuals.selection.stroke = Stroke::new(1.0, self.accent_blue);

        visuals.widgets.noninteractive.bg_fill = self.bg_card;
        visuals.widgets.noninteractive.weak_bg_fill = self.bg_card;
        visuals.widgets.noninteractive.bg_stroke = Stroke::new(1.0, self.border_subtle);
        visuals.widgets.noninteractive.fg_stroke = Stroke::new(1.0, self.text_secondary);
        visuals.widgets.noninteractive.corner_radius = CornerRadius::same(8);

        visuals.widgets.inactive.bg_fill = if is_dark {
            Color32::from_rgb(32, 38, 52)
        } else {
            Color32::from_rgb(226, 232, 240)
        };
        visuals.widgets.inactive.weak_bg_fill = if is_dark {
            Color32::from_rgb(28, 33, 45)
        } else {
            Color32::from_rgb(234, 239, 246)
        };
        visuals.widgets.inactive.bg_stroke = Stroke::new(1.0, self.border_card);
        visuals.widgets.inactive.fg_stroke = Stroke::new(1.0, self.text_primary);
        visuals.widgets.inactive.corner_radius = CornerRadius::same(8);

        let hover_fg = if is_dark {
            Color32::WHITE
        } else {
            self.text_primary
        };
        visuals.widgets.hovered.bg_fill = self.bg_card_hover;
        visuals.widgets.hovered.weak_bg_fill = self.bg_card_hover;
        visuals.widgets.hovered.bg_stroke = Stroke::new(1.0, self.accent_blue);
        visuals.widgets.hovered.fg_stroke = Stroke::new(1.2, hover_fg);
        visuals.widgets.hovered.corner_radius = CornerRadius::same(8);

        visuals.widgets.active.bg_fill = self.accent_blue;
        visuals.widgets.active.weak_bg_fill = self.accent_blue;
        visuals.widgets.active.bg_stroke = Stroke::new(1.0, Color32::WHITE);
        visuals.widgets.active.fg_stroke = Stroke::new(1.5, Color32::WHITE);
        visuals.widgets.active.corner_radius = CornerRadius::same(8);

        visuals.widgets.open.bg_fill = self.bg_card_hover;
        visuals.widgets.open.weak_bg_fill = self.bg_card_hover;
        visuals.widgets.open.bg_stroke = Stroke::new(1.0, self.accent_blue);
        visuals.widgets.open.fg_stroke = Stroke::new(1.2, hover_fg);
        visuals.widgets.open.corner_radius = CornerRadius::same(8);

        ctx.set_visuals(visuals);
    }

    /// Viewport width threshold below which phone-first compact stacking applies.
    pub const MOBILE_BREAKPOINT: f32 = 640.0;

    /// Viewport width threshold below which the sidebar collapses into an overlay drawer.
    pub const COMPACT_BREAKPOINT: f32 = 960.0;

    /// Returns the safe content viewport size, clamped against transient zero-sized frames.
    pub fn viewport_size(ctx: &egui::Context) -> Vec2 {
        ctx.content_rect().size().max(Vec2::new(280.0, 320.0))
    }

    /// Returns `true` on narrow mobile viewports (`width < 640px`).
    pub fn is_mobile(ctx: &egui::Context) -> bool {
        Self::viewport_size(ctx).x < Self::MOBILE_BREAKPOINT
    }

    /// Returns `true` on mobile and portrait/narrow tablet viewports (`width < 960px`).
    pub fn is_compact(ctx: &egui::Context) -> bool {
        Self::viewport_size(ctx).x < Self::COMPACT_BREAKPOINT
    }

    /// Clamps a desired modal/window size against an explicit viewport dimension.
    pub fn clamp_modal_for_viewport(vp: Vec2, desired: Vec2) -> Vec2 {
        let safe_vp = vp.max(Vec2::new(280.0, 320.0));
        Vec2::new(
            (safe_vp.x - 24.0).clamp(240.0, desired.x.max(240.0)),
            (safe_vp.y - 48.0).clamp(240.0, desired.y.max(240.0)),
        )
    }

    /// Clamps a desired modal/window size so it always fits inside the current viewport.
    pub fn clamp_modal_size(ctx: &egui::Context, desired: Vec2) -> Vec2 {
        Self::clamp_modal_for_viewport(Self::viewport_size(ctx), desired)
    }

    /// Maximum modal size bounded cleanly inside the active viewport.
    pub fn modal_max_size(ctx: &egui::Context) -> Vec2 {
        let vp = Self::viewport_size(ctx);
        Vec2::new((vp.x - 16.0).max(260.0), (vp.y - 32.0).max(240.0))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_modal_size_clamps_for_mobile_tablet_and_desktop() {
        let desired = Vec2::new(620.0, 560.0);

        // Mobile viewport (375x667) must clamp horizontally below 375px
        let mobile = MacTheme::clamp_modal_for_viewport(Vec2::new(375.0, 667.0), desired);
        assert_eq!(mobile, Vec2::new(351.0, 560.0));

        // Ultra-compact landscape phone (320x300) still clamps safely
        let small = MacTheme::clamp_modal_for_viewport(Vec2::new(320.0, 300.0), desired);
        assert_eq!(small, Vec2::new(296.0, 272.0));

        // Desktop (1440x900) keeps desired size untouched
        let desktop = MacTheme::clamp_modal_for_viewport(Vec2::new(1440.0, 900.0), desired);
        assert_eq!(desktop, desired);
    }
}
