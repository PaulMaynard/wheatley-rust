use doryen_rs::Color;

// NOTE; colour names and values copied from:
// tcod-sys/libtcod/include/libtcod_int.h
pub const BLACK: Color = (0, 0, 0, 255);
pub const DARKEST_GREY: Color = (31, 31, 31, 255);
pub const DARKER_GREY: Color = (63, 63, 63, 255);
pub const DARK_GREY: Color = (95, 95, 95, 255);
pub const GREY: Color = (127, 127, 127, 255);
pub const LIGHT_GREY: Color = (159, 159, 159, 255);
pub const LIGHTER_GREY: Color = (191, 191, 191, 255);
pub const LIGHTEST_GREY: Color = (223, 223, 223, 255);
pub const WHITE: Color = (255, 255, 255, 255);
pub const DARKEST_SEPIA: Color = (31, 24, 15, 255);
pub const DARKER_SEPIA: Color = (63, 50, 31, 255);
pub const DARK_SEPIA: Color = (94, 75, 47, 255);
pub const SEPIA: Color = (127, 101, 63, 255);
pub const LIGHT_SEPIA: Color = (158, 134, 100, 255);
pub const LIGHTER_SEPIA: Color = (191, 171, 143, 255);
pub const LIGHTEST_SEPIA: Color = (222, 211, 195, 255);
pub const DESATURATED_RED: Color = (127, 63, 63, 255);
pub const DESATURATED_FLAME: Color = (127, 79, 63, 255);
pub const DESATURATED_ORANGE: Color = (127, 95, 63, 255);
pub const DESATURATED_AMBER: Color = (127, 111, 63, 255);
pub const DESATURATED_YELLOW: Color = (127, 127, 63, 255);
pub const DESATURATED_LIME: Color = (111, 127, 63, 255);
pub const DESATURATED_CHARTREUSE: Color = (95, 127, 63, 255);
pub const DESATURATED_GREEN: Color = (63, 127, 63, 255);
pub const DESATURATED_SEA: Color = (63, 127, 95, 255);
pub const DESATURATED_TURQUOISE: Color = (63, 127, 111, 255);
pub const DESATURATED_CYAN: Color = (63, 127, 127, 255);
pub const DESATURATED_SKY: Color = (63, 111, 127, 255);
pub const DESATURATED_AZURE: Color = (63, 95, 127, 255);
pub const DESATURATED_BLUE: Color = (63, 63, 127, 255);
pub const DESATURATED_HAN: Color = (79, 63, 127, 255);
pub const DESATURATED_VIOLET: Color = (95, 63, 127, 255);
pub const DESATURATED_PURPLE: Color = (111, 63, 127, 255);
pub const DESATURATED_FUCHSIA: Color = (127, 63, 127, 255);
pub const DESATURATED_MAGENTA: Color = (127, 63, 111, 255);
pub const DESATURATED_PINK: Color = (127, 63, 95, 255);
pub const DESATURATED_CRIMSON: Color = (127, 63, 79, 255);
pub const LIGHTEST_RED: Color = (255, 191, 191, 255);
pub const LIGHTEST_FLAME: Color = (255, 207, 191, 255);
pub const LIGHTEST_ORANGE: Color = (255, 223, 191, 255);
pub const LIGHTEST_AMBER: Color = (255, 239, 191, 255);
pub const LIGHTEST_YELLOW: Color = (255, 255, 191, 255);
pub const LIGHTEST_LIME: Color = (239, 255, 191, 255);
pub const LIGHTEST_CHARTREUSE: Color = (223, 255, 191, 255);
pub const LIGHTEST_GREEN: Color = (191, 255, 191, 255);
pub const LIGHTEST_SEA: Color = (191, 255, 223, 255);
pub const LIGHTEST_TURQUOISE: Color = (191, 255, 239, 255);
pub const LIGHTEST_CYAN: Color = (191, 255, 255, 255);
pub const LIGHTEST_SKY: Color = (191, 239, 255, 255);
pub const LIGHTEST_AZURE: Color = (191, 223, 255, 255);
pub const LIGHTEST_BLUE: Color = (191, 191, 255, 255);
pub const LIGHTEST_HAN: Color = (207, 191, 255, 255);
pub const LIGHTEST_VIOLET: Color = (223, 191, 255, 255);
pub const LIGHTEST_PURPLE: Color = (239, 191, 255, 255);
pub const LIGHTEST_FUCHSIA: Color = (255, 191, 255, 255);
pub const LIGHTEST_MAGENTA: Color = (255, 191, 239, 255);
pub const LIGHTEST_PINK: Color = (255, 191, 223, 255);
pub const LIGHTEST_CRIMSON: Color = (255, 191, 207, 255);
pub const LIGHTER_RED: Color = (255, 127, 127, 255);
pub const LIGHTER_FLAME: Color = (255, 159, 127, 255);
pub const LIGHTER_ORANGE: Color = (255, 191, 127, 255);
pub const LIGHTER_AMBER: Color = (255, 223, 127, 255);
pub const LIGHTER_YELLOW: Color = (255, 255, 127, 255);
pub const LIGHTER_LIME: Color = (223, 255, 127, 255);
pub const LIGHTER_CHARTREUSE: Color = (191, 255, 127, 255);
pub const LIGHTER_GREEN: Color = (127, 255, 127, 255);
pub const LIGHTER_SEA: Color = (127, 255, 191, 255);
pub const LIGHTER_TURQUOISE: Color = (127, 255, 223, 255);
pub const LIGHTER_CYAN: Color = (127, 255, 255, 255);
pub const LIGHTER_SKY: Color = (127, 223, 255, 255);
pub const LIGHTER_AZURE: Color = (127, 191, 255, 255);
pub const LIGHTER_BLUE: Color = (127, 127, 255, 255);
pub const LIGHTER_HAN: Color = (159, 127, 255, 255);
pub const LIGHTER_VIOLET: Color = (191, 127, 255, 255);
pub const LIGHTER_PURPLE: Color = (223, 127, 255, 255);
pub const LIGHTER_FUCHSIA: Color = (255, 127, 255, 255);
pub const LIGHTER_MAGENTA: Color = (255, 127, 223, 255);
pub const LIGHTER_PINK: Color = (255, 127, 191, 255);
pub const LIGHTER_CRIMSON: Color = (255, 127, 159, 255);
pub const LIGHT_RED: Color = (255, 63, 63, 255);
pub const LIGHT_FLAME: Color = (255, 111, 63, 255);
pub const LIGHT_ORANGE: Color = (255, 159, 63, 255);
pub const LIGHT_AMBER: Color = (255, 207, 63, 255);
pub const LIGHT_YELLOW: Color = (255, 255, 63, 255);
pub const LIGHT_LIME: Color = (207, 255, 63, 255);
pub const LIGHT_CHARTREUSE: Color = (159, 255, 63, 255);
pub const LIGHT_GREEN: Color = (63, 255, 63, 255);
pub const LIGHT_SEA: Color = (63, 255, 159, 255);
pub const LIGHT_TURQUOISE: Color = (63, 255, 207, 255);
pub const LIGHT_CYAN: Color = (63, 255, 255, 255);
pub const LIGHT_SKY: Color = (63, 207, 255, 255);
pub const LIGHT_AZURE: Color = (63, 159, 255, 255);
pub const LIGHT_BLUE: Color = (63, 63, 255, 255);
pub const LIGHT_HAN: Color = (111, 63, 255, 255);
pub const LIGHT_VIOLET: Color = (159, 63, 255, 255);
pub const LIGHT_PURPLE: Color = (207, 63, 255, 255);
pub const LIGHT_FUCHSIA: Color = (255, 63, 255, 255);
pub const LIGHT_MAGENTA: Color = (255, 63, 207, 255);
pub const LIGHT_PINK: Color = (255, 63, 159, 255);
pub const LIGHT_CRIMSON: Color = (255, 63, 111, 255);
pub const RED: Color = (255, 0, 0, 255);
pub const FLAME: Color = (255, 63, 0, 255);
pub const ORANGE: Color = (255, 127, 0, 255);
pub const AMBER: Color = (255, 191, 0, 255);
pub const YELLOW: Color = (255, 255, 0, 255);
pub const LIME: Color = (191, 255, 0, 255);
pub const CHARTREUSE: Color = (127, 255, 0, 255);
pub const GREEN: Color = (0, 255, 0, 255);
pub const SEA: Color = (0, 255, 127, 255);
pub const TURQUOISE: Color = (0, 255, 191, 255);
pub const CYAN: Color = (0, 255, 255, 255);
pub const SKY: Color = (0, 191, 255, 255);
pub const AZURE: Color = (0, 127, 255, 255);
pub const BLUE: Color = (0, 0, 255, 255);
pub const HAN: Color = (63, 0, 255, 255);
pub const VIOLET: Color = (127, 0, 255, 255);
pub const PURPLE: Color = (191, 0, 255, 255);
pub const FUCHSIA: Color = (255, 0, 255, 255);
pub const MAGENTA: Color = (255, 0, 191, 255);
pub const PINK: Color = (255, 0, 127, 255);
pub const CRIMSON: Color = (255, 0, 63, 255);
pub const DARK_RED: Color = (191, 0, 0, 255);
pub const DARK_FLAME: Color = (191, 47, 0, 255);
pub const DARK_ORANGE: Color = (191, 95, 0, 255);
pub const DARK_AMBER: Color = (191, 143, 0, 255);
pub const DARK_YELLOW: Color = (191, 191, 0, 255);
pub const DARK_LIME: Color = (143, 191, 0, 255);
pub const DARK_CHARTREUSE: Color = (95, 191, 0, 255);
pub const DARK_GREEN: Color = (0, 191, 0, 255);
pub const DARK_SEA: Color = (0, 191, 95, 255);
pub const DARK_TURQUOISE: Color = (0, 191, 143, 255);
pub const DARK_CYAN: Color = (0, 191, 191, 255);
pub const DARK_SKY: Color = (0, 143, 191, 255);
pub const DARK_AZURE: Color = (0, 95, 191, 255);
pub const DARK_BLUE: Color = (0, 0, 191, 255);
pub const DARK_HAN: Color = (47, 0, 191, 255);
pub const DARK_VIOLET: Color = (95, 0, 191, 255);
pub const DARK_PURPLE: Color = (143, 0, 191, 255);
pub const DARK_FUCHSIA: Color = (191, 0, 191, 255);
pub const DARK_MAGENTA: Color = (191, 0, 143, 255);
pub const DARK_PINK: Color = (191, 0, 95, 255);
pub const DARK_CRIMSON: Color = (191, 0, 47, 255);
pub const DARKER_RED: Color = (127, 0, 0, 255);
pub const DARKER_FLAME: Color = (127, 31, 0, 255);
pub const DARKER_ORANGE: Color = (127, 63, 0, 255);
pub const DARKER_AMBER: Color = (127, 95, 0, 255);
pub const DARKER_YELLOW: Color = (127, 127, 0, 255);
pub const DARKER_LIME: Color = (95, 127, 0, 255);
pub const DARKER_CHARTREUSE: Color = (63, 127, 0, 255);
pub const DARKER_GREEN: Color = (0, 127, 0, 255);
pub const DARKER_SEA: Color = (0, 127, 63, 255);
pub const DARKER_TURQUOISE: Color = (0, 127, 95, 255);
pub const DARKER_CYAN: Color = (0, 127, 127, 255);
pub const DARKER_SKY: Color = (0, 95, 127, 255);
pub const DARKER_AZURE: Color = (0, 63, 127, 255);
pub const DARKER_BLUE: Color = (0, 0, 127, 255);
pub const DARKER_HAN: Color = (31, 0, 127, 255);
pub const DARKER_VIOLET: Color = (63, 0, 127, 255);
pub const DARKER_PURPLE: Color = (95, 0, 127, 255);
pub const DARKER_FUCHSIA: Color = (127, 0, 127, 255);
pub const DARKER_MAGENTA: Color = (127, 0, 95, 255);
pub const DARKER_PINK: Color = (127, 0, 63, 255);
pub const DARKER_CRIMSON: Color = (127, 0, 31, 255);
pub const DARKEST_RED: Color = (63, 0, 0, 255);
pub const DARKEST_FLAME: Color = (63, 15, 0, 255);
pub const DARKEST_ORANGE: Color = (63, 31, 0, 255);
pub const DARKEST_AMBER: Color = (63, 47, 0, 255);
pub const DARKEST_YELLOW: Color = (63, 63, 0, 255);
pub const DARKEST_LIME: Color = (47, 63, 0, 255);
pub const DARKEST_CHARTREUSE: Color = (31, 63, 0, 255);
pub const DARKEST_GREEN: Color = (0, 63, 0, 255);
pub const DARKEST_SEA: Color = (0, 63, 31, 255);
pub const DARKEST_TURQUOISE: Color = (0, 63, 47, 255);
pub const DARKEST_CYAN: Color = (0, 63, 63, 255);
pub const DARKEST_SKY: Color = (0, 47, 63, 255);
pub const DARKEST_AZURE: Color = (0, 31, 63, 255);
pub const DARKEST_BLUE: Color = (0, 0, 63, 255);
pub const DARKEST_HAN: Color = (15, 0, 63, 255);
pub const DARKEST_VIOLET: Color = (31, 0, 63, 255);
pub const DARKEST_PURPLE: Color = (47, 0, 63, 255);
pub const DARKEST_FUCHSIA: Color = (63, 0, 63, 255);
pub const DARKEST_MAGENTA: Color = (63, 0, 47, 255);
pub const DARKEST_PINK: Color = (63, 0, 31, 255);
pub const DARKEST_CRIMSON: Color = (63, 0, 15, 255);
pub const BRASS: Color = (191, 151, 96, 255);
pub const COPPER: Color = (197, 136, 124, 255);
pub const GOLD: Color = (229, 191, 0, 255);
pub const SILVER: Color = (203, 203, 203, 255);
pub const CELADON: Color = (172, 255, 175, 255);
pub const PEACH: Color = (255, 159, 127, 255);