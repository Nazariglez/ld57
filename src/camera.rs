use rkit::{
    draw::{BaseCam2D, Camera2D, ScreenMode},
    math::{Rect, Vec2},
    prelude::*,
};

use crate::{components::Pos, consts::*};

pub fn camera_plugin(app: &mut App) {
    app.add_systems(OnSetup, init_cam_system)
        .add_systems(OnPreUpdate, update_camera_system)
        .add_systems(OnPreRender, update_camera_system);
}

#[derive(Component, Clone, Copy)]
pub struct GameCam;

#[derive(Component, Clone, Copy)]
pub struct UICam;

#[derive(Component, Deref)]
pub struct Cam {
    #[deref]
    pub cam: Camera2D,
    pub mouse_pos: Vec2,
}

impl Cam {
    fn new(size: Vec2, res: Vec2) -> Self {
        let mut cam = Camera2D::new(size, ScreenMode::AspectFit(res));
        cam.set_pixel_perfect(true);
        cam.update();

        Self {
            cam,
            mouse_pos: Vec2::ZERO,
        }
    }

    #[inline]
    pub fn bounds_with_offset(&self, offset: f32) -> Rect {
        let rect = self.bounds();
        Rect::new(rect.origin - offset, rect.size + offset * 2.0)
    }

    #[inline]
    pub fn set_resolution(&mut self, res: Vec2) {
        self.cam.set_screen_mode(ScreenMode::AspectFit(res));
        self.cam.update();
    }
}

fn init_cam_system(mut cmds: Commands, win: Res<Window>) {
    cmds.spawn((
        GameCam,
        Cam::new(win.size(), RESOLUTION),
        Pos(RESOLUTION * 0.5),
    ));

    cmds.spawn((
        UICam,
        Cam::new(win.size(), UI_RESOLUTION),
        Pos(UI_RESOLUTION * 0.5),
    ));
}

fn update_camera_system(
    mut cams_query: Query<(&mut Cam, &Pos)>,
    win: Res<Window>,
    mouse: Res<Mouse>,
) {
    for (mut cam, pos) in &mut cams_query {
        cam.mouse_pos = cam.screen_to_local(mouse.position());
        cam.set_size(win.size());
        cam.set_position(pos.0);
        cam.update();
    }
}
