use std::collections::HashSet;

use rkit::{
    math::{Rect, UVec2, Vec2},
    prelude::*,
};
use strum_macros::EnumIter;

use crate::{camera::Cam, components::Pos, consts::*, screens::AppScreen};

pub fn game_plugin(app: &mut App) {
    let screen = AppScreen::Game;
    app.add_systems(OnEnter(screen), init_game_resources_system)
        .add_screen_systems(screen, OnUpdate, find_focus_system)
        .add_screen_systems(screen, OnPostUpdate, on_added_building_system);
}

#[derive(Component, Clone, Copy)]
pub struct Building {
    pub land: Entity,
    pub pos: UVec2,
}

#[derive(Component, Clone, Copy, EnumIter)]
pub enum BuildKind {
    Farm,
    House,
    Forest,
    Factory,
    Shop,
    Mine,
}

#[derive(Component, Default)]
pub struct Land {
    pub buildings: HashSet<Entity>,
    pub hover: Option<UVec2>,
    pub focus: Option<UVec2>,
}

impl Land {
    pub fn bounds(&self, pos: Vec2) -> Rect {
        let size = LAND_SIZE * (TILE_SIZE + TILE_GAP);
        Rect::new(pos - size * 0.5, size)
    }

    pub fn add(&mut self, building: Entity) {
        self.buildings.insert(building);
    }

    pub fn remove(&mut self, building: &Entity) {
        self.buildings.remove(building);
    }
}

fn init_game_resources_system(mut cmds: Commands) {
    let land_e = cmds.spawn((Pos(RESOLUTION * 0.5), Land::default())).id();
    cmds.spawn((
        Building {
            land: land_e,
            pos: UVec2::new(1, 1),
        },
        BuildKind::Mine,
    ));
}

fn on_added_building_system(
    mut lands: Query<&mut Land>,
    buildings: Query<(Entity, &Building), Added<Building>>,
) {
    buildings.iter().for_each(|(entity, building)| {
        let Ok(mut land) = lands.get_mut(building.land) else {
            return;
        };

        land.add(entity);
    });
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
