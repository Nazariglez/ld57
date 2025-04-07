
use rkit::{
    draw::{Draw2D, Sprite},
    math::{Vec2, vec2},
    prelude::*,
};

use crate::consts::*;

#[derive(Component, Debug, Clone)]
#[require(UIStyle, UIRender(img_btn_render_component), UINodeType(|| UINodeType::Image))]
pub struct UIImgButton {
    pub sprite: Sprite,
    pub text: String,
    pub enabled: bool,
}

fn img_btn_render_component() -> UIRender {
    UIRender::run::<(&UIImgButton, &UINode), _>(render_img_btn)
}

fn render_img_btn(draw: &mut Draw2D, (btn, node): (&UIImgButton, &UINode)) {
    let scale = node.size() / btn.sprite.size();

    draw.image(&btn.sprite)
        .alpha(0.5)
        .scale(scale)
        .origin(Vec2::splat(0.5))
        .translate(node.size() * 0.5);

    draw.text(&btn.text)
        .translate(vec2(node.size().x * 0.5, -10.0))
        // .translate((node.size() * 0.5).floor())
        .color(PICO8_WHITE)
        .size(8.0)
        .scale(Vec2::splat(0.6))
        .h_align_center()
        .origin(vec2(0.5, 0.0));

    let color = if btn.enabled {
        PICO8_GREEN
    } else {
        PICO8_LIGHT_GRAY
    };

    draw.rect(Vec2::ZERO, node.size() + 2.0)
        .translate(node.size() * 0.5)
        .origin(Vec2::splat(0.5))
        .stroke_color(color)
        .stroke(2.0)
        .alpha(0.8);
}
