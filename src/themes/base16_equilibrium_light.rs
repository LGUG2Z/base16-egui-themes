use egui::{
    epaint::{Color32, Shadow, Stroke},
    style::{Selection, WidgetVisuals, Widgets},
    Style, Visuals,
};

pub const BASE_00: Color32 = Color32::from_rgb(245, 240, 231);
pub const BASE_01: Color32 = Color32::from_rgb(231, 226, 217);
pub const BASE_02: Color32 = Color32::from_rgb(216, 212, 203);
pub const BASE_03: Color32 = Color32::from_rgb(115, 119, 127);
pub const BASE_04: Color32 = Color32::from_rgb(90, 95, 102);
pub const BASE_05: Color32 = Color32::from_rgb(67, 71, 78);
pub const BASE_06: Color32 = Color32::from_rgb(44, 49, 56);
pub const BASE_07: Color32 = Color32::from_rgb(24, 28, 34);
pub const BASE_08: Color32 = Color32::from_rgb(208, 32, 35);
pub const BASE_09: Color32 = Color32::from_rgb(191, 62, 5);
pub const BASE_0A: Color32 = Color32::from_rgb(157, 111, 0);
pub const BASE_0B: Color32 = Color32::from_rgb(99, 114, 0);
pub const BASE_0C: Color32 = Color32::from_rgb(0, 122, 114);
pub const BASE_0D: Color32 = Color32::from_rgb(0, 115, 181);
pub const BASE_0E: Color32 = Color32::from_rgb(78, 102, 182);
pub const BASE_0F: Color32 = Color32::from_rgb(196, 39, 117);

pub const SHADOW: Color32 = Color32::from_rgba_premultiplied(0, 0, 0, 96);
pub const TRANSPARENT: Color32 = Color32::from_rgba_premultiplied(0, 0, 0, 0);
pub const BACKGROUND: Color32 = BASE_01;

pub fn style_from(original: Style) -> Style {
    Style {
        visuals: Visuals {
            widgets: Widgets {
                noninteractive: WidgetVisuals {
                    bg_fill: BASE_01,
                    weak_bg_fill: BASE_01,
                    bg_stroke: Stroke {
                        color: BASE_02,
                        ..original.visuals.widgets.noninteractive.bg_stroke
                    },
                    fg_stroke: Stroke {
                        color: BASE_05,
                        ..original.visuals.widgets.noninteractive.fg_stroke
                    },
                    ..original.visuals.widgets.noninteractive
                },
                inactive: WidgetVisuals {
                    bg_fill: BASE_02,
                    weak_bg_fill: BASE_02,
                    bg_stroke: Stroke {
                        color: TRANSPARENT,
                        ..original.visuals.widgets.inactive.bg_stroke
                    },
                    fg_stroke: Stroke {
                        color: BASE_05,
                        ..original.visuals.widgets.inactive.fg_stroke
                    },
                    ..original.visuals.widgets.inactive
                },
                hovered: WidgetVisuals {
                    bg_fill: BASE_02,
                    weak_bg_fill: BASE_02,
                    bg_stroke: Stroke {
                        color: BASE_03,
                        ..original.visuals.widgets.hovered.bg_stroke
                    },
                    fg_stroke: Stroke {
                        color: BASE_06,
                        ..original.visuals.widgets.hovered.fg_stroke
                    },
                    ..original.visuals.widgets.hovered
                },
                active: WidgetVisuals {
                    bg_fill: BASE_02,
                    weak_bg_fill: BASE_02,
                    bg_stroke: Stroke {
                        color: BASE_03,
                        ..original.visuals.widgets.hovered.bg_stroke
                    },
                    fg_stroke: Stroke {
                        color: BASE_06,
                        ..original.visuals.widgets.hovered.fg_stroke
                    },
                    ..original.visuals.widgets.active
                },
                open: WidgetVisuals {
                    bg_fill: BASE_01,
                    weak_bg_fill: BASE_01,
                    bg_stroke: Stroke {
                        color: BASE_02,
                        ..original.visuals.widgets.open.bg_stroke
                    },
                    fg_stroke: Stroke {
                        color: BASE_06,
                        ..original.visuals.widgets.open.fg_stroke
                    },
                    ..original.visuals.widgets.open
                },
            },
            selection: Selection {
                bg_fill: BASE_02,
                stroke: Stroke {
                    color: BASE_06,
                    ..original.visuals.selection.stroke
                },
            },
            hyperlink_color: BASE_08,
            faint_bg_color: TRANSPARENT,
            extreme_bg_color: BASE_00,
            code_bg_color: BASE_02,
            warn_fg_color: BASE_0C,
            error_fg_color: BASE_0B,
            window_shadow: Shadow {
                color: SHADOW,
                ..original.visuals.window_shadow
            },
            window_fill: BASE_01,
            window_stroke: Stroke {
                color: BASE_02,
                ..original.visuals.window_stroke
            },
            panel_fill: BASE_01,
            popup_shadow: Shadow {
                color: SHADOW,
                ..original.visuals.popup_shadow
            },
            ..original.visuals
        },
        ..original
    }
}

pub fn style() -> Style {
    style_from(Default::default())
}
