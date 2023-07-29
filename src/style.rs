use egui::epaint::Shadow;
use egui::style::Selection;
use egui::style::WidgetVisuals;
use egui::style::Widgets;
use egui::Color32;
use egui::Rounding;
use egui::Stroke;
use egui::Visuals;

pub struct NordColorScheme {
    pub nord0: Color32,
    pub nord1: Color32,
    pub nord2: Color32,
    pub nord3: Color32,
    pub nord4: Color32,
    pub nord5: Color32,
    pub nord6: Color32,
    pub nord7: Color32,
    pub nord8: Color32,
    pub nord9: Color32,
    pub nord10: Color32,
    pub nord11: Color32,
    pub nord12: Color32,
    pub nord13: Color32,
    pub nord14: Color32,
    pub nord15: Color32,
}

impl NordColorScheme {
    pub fn new() -> Self {
        Self {
            nord0: Color32::from_rgb(46, 52, 64),
            nord1: Color32::from_rgb(59, 66, 82),
            nord2: Color32::from_rgb(67, 76, 94),
            nord3: Color32::from_rgb(76, 86, 106),
            nord4: Color32::from_rgb(216, 222, 233),
            nord5: Color32::from_rgb(229, 233, 240),
            nord6: Color32::from_rgb(236, 239, 244),
            nord7: Color32::from_rgb(143, 188, 187),
            nord8: Color32::from_rgb(136, 192, 208),
            nord9: Color32::from_rgb(129, 161, 193),
            nord10: Color32::from_rgb(94, 129, 172),
            nord11: Color32::from_rgb(191, 97, 106),
            nord12: Color32::from_rgb(208, 135, 112),
            nord13: Color32::from_rgb(163, 190, 140),
            nord14: Color32::from_rgb(180, 142, 173),
            nord15: Color32::from_rgb(235, 203, 139),
        }
    }
}

pub fn nord_ui_visuals() -> egui::Visuals {
    let nord_colors = NordColorScheme::new();
    Visuals {
        dark_mode: true,
        override_text_color: Some(nord_colors.nord6),
        widgets: nord_widgets(),
        selection: Selection::default(),
        hyperlink_color: nord_colors.nord9,
        faint_bg_color: nord_colors.nord0,
        extreme_bg_color: nord_colors.nord1,
        code_bg_color: nord_colors.nord1,
        warn_fg_color: nord_colors.nord12,
        error_fg_color: nord_colors.nord11,

        window_rounding: Rounding::same(6.0),
        window_shadow: Shadow::small_dark(),
        window_fill: nord_colors.nord0,
        window_stroke: Stroke::new(1.0, nord_colors.nord1),

        menu_rounding: Rounding::same(6.0),

        panel_fill: nord_colors.nord0,

        popup_shadow: Shadow::small_dark(),
        resize_corner_size: 12.0,
        text_cursor_width: 2.0,
        text_cursor_preview: false,
        clip_rect_margin: 3.0,
        button_frame: true,
        collapsing_header_frame: false,
        indent_has_left_vline: true,

        striped: false,

        slider_trailing_fill: false,
    }
}

fn nord_widgets() -> Widgets {
    let nord_colors = NordColorScheme::new();

    Widgets {
        noninteractive: WidgetVisuals {
            weak_bg_fill: nord_colors.nord0,
            bg_fill: nord_colors.nord0,
            bg_stroke: Stroke::new(1.0, nord_colors.nord1),
            fg_stroke: Stroke::new(1.0, nord_colors.nord4),
            rounding: Rounding::same(2.0),
            expansion: 0.0,
        },
        inactive: WidgetVisuals {
            weak_bg_fill: nord_colors.nord1,
            bg_fill: nord_colors.nord1,
            bg_stroke: Default::default(),
            fg_stroke: Stroke::new(1.0, nord_colors.nord4),
            rounding: Rounding::same(2.0),
            expansion: 0.0,
        },
        hovered: WidgetVisuals {
            weak_bg_fill: nord_colors.nord2,
            bg_fill: nord_colors.nord2,
            bg_stroke: Stroke::new(1.0, nord_colors.nord3),
            fg_stroke: Stroke::new(1.5, nord_colors.nord6),
            rounding: Rounding::same(3.0),
            expansion: 1.0,
        },
        active: WidgetVisuals {
            weak_bg_fill: nord_colors.nord1,
            bg_fill: nord_colors.nord1,
            bg_stroke: Stroke::new(1.0, nord_colors.nord6),
            fg_stroke: Stroke::new(2.0, nord_colors.nord6),
            rounding: Rounding::same(2.0),
            expansion: 1.0,
        },
        open: WidgetVisuals {
            weak_bg_fill: nord_colors.nord0,
            bg_fill: nord_colors.nord0,
            bg_stroke: Stroke::new(1.0, nord_colors.nord1),
            fg_stroke: Stroke::new(1.0, nord_colors.nord4),
            rounding: Rounding::same(2.0),
            expansion: 0.0,
        },
    }
}
