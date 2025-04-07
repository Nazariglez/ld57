use rkit::{
    draw::Draw2D,
    gfx::Color,
    math::{Vec2, vec2},
    prelude::*,
};

use crate::consts::*;

#[derive(Component, Debug, Clone, Copy)]
#[require(UIStyle, UIRender(load_bar_renderer))]
pub struct UILoadBar {
    pub bg_color: Color,
    pub fill_color: Color,
    pub border_color: Color,
    pub border_width: f32,
    pub progress: f32,
}

impl Default for UILoadBar {
    fn default() -> Self {
        Self {
            bg_color: PICO8_LIGHT_GRAY,
            fill_color: PICO8_BLUE,
            border_color: PICO8_WHITE,
            border_width: 2.0,
            progress: 0.0,
        }
    }
}

fn load_bar_renderer() -> UIRender {
    UIRender::run::<(&UILoadBar, &UINode), _>(render_load_bar_sys)
}

fn render_load_bar_sys(draw: &mut Draw2D, (bar, node): (&UILoadBar, &UINode)) {
    let bar_size = node.size();

    draw.rect(Vec2::ZERO, bar_size)
        .fill_color(bar.bg_color)
        .fill();

    draw.rect(Vec2::ZERO, vec2(bar_size.x * bar.progress, bar_size.y))
        .fill_color(bar.fill_color)
        .fill();

    draw.rect(Vec2::ZERO, bar_size)
        .stroke_color(bar.border_color)
        .stroke(bar.border_width);
}
