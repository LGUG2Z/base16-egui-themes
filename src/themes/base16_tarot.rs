use egui::{
    epaint::{Color32, Shadow, Stroke},
    style::{Selection, WidgetVisuals, Widgets},
    Style, Visuals,
};

pub const BASE_00: Color32 = Color32::from_rgb(14, 9, 29);
pub const BASE_01: Color32 = Color32::from_rgb(42, 21, 60);
pub const BASE_02: Color32 = Color32::from_rgb(75, 32, 84);
pub const BASE_03: Color32 = Color32::from_rgb(116, 49, 107);
pub const BASE_04: Color32 = Color32::from_rgb(140, 64, 111);
pub const BASE_05: Color32 = Color32::from_rgb(170, 85, 111);
pub const BASE_06: Color32 = Color32::from_rgb(196, 104, 109);
pub const BASE_07: Color32 = Color32::from_rgb(220, 143, 124);
pub const BASE_08: Color32 = Color32::from_rgb(197, 50, 83);
pub const BASE_09: Color32 = Color32::from_rgb(234, 77, 96);
pub const BASE_0A: Color32 = Color32::from_rgb(255, 101, 101);
pub const BASE_0B: Color32 = Color32::from_rgb(166, 142, 90);
pub const BASE_0C: Color32 = Color32::from_rgb(140, 151, 133);
pub const BASE_0D: Color32 = Color32::from_rgb(110, 96, 128);
pub const BASE_0E: Color32 = Color32::from_rgb(164, 87, 130);
pub const BASE_0F: Color32 = Color32::from_rgb(152, 77, 81);

const SHADOW: Color32 = Color32::from_rgba_premultiplied(0, 0, 0, 96);
const TRANSPARENT: Color32 = Color32::from_rgba_premultiplied(0, 0, 0, 0);
const EXTREME_BACKGROUND: Color32 = BASE_00;
const MAIN_BACKGROUND: Color32 = BASE_01;
const MAIN_FOREGROUND: Color32 = BASE_02;
const CODE_BACKGROUND: Color32 = BASE_02;
const HOVERED_FILL: Color32 = BASE_02;
const HOVER_BACKGROUND: Color32 = BASE_03;
const UNUSABLE: Color32 = BASE_05;
const INACTIVE_STROKE: Color32 = BASE_05;
const OPEN_FILL: Color32 = BASE_06;
const HOVER: Color32 = BASE_06;
const HYPERLINK: Color32 = BASE_08;
const ERROR_FOREGROUND: Color32 = BASE_0B;
const WARNING_FOREGROUND: Color32 = BASE_0C;

pub fn style_from(original: Style) -> Style {
    Style {
        visuals: Visuals {
            widgets: Widgets {
                noninteractive: WidgetVisuals {
                    bg_fill: MAIN_BACKGROUND,
                    weak_bg_fill: MAIN_BACKGROUND,
                    bg_stroke: Stroke {
                        color: MAIN_FOREGROUND,
                        ..original.visuals.widgets.noninteractive.bg_stroke
                    },
                    fg_stroke: Stroke {
                        color: UNUSABLE,
                        ..original.visuals.widgets.noninteractive.fg_stroke
                    },
                    ..original.visuals.widgets.noninteractive
                },
                inactive: WidgetVisuals {
                    bg_fill: MAIN_FOREGROUND,
                    weak_bg_fill: MAIN_FOREGROUND,
                    bg_stroke: Stroke {
                        color: TRANSPARENT,
                        ..original.visuals.widgets.inactive.bg_stroke
                    },
                    fg_stroke: Stroke {
                        color: INACTIVE_STROKE,
                        ..original.visuals.widgets.inactive.fg_stroke
                    },
                    ..original.visuals.widgets.inactive
                },
                hovered: WidgetVisuals {
                    bg_fill: HOVERED_FILL,
                    weak_bg_fill: HOVERED_FILL,
                    bg_stroke: Stroke {
                        color: HOVER_BACKGROUND,
                        ..original.visuals.widgets.hovered.bg_stroke
                    },
                    fg_stroke: Stroke {
                        color: HOVER,
                        ..original.visuals.widgets.hovered.fg_stroke
                    },
                    ..original.visuals.widgets.hovered
                },
                active: WidgetVisuals {
                    bg_fill: HOVERED_FILL,
                    weak_bg_fill: HOVERED_FILL,
                    bg_stroke: Stroke {
                        color: HOVER_BACKGROUND,
                        ..original.visuals.widgets.hovered.bg_stroke
                    },
                    fg_stroke: Stroke {
                        color: HOVER,
                        ..original.visuals.widgets.hovered.fg_stroke
                    },
                    ..original.visuals.widgets.active
                },
                open: WidgetVisuals {
                    bg_fill: MAIN_BACKGROUND,
                    weak_bg_fill: MAIN_BACKGROUND,
                    bg_stroke: Stroke {
                        color: MAIN_FOREGROUND,
                        ..original.visuals.widgets.open.bg_stroke
                    },
                    fg_stroke: Stroke {
                        color: OPEN_FILL,
                        ..original.visuals.widgets.open.fg_stroke
                    },
                    ..original.visuals.widgets.open
                },
            },
            selection: Selection {
                bg_fill: HOVERED_FILL,
                stroke: Stroke {
                    color: HOVER,
                    ..original.visuals.selection.stroke
                },
            },
            hyperlink_color: HYPERLINK,
            faint_bg_color: TRANSPARENT,
            extreme_bg_color: EXTREME_BACKGROUND,
            code_bg_color: CODE_BACKGROUND,
            warn_fg_color: WARNING_FOREGROUND,
            error_fg_color: ERROR_FOREGROUND,
            window_shadow: Shadow {
                color: SHADOW,
                ..original.visuals.window_shadow
            },
            window_fill: MAIN_BACKGROUND,
            window_stroke: Stroke {
                color: MAIN_FOREGROUND,
                ..original.visuals.window_stroke
            },
            panel_fill: MAIN_BACKGROUND,
            popup_shadow: Shadow {
                color: SHADOW,
                ..original.visuals.popup_shadow
            },
            ..original.visuals
        },
        ..original
    }
}
pub fn default_style() -> Style {
    style_from(Default::default())
}
