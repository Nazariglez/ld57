use std::ops::Deref;

use rkit::{
    draw::{Draw2D, create_draw_2d},
    math::{UVec2, Vec2},
    prelude::*,
};

use crate::{
    assets::Assets,
    camera::{Cam, GameCam, UICam},
    components::Pos,
    consts::*,
    game::{BuildKind, Building, Land},
    postfx::rtf,
    screens::AppScreen,
    ui::UIGameLayout,
};

pub fn render_plugin(app: &mut App) {
    app.add_systems(OnSetup, init_resources_system)
        .add_screen_systems(AppScreen::Game, OnRender, (render, draw_ui_system).chain());
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

fn draw_ui_system(world: &mut World) {
    let mut draw = create_draw_2d();
    draw.set_round_pixels(true);
    {
        let cam = world.query_filtered::<&Cam, With<UICam>>().single(world);
        draw.set_camera(cam.deref());
    }
    draw_ui_layout::<UIGameLayout>(&mut draw, world);

    rtf(&draw).unwrap();
}

// -- render systems
fn draw_land_layer_system(
    mut draw: InMut<Draw2D>,
    lands: Query<(&Land, &Pos)>,
    buildings: Query<(&BuildKind, &Building)>,
    cam: Single<&Cam, With<GameCam>>,
    assets: Res<Assets>,
) {
    draw.set_camera(cam.into_inner().deref());
    draw.clear(PICO8_BLACK);

    let tile_with_gap = TILE_SIZE + TILE_GAP;
    lands.iter().for_each(|(land, pos)| {
        let relative_pos = pos.0 - LAND_GAP;

        // outline
        let is_hover = land.hover.is_some();
        let stroke_width = 2.0;
        let stroke_color = if is_hover {
            PICO8_INDIGO
        } else {
            PICO8_DARK_GRAY
        };
        draw.rect(
            Vec2::ZERO, // - LAND_GAP - stroke_width * 0.5,
            LAND_SIZE * tile_with_gap + LAND_GAP * 2.0,
        )
        .alpha(0.4)
        .origin(Vec2::splat(0.5))
        .translate(relative_pos - stroke_width * 0.5)
        .stroke_color(stroke_color)
        .stroke(stroke_width);

        // inner grid
        let UVec2 { x: cols, y: rows } = LAND_SIZE.as_uvec2();
        for y in 0..rows {
            for x in 0..cols {
                let tile = UVec2::new(x, y);
                let tile_f32 = tile.as_vec2();
                let tile_pos =
                    relative_pos + (tile_f32 * tile_with_gap - (LAND_SIZE * tile_with_gap * 0.5));

                draw.image(&assets.dotted_square)
                    .translate(tile_pos)
                    .alpha(0.02);

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

        // draw buildings
        land.buildings.iter().for_each(|entity| {
            let Ok((kind, building)) = buildings.get(*entity) else {
                return;
            };

            let tile_f32 = building.pos.as_vec2();
            let tile_pos =
                relative_pos + (tile_f32 * tile_with_gap - (LAND_SIZE * tile_with_gap * 0.5));
            let img = match kind {
                BuildKind::Farm => &assets.farm,
                BuildKind::House => &assets.house,
                BuildKind::Forest => &assets.forest,
                BuildKind::Factory => &assets.factory,
                BuildKind::Shop => &assets.shop,
                BuildKind::Mine => &assets.mine,
            };
            draw.image(img).translate(tile_pos);
        });

        // draw overlay
        for y in 0..rows {
            for x in 0..cols {
                let tile = UVec2::new(x, y);
                let tile_f32 = tile.as_vec2();
                let tile_pos =
                    relative_pos + (tile_f32 * tile_with_gap - (LAND_SIZE * tile_with_gap * 0.5));

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
