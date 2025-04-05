use rkit::{
    gfx::{AsRenderer, Color},
    math::Vec2,
    postfx::{self, BlurFx, BlurParams, CrtFx, CrtParams, PostFx, ShadowFx, ShadowParams},
    prelude::*,
};

use crate::screens::AppScreen;

// Render to PostFX Frame
#[inline]
pub fn rtf<R: AsRenderer>(renderer: &R) -> Result<(), String> {
    postfx::render_to_pfx_frame(renderer)
}

pub fn post_fx_plugin(app: &mut App) {
    app.add_systems(OnSetup, init_postfx_system)
        .add_systems(OnPreRender, update_postfx_system)
        .add_systems(OnPostRender, render_postfx_system);
}

#[derive(Resource)]
pub struct PostFxRender {
    pub crt_fx: CrtFx,
}

impl PostFxRender {
    fn new() -> Result<Self, String> {
        let crt_fx = CrtFx::new(CrtParams {
            scanline_count: 300.0,
            curvature_amount: 3.4,
            chromatic_aberration_amount: 0.001,
            scanline_intensity: 0.45,
            roll_line_offset: 2.0,
            roll_speed: 0.04,
            roll_height: 310.0,
            scale_factor: 0.999,
            vignette_width: 10.0,
        })?;
        Ok(Self { crt_fx })
    }

    pub fn update(&mut self) -> Result<(), String> {
        self.crt_fx.update()?;

        Ok(())
    }

    pub fn screen_effects(&self) -> [&dyn PostFx; 1] {
        [&self.crt_fx]
    }
}

// -- systems
fn init_postfx_system(mut cmds: Commands) {
    let pfx = PostFxRender::new().or_panic("Initiating PostFxRender");
    cmds.insert_resource(pfx);
}

fn update_postfx_system(mut pfx: ResMut<PostFxRender>) {
    pfx.update().or_panic("Updating PostFX");
}

fn render_postfx_system(pfx: ResMut<PostFxRender>) {
    postfx::present_pfx_frame(&pfx.screen_effects(), true, true).or_panic("Presenting PFX Frame");
}
