use rkit::prelude::*;

#[derive(Screen, Clone, Copy, Debug, Hash, PartialEq, Eq)]
pub enum AppScreen {
    Load,
    Game,
}

pub fn screens_plugin(app: &mut App) {
    app.with_screen(AppScreen::Load)
        .add_plugin(load_screen::plugin);
}

mod load_screen {
    use std::ops::Deref;

    use rkit::{draw::create_draw_2d, gfx, math::Vec2, prelude::*};

    use crate::{
        assets::{AssetLoader, init_assets},
        camera::Cam,
        consts::*,
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
            cmd.add((UILoadBar::default(), UIStyle::default().size(300.0, 30.0)));
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
        load_bar: Single<&UILoadBar, With<UILoadLayout>>,
        mouse: Res<Mouse>,
        keyboard: Res<Keyboard>,
    ) {
        let is_loading = load_bar.progress < 1.0;
        if is_loading {
            return;
        }

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
                draw.text("Tap or press anything to play")
                    .alpha(alpha)
                    .origin(Vec2::splat(0.5))
                    .translate(RESOLUTION * 0.5 + Vec2::Y * 100.0)
                    .max_width(RESOLUTION.x * 0.8)
                    .h_align_center()
                    .color(PICO8_BLACK)
                    .size(32.0);
            }
        }

        gfx::render_to_frame(&draw).unwrap();
    }
}
