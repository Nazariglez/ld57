use std::usize;

use rkit::{
    math::{Rect, UVec2, Vec2},
    prelude::*,
    random,
};
use strum::IntoEnumIterator;
use strum_macros::EnumIter;

use crate::{camera::Cam, components::Pos, consts::*, screens::AppScreen};

pub fn game_plugin(app: &mut App) {
    let screen = AppScreen::Game;
    app.add_systems(OnEnter(screen), init_game_resources_system)
        .add_screen_systems(screen, OnUpdate, find_focus_system);
}

#[derive(Clone, Copy, Default, EnumIter)]
pub enum Building {
    #[default]
    Empty,
    Farm,
    House,
    Forest,
    Factory,
    Shop,
    Mine,
}

#[derive(Component)]
pub struct Land {
    pub buildings: Vec<Building>,
    pub hover: Option<UVec2>,
    pub focus: Option<UVec2>,
}

impl Default for Land {
    fn default() -> Self {
        let tiles = LAND_SIZE.as_uvec2().element_product() as usize;
        let buildings = (0..tiles)
            .map(|_| random::pick(Building::iter()).unwrap())
            .collect::<Vec<_>>();
        Self {
            buildings,
            hover: Default::default(),
            focus: Default::default(),
        }
    }
}

impl Land {
    pub fn bounds(&self, pos: Vec2) -> Rect {
        let size = LAND_SIZE * (TILE_SIZE + TILE_GAP);
        Rect::new(pos - size * 0.5, size)
    }
}

fn init_game_resources_system(mut cmds: Commands) {
    cmds.spawn((Pos(RESOLUTION * 0.5), Land::default()));
}

fn find_focus_system(mut lands: Query<(&mut Land, &Pos)>, mouse: Res<Mouse>, cam: Single<&Cam>) {
    let local_pos = cam.screen_to_local(mouse.position());
    lands.iter_mut().for_each(|(mut land, pos)| {
        let bounds = land.bounds(pos.0);
        let mouse_hover = bounds.contains(local_pos);
        if !mouse_hover {
            land.hover = None;
            return;
        }

        // set hover
        let relative_pos = local_pos - (bounds.min());
        let tile = (relative_pos / (TILE_SIZE + TILE_GAP)).as_uvec2();
        land.hover = Some(tile);

        if mouse.just_pressed(MouseButton::Left) {
            land.focus = Some(tile);
        }
    });
}
