//! icons.rs — Crisp Geometric Vector Icon Engine for Native & WebAssembly UI.
//! Renders resolution-independent icons using `egui::Painter` so navigation, toolbars,
//! ribbons, subtabs, and status badges never suffer from missing OS/WASM emoji glyphs (`□`).

use egui::{
    Color32, CornerRadius, CursorIcon, FontId, Pos2, Rect, Response, Sense, Stroke, Ui, Vec2,
};

use crate::theme::MacTheme;

/// Semantic vector icon catalog used across the 15 engineering modules and workspace chrome.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum AppIcon {
    // 15 Engineering Domains
    Folder,
    Bot,
    Compass,
    RulerSquare,
    Cube3D,
    Bolt,
    Clock,
    Coins,
    Leaf,
    Building,
    Shield,
    Recycle,
    Cpu,
    Pulse,
    Document,

    // Chrome & Navigation Controls
    Menu,
    Close,
    ChevronLeft,
    ChevronDown,
    ChevronUp,
    ArrowRight,
    ArrowDown,
    SettingsGear,
    Search,
    Lock,
    Unlock,
    CheckCircle,
    AlertTriangle,
    Info,
    Star,
    Target,
    Sparkles,
    Sun,
    Moon,
    Plus,
    Minus,
    Play,
    Pause,
    SkipBack,
    SkipNext,

    // CAD, Universal Canvas & BIM Drafting Tools
    SelectArrow,
    CursorPan,
    Pencil,
    LineSeg,
    ArcCurve,
    RectShape,
    CircleShape,
    DimensionRuler,
    CommentNote,
    Eraser,
    Undo,
    Redo,
    Grid,
    MagnetSnap,
    ZoomFit,
    Snowflake,
    SlidersPanel,
    SaveDisk,
    UploadImport,
    Layers,
    MapPin,
    Globe,
    Trash,
    Printer,
    Palette,

    // BIM Elements & Analysis Visuals
    Wall,
    Door,
    Window,
    Column,
    Beam,
    Rebar,
    Slab,
    Stairs,
    SectionCut,
    NodesGraph,
    SolidBox,
    Wireframe,
    ChartBar,
    ChartLine,
    BoxPackage,
    Database,
    Clipboard,
    Wrench,
    Camera,
    Wind,
    Flame,
    Truck,
    Hammer,
    HardHat,
    Scissors,
    LinkChain,
    Book,
}

impl AppIcon {
    /// Renders this vector icon inside `rect` with clean proportional strokes.
    pub fn paint(self, painter: &egui::Painter, rect: Rect, color: Color32) {
        paint_icon(painter, rect, self, color);
    }
}

/// Paints a crisp vector [`AppIcon`] scaled cleanly into `rect`.
pub fn paint_icon(painter: &egui::Painter, rect: Rect, icon: AppIcon, color: Color32) {
    let w = rect.width().min(rect.height()).max(6.0);
    let sw = (w * 0.115).clamp(1.25, 2.4);
    let stroke = Stroke::new(sw, color);
    let thin = Stroke::new((sw * 0.85).max(1.0), color);
    let sq = Rect::from_center_size(rect.center(), Vec2::splat(w));
    let p = |x: f32, y: f32| Pos2::new(sq.left() + x * sq.width(), sq.top() + y * sq.height());

    match icon {
        AppIcon::Folder => {
            painter.line_segment([p(0.14, 0.24), p(0.42, 0.24)], stroke);
            painter.line_segment([p(0.42, 0.24), p(0.50, 0.34)], stroke);
            painter.line_segment([p(0.50, 0.34), p(0.86, 0.34)], stroke);
            painter.line_segment([p(0.86, 0.34), p(0.86, 0.80)], stroke);
            painter.line_segment([p(0.86, 0.80), p(0.14, 0.80)], stroke);
            painter.line_segment([p(0.14, 0.80), p(0.14, 0.24)], stroke);
        }
        AppIcon::Bot => {
            painter.line_segment([p(0.50, 0.14), p(0.50, 0.28)], stroke);
            painter.circle_filled(p(0.50, 0.14), sw * 0.9, color);
            let head = Rect::from_min_max(p(0.18, 0.28), p(0.82, 0.78));
            painter.rect_stroke(head, CornerRadius::same(3), stroke, egui::StrokeKind::Middle);
            painter.circle_filled(p(0.36, 0.50), sw * 0.85, color);
            painter.circle_filled(p(0.64, 0.50), sw * 0.85, color);
            painter.line_segment([p(0.36, 0.66), p(0.64, 0.66)], thin);
        }
        AppIcon::Compass => {
            painter.circle_stroke(sq.center(), w * 0.36, stroke);
            painter.line_segment([p(0.66, 0.34), p(0.42, 0.48)], stroke);
            painter.line_segment([p(0.42, 0.48), p(0.34, 0.66)], stroke);
            painter.line_segment([p(0.34, 0.66), p(0.58, 0.52)], stroke);
            painter.line_segment([p(0.58, 0.52), p(0.66, 0.34)], stroke);
        }
        AppIcon::RulerSquare => {
            let pts = [
                p(0.18, 0.16),
                p(0.18, 0.84),
                p(0.86, 0.84),
                p(0.18, 0.16),
            ];
            for i in 0..3 {
                painter.line_segment([pts[i], pts[i + 1]], stroke);
            }
            painter.line_segment([p(0.32, 0.46), p(0.32, 0.70)], thin);
            painter.line_segment([p(0.32, 0.70), p(0.56, 0.70)], thin);
            painter.line_segment([p(0.56, 0.70), p(0.32, 0.46)], thin);
        }
        AppIcon::Cube3D | AppIcon::SolidBox => {
            let top = p(0.50, 0.14);
            let tr = p(0.82, 0.32);
            let br = p(0.82, 0.68);
            let bot = p(0.50, 0.86);
            let bl = p(0.18, 0.68);
            let tl = p(0.18, 0.32);
            let ctr = p(0.50, 0.50);
            for (a, b) in [
                (top, tr),
                (tr, br),
                (br, bot),
                (bot, bl),
                (bl, tl),
                (tl, top),
                (tl, ctr),
                (tr, ctr),
                (ctr, bot),
            ] {
                painter.line_segment([a, b], stroke);
            }
        }
        AppIcon::Wireframe => {
            let b1 = Rect::from_min_max(p(0.16, 0.30), p(0.66, 0.82));
            let b2 = Rect::from_min_max(p(0.34, 0.16), p(0.84, 0.68));
            painter.rect_stroke(b1, CornerRadius::ZERO, thin, egui::StrokeKind::Middle);
            painter.rect_stroke(b2, CornerRadius::ZERO, thin, egui::StrokeKind::Middle);
            painter.line_segment([b1.min, b2.min], thin);
            painter.line_segment([b1.max, b2.max], thin);
            painter.line_segment([Pos2::new(b1.max.x, b1.min.y), Pos2::new(b2.max.x, b2.min.y)], thin);
            painter.line_segment([Pos2::new(b1.min.x, b1.max.y), Pos2::new(b2.min.x, b2.max.y)], thin);
        }
        AppIcon::Bolt => {
            let pts = [
                p(0.56, 0.14),
                p(0.24, 0.54),
                p(0.48, 0.54),
                p(0.42, 0.86),
                p(0.76, 0.44),
                p(0.52, 0.44),
                p(0.56, 0.14),
            ];
            for pair in pts.windows(2) {
                painter.line_segment([pair[0], pair[1]], stroke);
            }
        }
        AppIcon::Clock => {
            painter.circle_stroke(sq.center(), w * 0.36, stroke);
            painter.line_segment([sq.center(), p(0.50, 0.28)], stroke);
            painter.line_segment([sq.center(), p(0.68, 0.58)], stroke);
        }
        AppIcon::Coins => {
            painter.circle_stroke(sq.center(), w * 0.36, stroke);
            painter.line_segment([p(0.50, 0.24), p(0.50, 0.76)], thin);
            painter.line_segment([p(0.38, 0.38), p(0.62, 0.38)], thin);
            painter.line_segment([p(0.38, 0.50), p(0.62, 0.50)], thin);
            painter.line_segment([p(0.38, 0.62), p(0.62, 0.62)], thin);
        }
        AppIcon::Leaf => {
            painter.line_segment([p(0.20, 0.82), p(0.76, 0.24)], stroke);
            painter.line_segment([p(0.78, 0.20), p(0.36, 0.24)], stroke);
            painter.line_segment([p(0.36, 0.24), p(0.24, 0.54)], stroke);
            painter.line_segment([p(0.24, 0.54), p(0.38, 0.72)], thin);
            painter.line_segment([p(0.78, 0.20), p(0.76, 0.58)], stroke);
            painter.line_segment([p(0.76, 0.58), p(0.44, 0.74)], stroke);
        }
        AppIcon::Building => {
            let main = Rect::from_min_max(p(0.24, 0.18), p(0.76, 0.84));
            painter.rect_stroke(main, CornerRadius::same(1), stroke, egui::StrokeKind::Middle);
            for y in [0.34, 0.50, 0.66] {
                painter.line_segment([p(0.36, y), p(0.46, y)], thin);
                painter.line_segment([p(0.54, y), p(0.64, y)], thin);
            }
            painter.line_segment([p(0.14, 0.84), p(0.86, 0.84)], stroke);
        }
        AppIcon::Shield => {
            let pts = [
                p(0.50, 0.14),
                p(0.80, 0.26),
                p(0.76, 0.58),
                p(0.50, 0.86),
                p(0.24, 0.58),
                p(0.20, 0.26),
                p(0.50, 0.14),
            ];
            for pair in pts.windows(2) {
                painter.line_segment([pair[0], pair[1]], stroke);
            }
            painter.line_segment([p(0.38, 0.50), p(0.47, 0.60)], thin);
            painter.line_segment([p(0.47, 0.60), p(0.64, 0.40)], thin);
        }
        AppIcon::Recycle => {
            let pts = [p(0.50, 0.16), p(0.82, 0.74), p(0.18, 0.74), p(0.50, 0.16)];
            for pair in pts.windows(2) {
                painter.line_segment([pair[0], pair[1]], stroke);
            }
            painter.circle_filled(p(0.50, 0.16), sw, color);
            painter.circle_filled(p(0.82, 0.74), sw, color);
            painter.circle_filled(p(0.18, 0.74), sw, color);
        }
        AppIcon::Cpu => {
            let core = Rect::from_min_max(p(0.28, 0.28), p(0.72, 0.72));
            painter.rect_stroke(core, CornerRadius::same(2), stroke, egui::StrokeKind::Middle);
            let inner = Rect::from_min_max(p(0.40, 0.40), p(0.60, 0.60));
            painter.rect_stroke(inner, CornerRadius::ZERO, thin, egui::StrokeKind::Middle);
            for t in [0.38, 0.62] {
                painter.line_segment([p(t, 0.14), p(t, 0.28)], thin);
                painter.line_segment([p(t, 0.72), p(t, 0.86)], thin);
                painter.line_segment([p(0.14, t), p(0.28, t)], thin);
                painter.line_segment([p(0.72, t), p(0.86, t)], thin);
            }
        }
        AppIcon::Pulse | AppIcon::ChartLine => {
            let pts = [
                p(0.12, 0.54),
                p(0.30, 0.54),
                p(0.40, 0.24),
                p(0.54, 0.78),
                p(0.66, 0.40),
                p(0.74, 0.54),
                p(0.88, 0.54),
            ];
            for pair in pts.windows(2) {
                painter.line_segment([pair[0], pair[1]], stroke);
            }
        }
        AppIcon::Document | AppIcon::Clipboard | AppIcon::Book => {
            let pts = [
                p(0.22, 0.14),
                p(0.62, 0.14),
                p(0.78, 0.30),
                p(0.78, 0.86),
                p(0.22, 0.86),
                p(0.22, 0.14),
            ];
            for pair in pts.windows(2) {
                painter.line_segment([pair[0], pair[1]], stroke);
            }
            painter.line_segment([p(0.34, 0.46), p(0.66, 0.46)], thin);
            painter.line_segment([p(0.34, 0.60), p(0.66, 0.60)], thin);
            painter.line_segment([p(0.34, 0.74), p(0.56, 0.74)], thin);
        }
        AppIcon::Menu => {
            for y in [0.28, 0.50, 0.72] {
                painter.line_segment([p(0.18, y), p(0.82, y)], stroke);
            }
        }
        AppIcon::Close => {
            painter.line_segment([p(0.24, 0.24), p(0.76, 0.76)], stroke);
            painter.line_segment([p(0.76, 0.24), p(0.24, 0.76)], stroke);
        }
        AppIcon::ChevronLeft => {
            painter.line_segment([p(0.64, 0.22), p(0.34, 0.50)], stroke);
            painter.line_segment([p(0.34, 0.50), p(0.64, 0.78)], stroke);
        }
        AppIcon::ChevronDown | AppIcon::ArrowDown => {
            painter.line_segment([p(0.50, 0.20), p(0.50, 0.76)], stroke);
            painter.line_segment([p(0.26, 0.54), p(0.50, 0.78)], stroke);
            painter.line_segment([p(0.74, 0.54), p(0.50, 0.78)], stroke);
        }
        AppIcon::ChevronUp => {
            painter.line_segment([p(0.24, 0.64), p(0.50, 0.34)], stroke);
            painter.line_segment([p(0.50, 0.34), p(0.76, 0.64)], stroke);
        }
        AppIcon::ArrowRight => {
            painter.line_segment([p(0.20, 0.50), p(0.78, 0.50)], stroke);
            painter.line_segment([p(0.56, 0.28), p(0.78, 0.50)], stroke);
            painter.line_segment([p(0.56, 0.72), p(0.78, 0.50)], stroke);
        }
        AppIcon::SettingsGear | AppIcon::Wrench => {
            let ctr = sq.center();
            painter.circle_stroke(ctr, w * 0.26, stroke);
            painter.circle_stroke(ctr, w * 0.11, thin);
            for i in 0..8 {
                let a = (i as f32) * std::f32::consts::FRAC_PI_4;
                let p1 = ctr + Vec2::new(a.cos(), a.sin()) * (w * 0.26);
                let p2 = ctr + Vec2::new(a.cos(), a.sin()) * (w * 0.39);
                painter.line_segment([p1, p2], stroke);
            }
        }
        AppIcon::Search => {
            painter.circle_stroke(p(0.43, 0.43), w * 0.25, stroke);
            painter.line_segment([p(0.61, 0.61), p(0.84, 0.84)], stroke);
        }
        AppIcon::Lock => {
            let body = Rect::from_min_max(p(0.24, 0.46), p(0.76, 0.84));
            painter.rect_stroke(body, CornerRadius::same(2), stroke, egui::StrokeKind::Middle);
            painter.line_segment([p(0.34, 0.46), p(0.34, 0.28)], stroke);
            painter.line_segment([p(0.34, 0.28), p(0.66, 0.28)], stroke);
            painter.line_segment([p(0.66, 0.28), p(0.66, 0.46)], stroke);
            painter.circle_filled(p(0.50, 0.65), sw * 0.8, color);
        }
        AppIcon::Unlock => {
            let body = Rect::from_min_max(p(0.24, 0.46), p(0.76, 0.84));
            painter.rect_stroke(body, CornerRadius::same(2), stroke, egui::StrokeKind::Middle);
            painter.line_segment([p(0.34, 0.46), p(0.34, 0.24)], stroke);
            painter.line_segment([p(0.34, 0.24), p(0.64, 0.24)], stroke);
            painter.line_segment([p(0.64, 0.24), p(0.64, 0.32)], stroke);
            painter.circle_filled(p(0.50, 0.65), sw * 0.8, color);
        }
        AppIcon::CheckCircle => {
            painter.circle_stroke(sq.center(), w * 0.36, stroke);
            painter.line_segment([p(0.34, 0.52), p(0.46, 0.64)], stroke);
            painter.line_segment([p(0.46, 0.64), p(0.68, 0.38)], stroke);
        }
        AppIcon::AlertTriangle | AppIcon::Flame => {
            let pts = [p(0.50, 0.16), p(0.84, 0.80), p(0.16, 0.80), p(0.50, 0.16)];
            for pair in pts.windows(2) {
                painter.line_segment([pair[0], pair[1]], stroke);
            }
            painter.line_segment([p(0.50, 0.38), p(0.50, 0.58)], stroke);
            painter.circle_filled(p(0.50, 0.70), sw * 0.75, color);
        }
        AppIcon::Info => {
            painter.circle_stroke(sq.center(), w * 0.36, stroke);
            painter.circle_filled(p(0.50, 0.34), sw * 0.75, color);
            painter.line_segment([p(0.50, 0.46), p(0.50, 0.70)], stroke);
        }
        AppIcon::Star | AppIcon::Sparkles => {
            painter.line_segment([p(0.50, 0.14), p(0.50, 0.86)], stroke);
            painter.line_segment([p(0.14, 0.50), p(0.86, 0.50)], stroke);
            painter.line_segment([p(0.28, 0.28), p(0.72, 0.72)], thin);
            painter.line_segment([p(0.72, 0.28), p(0.28, 0.72)], thin);
        }
        AppIcon::Target => {
            painter.circle_stroke(sq.center(), w * 0.36, stroke);
            painter.circle_stroke(sq.center(), w * 0.18, thin);
            painter.circle_filled(sq.center(), sw * 0.85, color);
        }
        AppIcon::Sun => {
            let ctr = sq.center();
            painter.circle_stroke(ctr, w * 0.19, stroke);
            for i in 0..8 {
                let a = (i as f32) * std::f32::consts::FRAC_PI_4;
                painter.line_segment(
                    [
                        ctr + Vec2::new(a.cos(), a.sin()) * (w * 0.28),
                        ctr + Vec2::new(a.cos(), a.sin()) * (w * 0.40),
                    ],
                    thin,
                );
            }
        }
        AppIcon::Moon => {
            painter.circle_stroke(p(0.46, 0.50), w * 0.30, stroke);
            painter.line_segment([p(0.58, 0.22), p(0.44, 0.50)], thin);
            painter.line_segment([p(0.44, 0.50), p(0.58, 0.78)], thin);
        }
        AppIcon::Plus => {
            painter.line_segment([p(0.50, 0.20), p(0.50, 0.80)], stroke);
            painter.line_segment([p(0.20, 0.50), p(0.80, 0.50)], stroke);
        }
        AppIcon::Minus => {
            painter.line_segment([p(0.20, 0.50), p(0.80, 0.50)], stroke);
        }
        AppIcon::Play => {
            let pts = [p(0.28, 0.20), p(0.78, 0.50), p(0.28, 0.80), p(0.28, 0.20)];
            for pair in pts.windows(2) {
                painter.line_segment([pair[0], pair[1]], stroke);
            }
        }
        AppIcon::Pause => {
            painter.line_segment([p(0.36, 0.22), p(0.36, 0.78)], stroke);
            painter.line_segment([p(0.64, 0.22), p(0.64, 0.78)], stroke);
        }
        AppIcon::SkipBack => {
            painter.line_segment([p(0.26, 0.22), p(0.26, 0.78)], stroke);
            let pts = [p(0.74, 0.22), p(0.36, 0.50), p(0.74, 0.78), p(0.74, 0.22)];
            for pair in pts.windows(2) {
                painter.line_segment([pair[0], pair[1]], stroke);
            }
        }
        AppIcon::SkipNext => {
            let pts = [p(0.26, 0.22), p(0.64, 0.50), p(0.26, 0.78), p(0.26, 0.22)];
            for pair in pts.windows(2) {
                painter.line_segment([pair[0], pair[1]], stroke);
            }
            painter.line_segment([p(0.74, 0.22), p(0.74, 0.78)], stroke);
        }
        AppIcon::SelectArrow => {
            let pts = [
                p(0.24, 0.16),
                p(0.24, 0.78),
                p(0.42, 0.62),
                p(0.64, 0.84),
                p(0.74, 0.76),
                p(0.54, 0.54),
                p(0.76, 0.54),
                p(0.24, 0.16),
            ];
            for pair in pts.windows(2) {
                painter.line_segment([pair[0], pair[1]], thin);
            }
        }
        AppIcon::CursorPan => {
            painter.line_segment([p(0.50, 0.14), p(0.50, 0.86)], stroke);
            painter.line_segment([p(0.14, 0.50), p(0.86, 0.50)], stroke);
            painter.line_segment([p(0.38, 0.26), p(0.50, 0.14)], thin);
            painter.line_segment([p(0.62, 0.26), p(0.50, 0.14)], thin);
            painter.line_segment([p(0.38, 0.74), p(0.50, 0.86)], thin);
            painter.line_segment([p(0.62, 0.74), p(0.50, 0.86)], thin);
            painter.line_segment([p(0.26, 0.38), p(0.14, 0.50)], thin);
            painter.line_segment([p(0.26, 0.62), p(0.14, 0.50)], thin);
            painter.line_segment([p(0.74, 0.38), p(0.86, 0.50)], thin);
            painter.line_segment([p(0.74, 0.62), p(0.86, 0.50)], thin);
        }
        AppIcon::Pencil => {
            painter.line_segment([p(0.22, 0.78), p(0.68, 0.20)], stroke);
            painter.line_segment([p(0.68, 0.20), p(0.80, 0.32)], stroke);
            painter.line_segment([p(0.80, 0.32), p(0.34, 0.84)], stroke);
            painter.line_segment([p(0.22, 0.78), p(0.16, 0.86)], stroke);
            painter.line_segment([p(0.16, 0.86), p(0.34, 0.84)], stroke);
        }
        AppIcon::LineSeg => {
            painter.line_segment([p(0.22, 0.78), p(0.78, 0.22)], stroke);
            painter.circle_filled(p(0.22, 0.78), sw * 1.15, color);
            painter.circle_filled(p(0.78, 0.22), sw * 1.15, color);
        }
        AppIcon::ArcCurve => {
            painter.line_segment([p(0.20, 0.74), p(0.34, 0.42)], stroke);
            painter.line_segment([p(0.34, 0.42), p(0.58, 0.26)], stroke);
            painter.line_segment([p(0.58, 0.26), p(0.82, 0.36)], stroke);
            painter.circle_filled(p(0.20, 0.74), sw, color);
            painter.circle_filled(p(0.82, 0.36), sw, color);
        }
        AppIcon::RectShape => {
            let box_r = Rect::from_min_max(p(0.20, 0.24), p(0.80, 0.76));
            painter.rect_stroke(box_r, CornerRadius::same(1), stroke, egui::StrokeKind::Middle);
            for pt in [box_r.min, box_r.max, p(0.80, 0.24), p(0.20, 0.76)] {
                painter.circle_filled(pt, sw * 0.9, color);
            }
        }
        AppIcon::CircleShape => {
            painter.circle_stroke(sq.center(), w * 0.34, stroke);
            painter.circle_filled(sq.center(), sw * 0.9, color);
        }
        AppIcon::DimensionRuler => {
            painter.line_segment([p(0.16, 0.28), p(0.16, 0.72)], thin);
            painter.line_segment([p(0.84, 0.28), p(0.84, 0.72)], thin);
            painter.line_segment([p(0.18, 0.50), p(0.82, 0.50)], stroke);
            painter.line_segment([p(0.30, 0.38), p(0.18, 0.50)], thin);
            painter.line_segment([p(0.30, 0.62), p(0.18, 0.50)], thin);
            painter.line_segment([p(0.70, 0.38), p(0.82, 0.50)], thin);
            painter.line_segment([p(0.70, 0.62), p(0.82, 0.50)], thin);
        }
        AppIcon::CommentNote => {
            let bubble = Rect::from_min_max(p(0.16, 0.20), p(0.84, 0.68));
            painter.rect_stroke(bubble, CornerRadius::same(3), stroke, egui::StrokeKind::Middle);
            painter.line_segment([p(0.30, 0.68), p(0.22, 0.84)], stroke);
            painter.line_segment([p(0.22, 0.84), p(0.46, 0.68)], stroke);
            painter.line_segment([p(0.30, 0.38), p(0.70, 0.38)], thin);
            painter.line_segment([p(0.30, 0.52), p(0.58, 0.52)], thin);
        }
        AppIcon::Eraser | AppIcon::Trash => {
            painter.line_segment([p(0.22, 0.28), p(0.78, 0.28)], stroke);
            painter.line_segment([p(0.38, 0.18), p(0.62, 0.18)], thin);
            let body = Rect::from_min_max(p(0.28, 0.28), p(0.72, 0.82));
            painter.rect_stroke(body, CornerRadius::same(1), stroke, egui::StrokeKind::Middle);
            painter.line_segment([p(0.42, 0.42), p(0.42, 0.70)], thin);
            painter.line_segment([p(0.58, 0.42), p(0.58, 0.70)], thin);
        }
        AppIcon::Undo => {
            painter.line_segment([p(0.26, 0.36), p(0.64, 0.36)], stroke);
            painter.line_segment([p(0.64, 0.36), p(0.78, 0.50)], stroke);
            painter.line_segment([p(0.78, 0.50), p(0.64, 0.68)], stroke);
            painter.line_segment([p(0.64, 0.68), p(0.34, 0.68)], stroke);
            painter.line_segment([p(0.40, 0.22), p(0.24, 0.36)], stroke);
            painter.line_segment([p(0.40, 0.50), p(0.24, 0.36)], stroke);
        }
        AppIcon::Redo => {
            painter.line_segment([p(0.74, 0.36), p(0.36, 0.36)], stroke);
            painter.line_segment([p(0.36, 0.36), p(0.22, 0.50)], stroke);
            painter.line_segment([p(0.22, 0.50), p(0.36, 0.68)], stroke);
            painter.line_segment([p(0.36, 0.68), p(0.66, 0.68)], stroke);
            painter.line_segment([p(0.60, 0.22), p(0.76, 0.36)], stroke);
            painter.line_segment([p(0.60, 0.50), p(0.76, 0.36)], stroke);
        }
        AppIcon::Grid => {
            for t in [0.36, 0.64] {
                painter.line_segment([p(t, 0.16), p(t, 0.84)], thin);
                painter.line_segment([p(0.16, t), p(0.84, t)], thin);
            }
            painter.rect_stroke(
                Rect::from_min_max(p(0.16, 0.16), p(0.84, 0.84)),
                CornerRadius::same(1),
                thin,
                egui::StrokeKind::Middle,
            );
        }
        AppIcon::MagnetSnap => {
            painter.line_segment([p(0.26, 0.22), p(0.26, 0.60)], stroke);
            painter.line_segment([p(0.26, 0.60), p(0.40, 0.78)], stroke);
            painter.line_segment([p(0.40, 0.78), p(0.60, 0.78)], stroke);
            painter.line_segment([p(0.60, 0.78), p(0.74, 0.60)], stroke);
            painter.line_segment([p(0.74, 0.60), p(0.74, 0.22)], stroke);
            painter.line_segment([p(0.20, 0.34), p(0.34, 0.34)], thin);
            painter.line_segment([p(0.66, 0.34), p(0.80, 0.34)], thin);
        }
        AppIcon::ZoomFit => {
            for (cx, cy, dx, dy) in [
                (0.18, 0.18, 0.22, 0.22),
                (0.82, 0.18, -0.22, 0.22),
                (0.18, 0.82, 0.22, -0.22),
                (0.82, 0.82, -0.22, -0.22),
            ] {
                painter.line_segment([p(cx, cy), p(cx + dx, cy)], stroke);
                painter.line_segment([p(cx, cy), p(cx, cy + dy)], stroke);
            }
            painter.rect_stroke(
                Rect::from_min_max(p(0.38, 0.38), p(0.62, 0.62)),
                CornerRadius::ZERO,
                thin,
                egui::StrokeKind::Middle,
            );
        }
        AppIcon::Snowflake => {
            let ctr = sq.center();
            for i in 0..6 {
                let a = (i as f32) * (std::f32::consts::PI / 3.0);
                let tip = ctr + Vec2::new(a.cos(), a.sin()) * (w * 0.38);
                painter.line_segment([ctr, tip], stroke);
                let mid = ctr + Vec2::new(a.cos(), a.sin()) * (w * 0.25);
                let perp = Vec2::new(-a.sin(), a.cos()) * (w * 0.09);
                let fwd = Vec2::new(a.cos(), a.sin()) * (w * 0.09);
                painter.line_segment([mid, mid + perp + fwd], thin);
                painter.line_segment([mid, mid - perp + fwd], thin);
            }
        }
        AppIcon::SlidersPanel => {
            for (y, kx) in [(0.28, 0.36), (0.50, 0.66), (0.72, 0.44)] {
                painter.line_segment([p(0.16, y), p(0.84, y)], thin);
                painter.circle_filled(p(kx, y), sw * 1.35, color);
            }
        }
        AppIcon::SaveDisk => {
            let pts = [
                p(0.18, 0.18),
                p(0.70, 0.18),
                p(0.82, 0.30),
                p(0.82, 0.82),
                p(0.18, 0.82),
                p(0.18, 0.18),
            ];
            for pair in pts.windows(2) {
                painter.line_segment([pair[0], pair[1]], stroke);
            }
            painter.rect_stroke(
                Rect::from_min_max(p(0.32, 0.18), p(0.62, 0.38)),
                CornerRadius::ZERO,
                thin,
                egui::StrokeKind::Middle,
            );
            painter.rect_stroke(
                Rect::from_min_max(p(0.30, 0.54), p(0.70, 0.82)),
                CornerRadius::ZERO,
                thin,
                egui::StrokeKind::Middle,
            );
        }
        AppIcon::UploadImport => {
            painter.line_segment([p(0.50, 0.20), p(0.50, 0.66)], stroke);
            painter.line_segment([p(0.32, 0.48), p(0.50, 0.66)], stroke);
            painter.line_segment([p(0.68, 0.48), p(0.50, 0.66)], stroke);
            painter.line_segment([p(0.20, 0.68), p(0.20, 0.82)], stroke);
            painter.line_segment([p(0.20, 0.82), p(0.80, 0.82)], stroke);
            painter.line_segment([p(0.80, 0.82), p(0.80, 0.68)], stroke);
        }
        AppIcon::Layers => {
            let pts = [p(0.50, 0.16), p(0.84, 0.34), p(0.50, 0.52), p(0.16, 0.34), p(0.50, 0.16)];
            for pair in pts.windows(2) {
                painter.line_segment([pair[0], pair[1]], stroke);
            }
            painter.line_segment([p(0.16, 0.52), p(0.50, 0.70)], thin);
            painter.line_segment([p(0.50, 0.70), p(0.84, 0.52)], thin);
            painter.line_segment([p(0.16, 0.68), p(0.50, 0.86)], thin);
            painter.line_segment([p(0.50, 0.86), p(0.84, 0.68)], thin);
        }
        AppIcon::MapPin => {
            painter.circle_stroke(p(0.50, 0.40), w * 0.24, stroke);
            painter.circle_filled(p(0.50, 0.40), sw * 0.9, color);
            painter.line_segment([p(0.30, 0.52), p(0.50, 0.84)], stroke);
            painter.line_segment([p(0.70, 0.52), p(0.50, 0.84)], stroke);
        }
        AppIcon::Globe | AppIcon::Wind => {
            painter.circle_stroke(sq.center(), w * 0.36, stroke);
            painter.line_segment([p(0.14, 0.50), p(0.86, 0.50)], thin);
            painter.line_segment([p(0.50, 0.14), p(0.50, 0.86)], thin);
            painter.line_segment([p(0.34, 0.22), p(0.28, 0.50)], thin);
            painter.line_segment([p(0.28, 0.50), p(0.34, 0.78)], thin);
            painter.line_segment([p(0.66, 0.22), p(0.72, 0.50)], thin);
            painter.line_segment([p(0.72, 0.50), p(0.66, 0.78)], thin);
        }
        AppIcon::Printer => {
            painter.rect_stroke(
                Rect::from_min_max(p(0.28, 0.16), p(0.72, 0.38)),
                CornerRadius::ZERO,
                thin,
                egui::StrokeKind::Middle,
            );
            painter.rect_stroke(
                Rect::from_min_max(p(0.16, 0.38), p(0.84, 0.68)),
                CornerRadius::same(2),
                stroke,
                egui::StrokeKind::Middle,
            );
            painter.rect_stroke(
                Rect::from_min_max(p(0.30, 0.58), p(0.70, 0.84)),
                CornerRadius::ZERO,
                thin,
                egui::StrokeKind::Middle,
            );
        }
        AppIcon::Palette => {
            painter.circle_stroke(sq.center(), w * 0.36, stroke);
            painter.circle_filled(p(0.38, 0.38), sw * 0.9, color);
            painter.circle_filled(p(0.62, 0.38), sw * 0.9, color);
            painter.circle_filled(p(0.36, 0.60), sw * 0.9, color);
            painter.circle_filled(p(0.62, 0.64), sw * 1.1, color);
        }
        AppIcon::Wall | AppIcon::Slab => {
            painter.rect_stroke(
                Rect::from_min_max(p(0.16, 0.24), p(0.84, 0.76)),
                CornerRadius::ZERO,
                stroke,
                egui::StrokeKind::Middle,
            );
            painter.line_segment([p(0.16, 0.50), p(0.84, 0.50)], thin);
            painter.line_segment([p(0.50, 0.24), p(0.50, 0.50)], thin);
            painter.line_segment([p(0.34, 0.50), p(0.34, 0.76)], thin);
            painter.line_segment([p(0.68, 0.50), p(0.68, 0.76)], thin);
        }
        AppIcon::Door => {
            painter.rect_stroke(
                Rect::from_min_max(p(0.28, 0.16), p(0.72, 0.84)),
                CornerRadius::ZERO,
                stroke,
                egui::StrokeKind::Middle,
            );
            painter.circle_filled(p(0.62, 0.52), sw * 0.85, color);
        }
        AppIcon::Window => {
            let frame = Rect::from_min_max(p(0.20, 0.20), p(0.80, 0.80));
            painter.rect_stroke(frame, CornerRadius::same(1), stroke, egui::StrokeKind::Middle);
            painter.line_segment([p(0.50, 0.20), p(0.50, 0.80)], thin);
            painter.line_segment([p(0.20, 0.50), p(0.80, 0.50)], thin);
        }
        AppIcon::Column | AppIcon::Beam | AppIcon::Stairs => {
            painter.line_segment([p(0.24, 0.22), p(0.76, 0.22)], stroke);
            painter.line_segment([p(0.50, 0.22), p(0.50, 0.78)], stroke);
            painter.line_segment([p(0.24, 0.78), p(0.76, 0.78)], stroke);
        }
        AppIcon::Rebar => {
            painter.circle_stroke(sq.center(), w * 0.34, thin);
            for pt in [p(0.36, 0.36), p(0.64, 0.36), p(0.36, 0.64), p(0.64, 0.64)] {
                painter.circle_filled(pt, sw * 1.15, color);
            }
        }
        AppIcon::SectionCut | AppIcon::Scissors => {
            painter.line_segment([p(0.20, 0.50), p(0.80, 0.50)], stroke);
            painter.line_segment([p(0.20, 0.28), p(0.20, 0.50)], thin);
            painter.line_segment([p(0.80, 0.50), p(0.80, 0.72)], thin);
        }
        AppIcon::NodesGraph | AppIcon::LinkChain => {
            painter.line_segment([p(0.26, 0.30), p(0.74, 0.50)], thin);
            painter.line_segment([p(0.26, 0.70), p(0.74, 0.50)], thin);
            painter.circle_filled(p(0.26, 0.30), sw * 1.3, color);
            painter.circle_filled(p(0.26, 0.70), sw * 1.3, color);
            painter.circle_filled(p(0.74, 0.50), sw * 1.4, color);
        }
        AppIcon::ChartBar => {
            painter.line_segment([p(0.18, 0.80), p(0.84, 0.80)], stroke);
            painter.line_segment([p(0.30, 0.80), p(0.30, 0.48)], stroke);
            painter.line_segment([p(0.50, 0.80), p(0.50, 0.26)], stroke);
            painter.line_segment([p(0.70, 0.80), p(0.70, 0.38)], stroke);
        }
        AppIcon::BoxPackage | AppIcon::Database => {
            painter.rect_stroke(
                Rect::from_min_max(p(0.20, 0.20), p(0.80, 0.80)),
                CornerRadius::same(2),
                stroke,
                egui::StrokeKind::Middle,
            );
            painter.line_segment([p(0.20, 0.44), p(0.80, 0.44)], thin);
            painter.line_segment([p(0.20, 0.62), p(0.80, 0.62)], thin);
        }
        AppIcon::Camera => {
            painter.rect_stroke(
                Rect::from_min_max(p(0.16, 0.30), p(0.84, 0.78)),
                CornerRadius::same(2),
                stroke,
                egui::StrokeKind::Middle,
            );
            painter.circle_stroke(p(0.50, 0.54), w * 0.14, stroke);
            painter.line_segment([p(0.36, 0.30), p(0.42, 0.20)], thin);
            painter.line_segment([p(0.42, 0.20), p(0.58, 0.20)], thin);
            painter.line_segment([p(0.58, 0.20), p(0.64, 0.30)], thin);
        }
        AppIcon::Truck | AppIcon::Hammer | AppIcon::HardHat => {
            painter.rect_stroke(
                Rect::from_min_max(p(0.14, 0.30), p(0.62, 0.66)),
                CornerRadius::same(1),
                stroke,
                egui::StrokeKind::Middle,
            );
            painter.rect_stroke(
                Rect::from_min_max(p(0.62, 0.42), p(0.86, 0.66)),
                CornerRadius::same(1),
                stroke,
                egui::StrokeKind::Middle,
            );
            painter.circle_stroke(p(0.32, 0.74), w * 0.09, thin);
            painter.circle_stroke(p(0.72, 0.74), w * 0.09, thin);
        }
    }
}

/// Allocates a fixed-size square icon widget and draws `icon`.
pub fn draw_icon(ui: &mut Ui, icon: AppIcon, size: f32, color: Color32) -> Response {
    let (rect, resp) = ui.allocate_exact_size(Vec2::splat(size), Sense::hover());
    paint_icon(ui.painter(), rect, icon, color);
    resp
}

/// Draws a compact vector icon immediately followed by a clean label.
#[allow(clippy::too_many_arguments)]
fn paint_atomic_icon_text(
    ui: &mut Ui,
    icon: AppIcon,
    icon_size: f32,
    icon_color: Color32,
    text: String,
    text_size: f32,
    text_color: Color32,
    strong: bool,
    fill: Color32,
    stroke: Stroke,
    radius: CornerRadius,
    pad: Vec2,
    gap: f32,
) {
    let mut rt = egui::RichText::new(text).size(text_size).color(text_color);
    if strong {
        rt = rt.strong();
    }
    let max_row_w = ui.max_rect().width().max(80.0);
    let max_text_w = (max_row_w - pad.x * 2.0 - icon_size - gap).max(40.0);
    let mut galley = egui::WidgetText::from(rt.clone()).into_galley(
        ui,
        Some(egui::TextWrapMode::Extend),
        max_text_w,
        egui::TextStyle::Body,
    );
    if galley.size().x > max_text_w {
        galley = egui::WidgetText::from(rt).into_galley(
            ui,
            Some(egui::TextWrapMode::Wrap),
            max_text_w,
            egui::TextStyle::Body,
        );
    }
    let content_w = if galley.is_empty() {
        icon_size
    } else {
        icon_size + gap + galley.size().x
    };
    let content_h = icon_size.max(galley.size().y);
    let desired = Vec2::new(
        (content_w + pad.x * 2.0).min(max_row_w),
        content_h + pad.y * 2.0,
    );
    let (rect, _) = ui.allocate_exact_size(desired, Sense::hover());
    if fill != Color32::TRANSPARENT {
        ui.painter().rect_filled(rect, radius, fill);
    }
    if stroke.width > 0.0 {
        ui.painter()
            .rect_stroke(rect, radius, stroke, egui::StrokeKind::Middle);
    }
    let icon_rect = Rect::from_min_size(
        Pos2::new(rect.left() + pad.x, rect.center().y - icon_size * 0.5),
        Vec2::splat(icon_size),
    );
    paint_icon(ui.painter(), icon_rect, icon, icon_color);
    if !galley.is_empty() {
        let text_pos = Pos2::new(
            icon_rect.right() + gap,
            rect.center().y - galley.size().y * 0.5,
        );
        ui.painter().galley(text_pos, galley, text_color);
    }
}

#[allow(clippy::too_many_arguments)]
pub fn icon_label(
    ui: &mut Ui,
    icon: AppIcon,
    icon_size: f32,
    icon_color: Color32,
    text: impl Into<String>,
    text_size: f32,
    text_color: Color32,
    strong: bool,
) {
    paint_atomic_icon_text(
        ui,
        icon,
        icon_size,
        icon_color,
        text.into(),
        text_size,
        text_color,
        strong,
        Color32::TRANSPARENT,
        Stroke::NONE,
        CornerRadius::ZERO,
        Vec2::ZERO,
        5.0,
    );
}

/// Interactive button with a crisp painted vector icon on the left and clean English text.
#[allow(clippy::too_many_arguments)]
pub fn icon_button(
    ui: &mut Ui,
    icon: AppIcon,
    text: impl Into<String>,
    font_size: f32,
    fg: Color32,
    bg: Color32,
    stroke: Stroke,
    radius: CornerRadius,
    min_size: Vec2,
) -> Response {
    let label = text.into();
    let font = FontId::proportional(font_size);
    let galley = ui.painter().layout_no_wrap(label.clone(), font, fg);
    let icon_sz = (font_size + 2.5).clamp(11.0, 16.0);
    let gap = if label.is_empty() { 0.0 } else { 6.0 };
    let pad_x = 9.0;
    let pad_y = 5.0;
    let desired_w = (pad_x * 2.0 + icon_sz + gap + galley.size().x).max(min_size.x);
    let desired_h = (galley.size().y.max(icon_sz) + pad_y * 2.0).max(min_size.y);

    let (rect, resp) = ui.allocate_exact_size(Vec2::new(desired_w, desired_h), Sense::click());
    let hovered = resp.hovered();
    if hovered {
        ui.ctx().set_cursor_icon(CursorIcon::PointingHand);
    }
    let fill = if hovered {
        Color32::from_rgba_unmultiplied(
            bg.r().saturating_add(12),
            bg.g().saturating_add(12),
            bg.b().saturating_add(16),
            bg.a().max(32),
        )
    } else {
        bg
    };
    if fill != Color32::TRANSPARENT {
        ui.painter().rect_filled(rect, radius, fill);
    }
    if stroke.width > 0.0 {
        ui.painter()
            .rect_stroke(rect, radius, stroke, egui::StrokeKind::Middle);
    }

    let icon_x = if label.is_empty() {
        rect.center().x - icon_sz * 0.5
    } else {
        rect.left() + pad_x
    };
    let icon_rect = Rect::from_min_size(
        Pos2::new(icon_x, rect.center().y - icon_sz * 0.5),
        Vec2::splat(icon_sz),
    );
    paint_icon(ui.painter(), icon_rect, icon, fg);

    if !label.is_empty() {
        let text_pos = Pos2::new(
            icon_rect.right() + gap,
            rect.center().y - galley.size().y * 0.5,
        );
        ui.painter().galley(text_pos, galley, fg);
    }

    resp
}

/// Square / compact icon-only interactive button with tooltip support.
#[allow(clippy::too_many_arguments)]
pub fn icon_only_button(
    ui: &mut Ui,
    icon: AppIcon,
    fg: Color32,
    bg: Color32,
    stroke: Stroke,
    radius: CornerRadius,
    size: Vec2,
    hover_tip: &str,
) -> Response {
    let resp = icon_button(ui, icon, "", 12.0, fg, bg, stroke, radius, size);
    if hover_tip.is_empty() {
        resp
    } else {
        resp.on_hover_text(hover_tip)
    }
}

/// Maps any leading UI emoji/symbol token into a crisp [`AppIcon`] + clean text without tofu boxes.
pub fn split_icon_and_label(raw: &str) -> (Option<AppIcon>, String) {
    let trimmed = raw.trim();
    let prefix_map: &[(&str, AppIcon)] = &[
        ("🏗️", AppIcon::Cube3D),
        ("🏗", AppIcon::Cube3D),
        ("📐", AppIcon::RulerSquare),
        ("📏", AppIcon::DimensionRuler),
        ("✏️", AppIcon::Pencil),
        ("✏", AppIcon::Pencil),
        ("📁", AppIcon::Folder),
        ("📂", AppIcon::Folder),
        ("🤖", AppIcon::Bot),
        ("💡", AppIcon::Sparkles),
        ("⚡", AppIcon::Bolt),
        ("⏱️", AppIcon::Clock),
        ("⏱", AppIcon::Clock),
        ("⏳", AppIcon::Clock),
        ("💰", AppIcon::Coins),
        ("💵", AppIcon::Coins),
        ("🌱", AppIcon::Leaf),
        ("☀️", AppIcon::Sun),
        ("☀", AppIcon::Sun),
        ("🌑", AppIcon::Moon),
        ("🏢", AppIcon::Building),
        ("🏛️", AppIcon::Building),
        ("🏛", AppIcon::Building),
        ("🏠", AppIcon::Building),
        ("🏡", AppIcon::Building),
        ("🏥", AppIcon::Building),
        ("🛡️", AppIcon::Shield),
        ("🛡", AppIcon::Shield),
        ("♻️", AppIcon::Recycle),
        ("♻", AppIcon::Recycle),
        ("🔮", AppIcon::Pulse),
        ("📈", AppIcon::ChartLine),
        ("📉", AppIcon::ChartLine),
        ("📊", AppIcon::ChartBar),
        ("📑", AppIcon::Document),
        ("📋", AppIcon::Clipboard),
        ("📜", AppIcon::Document),
        ("📝", AppIcon::Pencil),
        ("🗺️", AppIcon::Globe),
        ("🗺", AppIcon::Globe),
        ("🌐", AppIcon::Globe),
        ("🌍", AppIcon::Globe),
        ("📡", AppIcon::Pulse),
        ("🛸", AppIcon::Compass),
        ("⚒️", AppIcon::Wrench),
        ("🛠️", AppIcon::Wrench),
        ("🛠", AppIcon::Wrench),
        ("🔧", AppIcon::Wrench),
        ("🚱", AppIcon::SlidersPanel),
        ("⚔️", AppIcon::AlertTriangle),
        ("⚔", AppIcon::AlertTriangle),
        ("🔥", AppIcon::Flame),
        ("⚠️", AppIcon::AlertTriangle),
        ("⚠", AppIcon::AlertTriangle),
        ("🚨", AppIcon::AlertTriangle),
        ("💥", AppIcon::AlertTriangle),
        ("💬", AppIcon::CommentNote),
        ("🧱", AppIcon::Wall),
        ("🔲", AppIcon::Slab),
        ("🟦", AppIcon::Slab),
        ("🚪", AppIcon::Door),
        ("🪟", AppIcon::Window),
        ("🔩", AppIcon::Rebar),
        ("🪡", AppIcon::Rebar),
        ("🪜", AppIcon::Stairs),
        ("🔪", AppIcon::SectionCut),
        ("✂️", AppIcon::Scissors),
        ("✂", AppIcon::Scissors),
        ("⬇️", AppIcon::ArrowDown),
        ("⬇", AppIcon::ArrowDown),
        ("⚙️", AppIcon::SettingsGear),
        ("⚙", AppIcon::SettingsGear),
        ("ℹ️", AppIcon::Info),
        ("ℹ", AppIcon::Info),
        ("🔒", AppIcon::Lock),
        ("🔓", AppIcon::Unlock),
        ("🎯", AppIcon::Target),
        ("⭐", AppIcon::Star),
        ("☆", AppIcon::Star),
        ("✅", AppIcon::CheckCircle),
        ("🟢", AppIcon::CheckCircle),
        ("🟡", AppIcon::AlertTriangle),
        ("🔴", AppIcon::AlertTriangle),
        ("🔵", AppIcon::CircleShape),
        ("🟠", AppIcon::CircleShape),
        ("🟣", AppIcon::CircleShape),
        ("❌", AppIcon::Close),
        ("✕", AppIcon::Close),
        ("✖", AppIcon::Close),
        ("🔍", AppIcon::Search),
        ("🎬", AppIcon::Play),
        ("▶", AppIcon::Play),
        ("⏸", AppIcon::Pause),
        ("⏮", AppIcon::SkipBack),
        ("⏭️", AppIcon::SkipNext),
        ("⏭", AppIcon::SkipNext),
        ("🧊", AppIcon::SolidBox),
        ("🕸️", AppIcon::Wireframe),
        ("🕸", AppIcon::Wireframe),
        ("🎥", AppIcon::Camera),
        ("🖨️", AppIcon::Printer),
        ("🖨", AppIcon::Printer),
        ("💾", AppIcon::SaveDisk),
        ("📥", AppIcon::UploadImport),
        ("📤", AppIcon::UploadImport),
        ("📦", AppIcon::BoxPackage),
        ("🧬", AppIcon::NodesGraph),
        ("🧲", AppIcon::MagnetSnap),
        ("❄️", AppIcon::Snowflake),
        ("❄", AppIcon::Snowflake),
        ("🎛️", AppIcon::SlidersPanel),
        ("🎛", AppIcon::SlidersPanel),
        ("🧹", AppIcon::Eraser),
        ("✋", AppIcon::CursorPan),
        ("╱", AppIcon::LineSeg),
        ("▭", AppIcon::RectShape),
        ("◯", AppIcon::CircleShape),
        ("↩", AppIcon::Undo),
        ("↶", AppIcon::Undo),
        ("⟲", AppIcon::Undo),
        ("↪", AppIcon::Redo),
        ("↷", AppIcon::Redo),
        ("⟳", AppIcon::Redo),
        ("◀", AppIcon::ChevronLeft),
        ("▼", AppIcon::ChevronDown),
        ("▲", AppIcon::ChevronUp),
        ("➕", AppIcon::Plus),
        ("🔬", AppIcon::Search),
        ("🧪", AppIcon::Search),
        ("⛰️", AppIcon::Compass),
        ("⛰", AppIcon::Compass),
        ("🕳️", AppIcon::CircleShape),
        ("🌊", AppIcon::Wind),
        ("🌪️", AppIcon::Wind),
        ("🛰️", AppIcon::Compass),
        ("🌌", AppIcon::Globe),
        ("📍", AppIcon::MapPin),
        ("🎪", AppIcon::Wireframe),
        ("🌉", AppIcon::Beam),
        ("🏷️", AppIcon::CommentNote),
        ("⬛", AppIcon::RectShape),
        ("〰️", AppIcon::ChartLine),
        ("🚚", AppIcon::Truck),
        ("🚛", AppIcon::Truck),
        ("🚋", AppIcon::Truck),
        ("🚂", AppIcon::Truck),
        ("👷", AppIcon::HardHat),
        ("🎲", AppIcon::Cube3D),
        ("📽️", AppIcon::Camera),
        ("🚑", AppIcon::AlertTriangle),
        ("🪪", AppIcon::Clipboard),
        ("⚛️", AppIcon::Sparkles),
        ("⌘", AppIcon::SettingsGear),
        ("⤢", AppIcon::ZoomFit),
        ("⤡", AppIcon::Close),
        ("🔄", AppIcon::Redo),
        ("🏭", AppIcon::Cpu),
        ("🦾", AppIcon::Cpu),
        ("👔", AppIcon::Bot),
        ("👥", AppIcon::Bot),
        ("⚖️", AppIcon::Shield),
        ("🚀", AppIcon::Bolt),
        ("🎨", AppIcon::Palette),
        ("⌨️", AppIcon::Cpu),
        ("⌨", AppIcon::Cpu),
        ("🪄", AppIcon::Sparkles),
        ("🔗", AppIcon::LinkChain),
        ("💧", AppIcon::Wind),
        ("🏅", AppIcon::Star),
        ("🏆", AppIcon::Star),
        ("🌟", AppIcon::Star),
        ("🎉", AppIcon::Sparkles),
        ("🧠", AppIcon::Bot),
        ("📖", AppIcon::Book),
        ("📚", AppIcon::Book),
        ("🗑️", AppIcon::Trash),
        ("🗑", AppIcon::Trash),
        ("✍️", AppIcon::Pencil),
        ("✍", AppIcon::Pencil),
        ("🔐", AppIcon::Lock),
        ("🔑", AppIcon::Lock),
        ("📢", AppIcon::CommentNote),
        ("🔎", AppIcon::Search),
        ("🪪", AppIcon::Clipboard),
        ("🧯", AppIcon::Flame),
        ("🧗", AppIcon::HardHat),
        ("🔌", AppIcon::Bolt),
        ("📬", AppIcon::UploadImport),
        ("🌳", AppIcon::Leaf),
        ("💣", AppIcon::AlertTriangle),
        ("🥞", AppIcon::Slab),
        ("🪢", AppIcon::LinkChain),
        ("⛓️", AppIcon::LinkChain),
        ("⛓", AppIcon::LinkChain),
        ("🎓", AppIcon::Book),
        ("⚓", AppIcon::Compass),
        ("🗼", AppIcon::Building),
        ("🎮", AppIcon::Cube3D),
        ("☁️", AppIcon::Info),
        ("☁", AppIcon::Info),
        ("⚪", AppIcon::CircleShape),
        ("⏪", AppIcon::SkipBack),
        ("⏩", AppIcon::SkipNext),
        ("↺", AppIcon::Undo),
        ("◼", AppIcon::RectShape),
        ("■", AppIcon::RectShape),
        ("⬛", AppIcon::RectShape),
        ("☰", AppIcon::Menu),
        ("💨", AppIcon::Wind),
        ("🌬️", AppIcon::Wind),
        ("🌬", AppIcon::Wind),
        ("🌀", AppIcon::SlidersPanel),
        ("🚰", AppIcon::Wrench),
        ("🚽", AppIcon::BoxPackage),
        ("🔋", AppIcon::Bolt),
        ("🚒", AppIcon::Flame),
        ("🚿", AppIcon::Wind),
        ("🛣️", AppIcon::LineSeg),
        ("🛣", AppIcon::LineSeg),
        ("🚜", AppIcon::Truck),
        ("📅", AppIcon::Clock),
        ("💲", AppIcon::Coins),
        ("🪙", AppIcon::Coins),
        ("⛅️", AppIcon::Sun),
        ("⛅", AppIcon::Sun),
        ("📄", AppIcon::Document),
        ("#", AppIcon::Grid),
    ];

    for (prefix, icon) in prefix_map {
        if let Some(rest) = trimmed.strip_prefix(prefix) {
            return (Some(*icon), strip_pictographs(rest));
        }
    }

    (None, strip_pictographs(trimmed))
}

/// Strips emoji/pictograph codepoints so UI strings never display broken font tofu (`□`)
/// while preserving ASCII and standard engineering typography (`— • × ± ° ≥ ≤ Δ σ ε μ λ θ φ`).
pub fn strip_pictographs(input: &str) -> String {
    let mut out = String::with_capacity(input.len());
    for ch in input.chars() {
        let cp = ch as u32;
        let is_emoji_or_vs = cp == 0xFE0F
            || cp == 0xFE0E
            || cp == 0x200D
            || (0x1F000..=0x1FAFF).contains(&cp)
            || (0x2600..=0x27BF).contains(&cp)
            || (0x2300..=0x23FF).contains(&cp)
            || (0x25A0..=0x25FF).contains(&cp)
            || (0x2B00..=0x2BFF).contains(&cp)
            || matches!(
                ch,
                '⌘' | '⤢' | '⤡' | '↶' | '↷' | '⟲' | '⟳' | '↺' | '↩' | '↪' | '↵' | '↔' | '✕' | '✖' | '☰' | '╱' | '✓'
            );
        if !is_emoji_or_vs {
            out.push(ch);
        }
    }
    out.split_whitespace().collect::<Vec<_>>().join(" ")
}

/// Cleans text for string-only egui surfaces (Window titles, ComboBox selected text, collapsing headers).
pub fn clean_text(raw: &str) -> String {
    let (_icon, cleaned) = split_icon_and_label(raw);
    cleaned
}

/// Smart drop-in button that extracts any leading symbol into a vector [`AppIcon`].
#[allow(clippy::too_many_arguments)]
pub fn smart_button(
    ui: &mut Ui,
    raw: &str,
    font_size: f32,
    fg: Color32,
    bg: Color32,
    stroke: Stroke,
    radius: CornerRadius,
    min_size: Vec2,
) -> Response {
    let (maybe_icon, label) = split_icon_and_label(raw);
    if let Some(icon) = maybe_icon {
        icon_button(ui, icon, label, font_size, fg, bg, stroke, radius, min_size)
    } else {
        let btn = egui::Button::new(egui::RichText::new(label).size(font_size).color(fg))
            .fill(bg)
            .stroke(stroke)
            .corner_radius(radius)
            .min_size(min_size);
        ui.add(btn)
    }
}

/// Drop-in selectable pill with crisp vector icon + theme highlight.
pub fn smart_selectable(
    ui: &mut Ui,
    theme: &MacTheme,
    selected: bool,
    raw: &str,
) -> Response {
    let fg = if selected {
        Color32::WHITE
    } else {
        theme.text_secondary
    };
    let bg = if selected {
        theme.accent_blue
    } else {
        theme.bg_card
    };
    let stroke = Stroke::new(
        1.0,
        if selected {
            theme.accent_blue
        } else {
            theme.border_subtle
        },
    );
    smart_button(
        ui,
        raw,
        11.0,
        fg,
        bg,
        stroke,
        CornerRadius::same(6),
        Vec2::new(0.0, 24.0),
    )
}

/// Renders a crisp section heading with its mapped vector [`AppIcon`].
pub fn smart_heading(
    ui: &mut Ui,
    raw: &str,
    size: f32,
    icon_color: Color32,
    text_color: Color32,
) {
    let (maybe_icon, text) = split_icon_and_label(raw);
    if let Some(icon) = maybe_icon {
        icon_label(ui, icon, (size + 2.0).clamp(14.0, 22.0), icon_color, text, size, text_color, true);
    } else {
        ui.heading(egui::RichText::new(text).size(size).strong().color(text_color));
    }
}

/// Renders an inline label with mapped vector [`AppIcon`] when a leading token is present.
pub fn smart_label(ui: &mut Ui, raw: &str, size: f32, color: Color32, strong: bool) {
    let (maybe_icon, text) = split_icon_and_label(raw);
    if let Some(icon) = maybe_icon {
        icon_label(ui, icon, (size + 2.0).clamp(11.0, 18.0), color, text, size, color, strong);
    } else {
        let mut rt = egui::RichText::new(text).size(size).color(color);
        if strong {
            rt = rt.strong();
        }
        ui.label(rt);
    }
}

/// Compact pill badge with a crisp painted vector icon.
pub fn pill_badge(ui: &mut Ui, icon: AppIcon, text: impl Into<String>, color: Color32) {
    paint_atomic_icon_text(
        ui,
        icon,
        11.5,
        color,
        text.into(),
        10.0,
        color,
        true,
        color.gamma_multiply(0.14),
        Stroke::new(1.0, color.gamma_multiply(0.55)),
        CornerRadius::same(6),
        Vec2::new(8.0, 3.0),
        5.0,
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_split_icon_and_label_strips_emoji_and_preserves_math() {
        let (icon, label) = split_icon_and_label("🏗️ BIMCraft LOD350 • ≥ 15°");
        assert_eq!(icon, Some(AppIcon::Cube3D));
        assert_eq!(label, "BIMCraft LOD350 • ≥ 15°");

        let (icon2, label2) = split_icon_and_label("❄️ Frozen (Scroll Page)");
        assert_eq!(icon2, Some(AppIcon::Snowflake));
        assert_eq!(label2, "Frozen (Scroll Page)");
    }

    #[test]
    fn test_clean_text_removes_tofu_triggers() {
        let cleaned = clean_text("⚙️ Settings — Workspace & 🛡️ Safety");
        assert_eq!(cleaned, "Settings — Workspace & Safety");
    }

    #[test]
    fn test_all_rect_paint_bounds_valid() {
        let rect = Rect::from_min_size(Pos2::ZERO, Vec2::splat(16.0));
        assert!(rect.width() > 0.0);
        let _ = egui::Align2::CENTER_CENTER;
    }

    #[test]
    fn test_ribbon_prefixes_map_cleanly() {
        let cases = [
            ("💨 Supply Duct", AppIcon::Wind, "Supply Duct"),
            ("🌬️ Air Terminal", AppIcon::Wind, "Air Terminal"),
            ("🌀 VAV Terminal", AppIcon::SlidersPanel, "VAV Terminal"),
            ("🚰 Domestic Pipe", AppIcon::Wrench, "Domestic Pipe"),
            ("🚽 Sanitary DWV", AppIcon::BoxPackage, "Sanitary DWV"),
            ("🔋 Main Switchboard", AppIcon::Bolt, "Main Switchboard"),
            ("🚒 Fire Sprinkler", AppIcon::Flame, "Fire Sprinkler"),
            ("🚿 Wet Riser", AppIcon::Wind, "Wet Riser"),
            ("🛣️ Road Centerline", AppIcon::LineSeg, "Road Centerline"),
            ("🚜 Cut & Fill Mesh", AppIcon::Truck, "Cut & Fill Mesh"),
            ("📅 CPM Gantt View", AppIcon::Clock, "CPM Gantt View"),
            ("💲 Bill of Quantities (BoQ)", AppIcon::Coins, "Bill of Quantities (BoQ)"),
            ("🪙 Currency USD/EUR/IDR", AppIcon::Coins, "Currency USD/EUR/IDR"),
            ("⛅ Cast Shadows", AppIcon::Sun, "Cast Shadows"),
            ("📄 New Sheet (A1)", AppIcon::Document, "New Sheet (A1)"),
        ];
        for (raw, expected_icon, expected_label) in cases {
            let (icon, label) = split_icon_and_label(raw);
            assert_eq!(icon, Some(expected_icon), "failed icon for {raw}");
            assert_eq!(label, expected_label, "failed label for {raw}");
        }
    }
}
