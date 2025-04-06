use std::ops::Deref;

use rkit::{
    draw::{Draw2D, create_draw_2d},
    math::{UVec2, Vec2},
    prelude::*,
};

use crate::{
    assets::Assets,
    camera::Cam,
    components::Pos,
    consts::*,
    game::{Building, Land},
    postfx::rtf,
    screens::AppScreen,
};

pub fn render_plugin(app: &mut App) {
    app.add_systems(OnSetup, init_resources_system)
        .add_screen_systems(AppScreen::Game, OnRender, render);
}

// - pipeline systems
pub type RenderSysFn =
    Box<dyn Fn(&mut World, &mut Draw2D) -> Result<(), String> + Send + Sync + 'static>;

#[derive(Resource)]
pub struct RenderGameSys {
    pub callback: RenderSysFn,
}

fn init_resources_system(mut cmds: Commands) {
    let callback = create_render_wrapper(&mut cmds);
    cmds.insert_resource(RenderGameSys { callback });
}

// generate the render systems pipeline
fn create_render_wrapper(cmds: &mut Commands) -> RenderSysFn {
    let mut systems = vec![cmds.register_system(draw_land_layer_system)];

    #[cfg(debug_assertions)]
    {
        systems.append(&mut vec![]);
    }

    Box::new(move |world: &mut World, draw: &mut Draw2D| {
        for sys_id in &systems {
            world
                .run_system_with_input(*sys_id, draw)
                .map_err(|err| err.to_string())?;
        }
        Ok(())
    })
}

fn render(world: &mut World) {
    let mut draw = create_draw_2d();

    world.resource_scope(|world, r_sys: Mut<RenderGameSys>| {
        (r_sys.callback)(world, &mut draw).unwrap();
    });

    rtf(&draw).unwrap();
}

// -- render systems
fn draw_land_layer_system(
    mut draw: InMut<Draw2D>,
    lands: Query<(&Land, &Pos)>,
    cam: Single<&Cam>,
    assets: Res<Assets>,
) {
    draw.set_camera(cam.into_inner().deref());
    draw.clear(PICO8_BLACK);

    let tile_with_gap = TILE_SIZE + TILE_GAP;
    lands.iter().for_each(|(land, pos)| {
        // outline
        let is_hover = land.hover.is_some();
        let stroke_width = 2.0;
        let stroke_color = if is_hover {
            PICO8_INDIGO
        } else {
            PICO8_DARK_GRAY
        };
        draw.rect(
            Vec2::ZERO - LAND_GAP - stroke_width * 0.5,
            LAND_SIZE * tile_with_gap + LAND_GAP * 2.0,
        )
        .alpha(0.4)
        .origin(Vec2::splat(0.5))
        .translate(pos.0)
        .stroke_color(stroke_color)
        .stroke(stroke_width);

        // inner grid
        let UVec2 { x: cols, y: rows } = LAND_SIZE.as_uvec2();
        for y in 0..rows {
            for x in 0..cols {
                let pos = pos.0 - LAND_GAP;

                let tile = UVec2::new(x, y);
                let tile_f32 = tile.as_vec2();
                let tile_pos = pos + (tile_f32 * tile_with_gap - (LAND_SIZE * tile_with_gap * 0.5));

                let idx = (y * cols + x) as usize;
                let (img, alpha) = match land.buildings[idx] {
                    Building::Empty => (&assets.dotted_square, 0.02),
                    Building::Farm => (&assets.farm, 1.0),
                    Building::House => (&assets.house, 1.0),
                    Building::Forest => (&assets.forest, 1.0),
                    Building::Factory => (&assets.factory, 1.0),
                    Building::Shop => (&assets.shop, 1.0),
                    Building::Mine => (&assets.mine, 1.0),
                };

                draw.image(img).translate(tile_pos).alpha(alpha);

                if let Some(hover) = land.hover {
                    if hover == tile {
                        draw.image(&assets.empty_square)
                            .translate(tile_pos)
                            .alpha(0.1);
                    }
                }

                if let Some(focus) = land.focus {
                    if focus == tile {
                        draw.image(&assets.empty_square)
                            .translate(tile_pos)
                            .alpha(0.6);
                    }
                }
            }
        }
    });
}
