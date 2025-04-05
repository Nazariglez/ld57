use std::ops::Deref;

use rkit::{
    draw::{Draw2D, create_draw_2d},
    math::{UVec2, Vec2},
    prelude::*,
};

use crate::{assets::Assets, camera::Cam, consts::*, postfx::rtf, screens::AppScreen};

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
    let mut systems = vec![cmds.register_system(draw_initial_layer_system)];

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
fn draw_initial_layer_system(mut draw: InMut<Draw2D>, cam: Single<&Cam>, assets: Res<Assets>) {
    draw.set_camera(cam.into_inner().deref());
    draw.clear(PICO8_BLACK);

    let tile_with_gap = TILE_SIZE + TILE_GAP;
    let pos = RESOLUTION * 0.5;
    draw.rect(Vec2::ZERO, LAND_SIZE * tile_with_gap)
        .origin(Vec2::splat(0.5))
        .translate(pos)
        .stroke_color(PICO8_INDIGO)
        .stroke(2.0);

    // Draw empty square for each tile in the grid
    let UVec2 { x: cols, y: rows } = LAND_SIZE.as_uvec2();
    for y in 0..rows {
        for x in 0..cols {
            let tile = UVec2::new(x, y).as_vec2();
            let tile_pos =
                tile * tile_with_gap - (LAND_SIZE * tile_with_gap * 0.5) + TILE_SIZE * 0.5;
            draw.image(&assets.empty_square)
                .translate(pos + tile_pos - TILE_SIZE * 0.5)
                .alpha(0.4);
        }
    }
}
