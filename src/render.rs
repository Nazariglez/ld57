use rkit::{
    draw::create_draw_2d,
    gfx::{self, Color},
    prelude::*,
};

pub fn render_plugin(app: &mut App) {
    app.add_systems(OnRender, render);
}

fn render() {
    let mut draw = create_draw_2d();
    draw.clear(Color::BLACK);
    gfx::render_to_frame(&draw).unwrap();
}
