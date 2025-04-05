use rkit::gfx::Color;
use rkit::math::*;

// Color Palette https://lospec.com/palette-list/pico-8
pub const PICO8_BLACK: Color = Color::rgba_u8(0, 0, 0, 255); // #000000
pub const PICO8_DARK_BLUE: Color = Color::rgba_u8(29, 43, 83, 255); // #1D2B53
pub const PICO8_DARK_PURPLE: Color = Color::rgba_u8(126, 37, 83, 255); // #7E2553
pub const PICO8_DARK_GREEN: Color = Color::rgba_u8(0, 135, 81, 255); // #008751
pub const PICO8_BROWN: Color = Color::rgba_u8(171, 82, 54, 255); // #AB5236
pub const PICO8_DARK_GRAY: Color = Color::rgba_u8(95, 87, 79, 255); // #5F574F
pub const PICO8_LIGHT_GRAY: Color = Color::rgba_u8(194, 195, 199, 255); // #C2C3C7
pub const PICO8_WHITE: Color = Color::rgba_u8(255, 241, 232, 255); // #FFF1E8
pub const PICO8_RED: Color = Color::rgba_u8(255, 0, 77, 255); // #FF004D
pub const PICO8_ORANGE: Color = Color::rgba_u8(255, 163, 0, 255); // #FFA300
pub const PICO8_YELLOW: Color = Color::rgba_u8(255, 236, 39, 255); // #FFEC27
pub const PICO8_GREEN: Color = Color::rgba_u8(0, 228, 54, 255); // #00E436
pub const PICO8_BLUE: Color = Color::rgba_u8(41, 173, 255, 255); // #29ADFF
pub const PICO8_INDIGO: Color = Color::rgba_u8(131, 118, 156, 255); // #83769C
pub const PICO8_PINK: Color = Color::rgba_u8(255, 119, 168, 255); // #FF77A8
pub const PICO8_PEACH: Color = Color::rgba_u8(255, 204, 170, 255); // #FFCCAA

// Game's resolutions
pub const RESOLUTION: Vec2 = Vec2::new(640.0, 512.0);

// Minimum time to load the bar
pub const LOAD_MIN_TIME: f32 = 1.0;
