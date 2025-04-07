pub mod btns;
pub mod click;
pub mod counter;
pub mod load_bar;
pub mod tooltip;

use rkit::prelude::*;

use crate::screens::AppScreen;

#[derive(Component, Clone, Copy)]
pub struct UILoadLayout;

#[derive(Component, Clone, Copy)]
pub struct UIGameLayout;

pub fn ui_plugin(app: &mut App) {
    app.add_systems(OnUpdate, (click::dispatch_on_click_system,))
        .add_screen_systems(AppScreen::Game, OnUpdate, counter::show_counter_info_system);
}
