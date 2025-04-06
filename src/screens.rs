use rkit::prelude::*;

#[derive(Screen, Clone, Copy, Debug, Hash, PartialEq, Eq)]
pub enum AppScreen {
    Load,
    Game,
}

pub fn screens_plugin(app: &mut App) {
    app.with_screen(AppScreen::Load)
        .add_plugin(load_screen::plugin)
        .add_plugin(game_screen::plugin);
}

mod game_screen {
    use std::ops::Deref;

    use rkit::{gfx::Color, prelude::*};

    use crate::{
        assets::Assets,
        camera::Cam,
        consts::{
            PICO8_BLACK, PICO8_BLUE, PICO8_BROWN, PICO8_DARK_PURPLE, PICO8_INDIGO, PICO8_ORANGE,
            PICO8_PEACH, PICO8_RED, PICO8_WHITE,
        },
        game::game_plugin,
        ui::{UIGameLayout, btns::UIImgButton, counter::create_img_counter, load_bar::UILoadBar},
    };

    use super::AppScreen;
    pub fn plugin(app: &mut App) {
        let screen = AppScreen::Game;
        app.add_plugin(UILayoutPlugin::<UIGameLayout>::default())
            .add_screen_systems(screen, OnUpdate, update_system)
            .add_systems(OnEnter(screen), setup_system)
            .add_systems(OnExit(screen), cleanup_system)
            .add_plugin(game_plugin);
    }

    #[derive(Debug, Component, Clone, Copy)]
    struct MoneyCounter;
    #[derive(Debug, Component, Clone, Copy)]
    struct CobberCounter;
    #[derive(Debug, Component, Clone, Copy)]
    struct IronCounter;
    #[derive(Debug, Component, Clone, Copy)]
    struct SilverCounter;
    #[derive(Debug, Component, Clone, Copy)]
    struct GoldCounter;
    #[derive(Debug, Component, Clone, Copy)]
    struct WoodCounter;
    #[derive(Debug, Component, Clone, Copy)]
    struct FoodCounter;
    #[derive(Debug, Component, Clone, Copy)]
    struct PeopleCounter;
    #[derive(Debug, Component, Clone, Copy)]
    struct RingCounter;

    fn setup_system(mut cmds: Commands, assets: Res<Assets>) {
        let layout = UIGameLayout;
        let root = cmds
            .spawn_ui_node(
                layout,
                (
                    UIContainer {
                        ..Default::default()
                    },
                    UIStyle::default()
                        .flex_col()
                        .size_full()
                        .justify_content_center()
                        .align_items_center(),
                ),
            )
            .entity_id();

        let top = cmds
            .spawn_ui_node(
                layout,
                (
                    UIContainer {
                        ..Default::default()
                    },
                    UIStyle::default()
                        .size_full()
                        .justify_content_space_between()
                        // .justify_content_center()
                        .align_items_center(),
                ),
            )
            .entity_id();
        cmds.add_ui_child(layout, root, top);

        let counters_container = cmds
            .spawn_ui_node(
                layout,
                (
                    UIContainer {
                        // bg_color: Some(PICO8_RED),
                        ..Default::default()
                    },
                    UIStyle::default()
                        .width(Unit::Relative(0.2))
                        .max_width(200.0)
                        .align_self_start()
                        .flex_col()
                        .gap_y(1.0)
                        .padding_top(4.0)
                        .padding_left(4.0),
                ),
            )
            .entity_id();

        cmds.add_ui_child(layout, top, counters_container);

        {
            let counter = create_img_counter(
                &mut cmds,
                layout,
                &assets.cobber,
                CobberCounter,
                PICO8_BLACK,
            );

            cmds.add_ui_child(layout, counters_container, counter);

            let counter =
                create_img_counter(&mut cmds, layout, &assets.iron, IronCounter, PICO8_BLACK);

            cmds.add_ui_child(layout, counters_container, counter);

            let counter = create_img_counter(
                &mut cmds,
                layout,
                &assets.silver,
                SilverCounter,
                PICO8_BLACK,
            );

            cmds.add_ui_child(layout, counters_container, counter);

            let counter =
                create_img_counter(&mut cmds, layout, &assets.gold, GoldCounter, PICO8_BLACK);

            cmds.add_ui_child(layout, counters_container, counter);

            let counter =
                create_img_counter(&mut cmds, layout, &assets.wood, WoodCounter, PICO8_BLACK);

            cmds.add_ui_child(layout, counters_container, counter);

            let counter =
                create_img_counter(&mut cmds, layout, &assets.food, FoodCounter, PICO8_BLACK);

            cmds.add_ui_child(layout, counters_container, counter);

            let counter = create_img_counter(
                &mut cmds,
                layout,
                &assets.people,
                PeopleCounter,
                PICO8_BLACK,
            );

            cmds.add_ui_child(layout, counters_container, counter);

            let counter =
                create_img_counter(&mut cmds, layout, &assets.ring, RingCounter, PICO8_BLACK);

            cmds.add_ui_child(layout, counters_container, counter);
        }

        let money_container = cmds
            .spawn_ui_node(
                layout,
                (
                    UIContainer {
                        ..Default::default()
                    },
                    UIStyle::default()
                        .width(Unit::Relative(0.2))
                        .padding_top(4.0)
                        .justify_content_center()
                        .align_self_start(), // .align_items_center(),
                ),
            )
            .entity_id();
        cmds.add_ui_child(layout, top, money_container);

        let counter = create_img_counter(
            &mut cmds,
            layout,
            &assets.money,
            MoneyCounter,
            PICO8_DARK_PURPLE,
        );

        cmds.add_ui_child(layout, money_container, counter);

        let options_container = cmds
            .spawn_ui_node(
                layout,
                (
                    UIContainer {
                        bg_color: Some(PICO8_RED),
                        ..Default::default()
                    },
                    UIStyle::default().width(Unit::Relative(0.2)),
                ),
            )
            .entity_id();
        cmds.add_ui_child(layout, top, options_container);

        let bottom = cmds
            .spawn_ui_node(
                layout,
                (
                    UIContainer {
                        // bg_color: Some(Color::GREEN),
                        ..Default::default()
                    },
                    UIStyle::default()
                        .flex_row()
                        .gap_x(8.0)
                        .size_full()
                        .justify_content_center()
                        .align_items_end()
                        .padding_bottom(4.0),
                ),
            )
            .entity_id();
        cmds.add_ui_child(layout, root, bottom);

        {
            let btn1 = cmds
                .spawn_ui_node(
                    layout,
                    (
                        UIImgButton {
                            sprite: assets.land.clone(),
                            text: "Land".to_string(),
                            enabled: false,
                        },
                        UIStyle::default().size(32.0, 32.0),
                    ),
                )
                .entity_id();

            cmds.add_ui_child(layout, bottom, btn1);

            let btn2 = cmds
                .spawn_ui_node(
                    layout,
                    (
                        UIImgButton {
                            sprite: assets.mine.clone(),
                            text: "Mine".to_string(),
                            enabled: false,
                        },
                        UIStyle::default().size(32.0, 32.0),
                    ),
                )
                .entity_id();

            cmds.add_ui_child(layout, bottom, btn2);

            let btn3 = cmds
                .spawn_ui_node(
                    layout,
                    (
                        UIImgButton {
                            sprite: assets.shop.clone(),
                            text: "Shop".to_string(),
                            enabled: false,
                        },
                        UIStyle::default().size(32.0, 32.0),
                    ),
                )
                .entity_id();

            cmds.add_ui_child(layout, bottom, btn3);

            let btn4 = cmds
                .spawn_ui_node(
                    layout,
                    (
                        UIImgButton {
                            sprite: assets.factory.clone(),
                            text: "Factory".to_string(),
                            enabled: false,
                        },
                        UIStyle::default().size(32.0, 32.0),
                    ),
                )
                .entity_id();

            cmds.add_ui_child(layout, bottom, btn4);

            let btn5 = cmds
                .spawn_ui_node(
                    layout,
                    (
                        UIImgButton {
                            sprite: assets.forest.clone(),
                            text: "Forest".to_string(),
                            enabled: true,
                        },
                        UIStyle::default().size(32.0, 32.0),
                    ),
                )
                .entity_id();

            cmds.add_ui_child(layout, bottom, btn5);

            let btn6 = cmds
                .spawn_ui_node(
                    layout,
                    (
                        UIImgButton {
                            sprite: assets.house.clone(),
                            text: "House".to_string(),
                            enabled: false,
                        },
                        UIStyle::default().size(32.0, 32.0),
                    ),
                )
                .entity_id();

            cmds.add_ui_child(layout, bottom, btn6);

            let btn7 = cmds
                .spawn_ui_node(
                    layout,
                    (
                        UIImgButton {
                            sprite: assets.farm.clone(),
                            text: "Farm".to_string(),
                            enabled: false,
                        },
                        UIStyle::default().size(32.0, 32.0),
                    ),
                )
                .entity_id();

            cmds.add_ui_child(layout, bottom, btn7);
        }
    }

    fn cleanup_system(mut cmds: Commands, ui_nodes: Query<Entity, With<UIGameLayout>>) {
        // clean ui nodes
        ui_nodes
            .iter()
            .for_each(|e| cmds.despawn_ui_node(UIGameLayout, e));
    }

    fn update_system(cam: Single<&Cam>, mut layout: ResMut<UILayout<UIGameLayout>>) {
        layout.set_camera(cam.into_inner().deref());
    }
}

mod load_screen {
    use std::ops::Deref;

    use rkit::{draw::create_draw_2d, math::vec2, prelude::*};

    use crate::{
        assets::{AssetLoader, init_assets},
        camera::Cam,
        consts::*,
        postfx::rtf,
        ui::{UILoadLayout, load_bar::UILoadBar},
    };

    use super::AppScreen;

    #[derive(Resource, Clone, Copy, Default)]
    struct AssetLoadTime(f32);

    pub fn plugin(app: &mut App) {
        let screen = AppScreen::Load;

        init_assets(screen, app)
            .add_resource(AssetLoadTime::default())
            .add_plugin(UILayoutPlugin::<UILoadLayout>::default())
            .add_screen_systems(
                screen,
                OnUpdate,
                (update_system, transition_to_game).chain(),
            )
            .add_screen_systems(screen, OnRender, draw_system)
            .add_systems(OnEnter(screen), setup_system)
            .add_systems(OnExit(screen), cleanup_system);
    }

    fn setup_system(mut cmds: Commands) {
        cmds.spawn_ui_node(
            UILoadLayout,
            (
                UIContainer {
                    bg_color: Some(PICO8_WHITE),
                    ..Default::default()
                },
                UIStyle::default()
                    .size_full()
                    .align_items_center()
                    .justify_content_center(),
            ),
        )
        .with_children(|cmd| {
            cmd.add((UILoadBar::default(), UIStyle::default().size(240.0, 24.0)));
        });
    }

    fn update_system(
        cam: Single<&Cam>,
        load_bar: Single<&mut UILoadBar, With<UILoadLayout>>,
        loader: Option<Res<AssetLoader>>,
        time: Res<Time>,
        mut load_time: ResMut<AssetLoadTime>,
        mut layout: ResMut<UILayout<UILoadLayout>>,
    ) {
        let ui_cam = cam.into_inner();
        layout.set_camera(&ui_cam.cam);

        let loader_progress = loader.map_or(1.0, |l| l.progress());
        let mut load_bar = load_bar.into_inner();
        load_bar.progress = loader_progress.min(load_time.0 / LOAD_MIN_TIME);

        load_time.0 += time.delta_f32();
    }

    fn transition_to_game(
        mut cmds: Commands,
        load_bar: Single<(&UILoadBar, &mut UIStyle), With<UILoadLayout>>,
        mouse: Res<Mouse>,
        keyboard: Res<Keyboard>,
    ) {
        let (load_bar, mut style) = load_bar.into_inner();
        let is_loading = load_bar.progress < 1.0;
        if is_loading {
            return;
        }

        #[cfg(not(feature = "final"))]
        {
            cmds.queue(ChangeScreen(AppScreen::Game));
        }

        *style = style.hide();
        let mouse_interaction = !mouse.pressed_buttons().is_empty();
        let keyboard_interaction = !keyboard.pressed_keys().is_empty();
        let did_interact = mouse_interaction || keyboard_interaction;
        if did_interact {
            cmds.queue(ChangeScreen(AppScreen::Game));
        }
    }

    fn cleanup_system(mut cmds: Commands, ui_nodes: Query<Entity, With<UILoadLayout>>) {
        cmds.remove_resource::<AssetLoadTime>();

        // clean ui nodes
        ui_nodes
            .iter()
            .for_each(|e| cmds.despawn_ui_node(UILoadLayout, e));

        // remove load layout because it will never used again
        cmds.remove_resource::<UILayout<UILoadLayout>>();
    }

    fn draw_system(world: &mut World) {
        let cam = world.query::<&Cam>().single(world);
        let mut draw = create_draw_2d();
        draw.set_round_pixels(true);
        draw.set_camera(cam.deref());
        draw.clear(PICO8_DARK_GRAY);
        draw_ui_layout::<UILoadLayout>(&mut draw, world);

        {
            let load_bar = world
                .query_filtered::<&UILoadBar, With<UILoadLayout>>()
                .single(world);
            let is_loaded = load_bar.progress >= 1.0;
            if is_loaded {
                let t = world.resource::<Time>().elapsed_f32();
                let alpha = 0.5 + 0.5 * (t * 2.0).sin();
                draw.text(TITLE)
                    .origin(vec2(0.5, 1.0))
                    .translate(RESOLUTION * 0.5)
                    .max_width(RESOLUTION.x * 0.8)
                    .h_align_center()
                    .color(PICO8_BLACK)
                    .size(24.0);

                draw.text("Click or press to start")
                    .alpha(alpha)
                    .origin(vec2(0.5, 0.0))
                    .translate(RESOLUTION * 0.5 + vec2(0.0, 24.0))
                    .max_width(RESOLUTION.x * 0.8)
                    .h_align_center()
                    .color(PICO8_INDIGO)
                    .size(12.0);
            }
        }

        rtf(&draw).unwrap();
    }
}
