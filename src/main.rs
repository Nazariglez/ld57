mod assets;
mod camera;
mod components;
mod consts;
mod render;
mod screens;
mod ui;

use camera::camera_plugin;
use consts::*;
use render::render_plugin;
use rkit::prelude::*;
use screens::screens_plugin;
use ui::ui_plugin;

pub fn main() -> Result<(), String> {
    App::new()
        // framework plugins
        .add_plugin(MainPlugins::default())
        .add_plugin(AudioPlugin)
        // app plugins
        .add_plugin(window_plugin())
        .add_plugin(logging_plugin())
        // game plugins
        .add_plugin(camera_plugin)
        .add_plugin(ui_plugin)
        .add_plugin(screens_plugin)
        .add_plugin(render_plugin)
        .run()
}

fn logging_plugin() -> LogPlugin {
    let is_beta = cfg!(not(feature = "final"));
    if is_beta {
        LogPlugin::info()
    } else {
        LogPlugin::default()
    }
}

fn window_plugin() -> WindowConfigPlugin {
    let size = RESOLUTION.as_uvec2();
    let plugin = WindowConfigPlugin::default()
        .title("Economic Depths")
        .max_fps(60)
        .vsync(true)
        .pixelated(true)
        .min_size(size.x, size.y)
        .size(size.x, size.y);

    // #[cfg(all(target_arch = "wasm32", debug_assertions, not(feature = "final")))]
    // let plugin = plugin.maximized(true);

    plugin
}
