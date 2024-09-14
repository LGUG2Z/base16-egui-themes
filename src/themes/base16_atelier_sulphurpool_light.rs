use egui::{
    epaint::{Color32, Shadow, Stroke},
    style::{Selection, WidgetVisuals, Widgets},
    Style, Visuals,
};

pub const BASE_00: Color32 = Color32::from_rgb(245, 247, 255);
pub const BASE_01: Color32 = Color32::from_rgb(223, 226, 241);
pub const BASE_02: Color32 = Color32::from_rgb(151, 157, 180);
pub const BASE_03: Color32 = Color32::from_rgb(137, 142, 164);
pub const BASE_04: Color32 = Color32::from_rgb(107, 115, 148);
pub const BASE_05: Color32 = Color32::from_rgb(94, 102, 135);
pub const BASE_06: Color32 = Color32::from_rgb(41, 50, 86);
pub const BASE_07: Color32 = Color32::from_rgb(32, 39, 70);
pub const BASE_08: Color32 = Color32::from_rgb(201, 73, 34);
pub const BASE_09: Color32 = Color32::from_rgb(199, 107, 41);
pub const BASE_0A: Color32 = Color32::from_rgb(192, 139, 48);
pub const BASE_0B: Color32 = Color32::from_rgb(172, 151, 57);
pub const BASE_0C: Color32 = Color32::from_rgb(34, 162, 201);
pub const BASE_0D: Color32 = Color32::from_rgb(61, 143, 209);
pub const BASE_0E: Color32 = Color32::from_rgb(102, 121, 204);
pub const BASE_0F: Color32 = Color32::from_rgb(156, 99, 122);

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
