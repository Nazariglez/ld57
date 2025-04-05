use rkit::prelude::*;

#[derive(Component, Clone, Copy)]
pub struct UILoadLayout;

#[derive(Component, Clone, Copy)]
pub struct UIGameLayout;

pub fn ui_plugin(app: &mut App) {
    app.add_systems(OnUpdate, click::dispatch_on_click_system);
}

pub mod load_bar {
    use rkit::{
        draw::Draw2D,
        gfx::Color,
        math::{Vec2, vec2},
        prelude::*,
    };

    use crate::consts::*;

    #[derive(Component, Debug, Clone, Copy)]
    #[require(UIStyle, UIRender(load_bar_renderer))]
    pub struct UILoadBar {
        pub bg_color: Color,
        pub fill_color: Color,
        pub border_color: Color,
        pub border_width: f32,
        pub progress: f32,
    }

    impl Default for UILoadBar {
        fn default() -> Self {
            Self {
                bg_color: PICO8_LIGHT_GRAY,
                fill_color: PICO8_BLUE,
                border_color: PICO8_WHITE,
                border_width: 2.0,
                progress: 0.0,
            }
        }
    }

    fn load_bar_renderer() -> UIRender {
        UIRender::run::<(&UILoadBar, &UINode), _>(render_load_bar_sys)
    }

    fn render_load_bar_sys(draw: &mut Draw2D, (bar, node): (&UILoadBar, &UINode)) {
        let bar_size = node.size();

        draw.rect(Vec2::ZERO, bar_size)
            .fill_color(bar.bg_color)
            .fill();

        draw.rect(Vec2::ZERO, vec2(bar_size.x * bar.progress, bar_size.y))
            .fill_color(bar.fill_color)
            .fill();

        draw.rect(Vec2::ZERO, bar_size)
            .stroke_color(bar.border_color)
            .stroke(bar.border_width);
    }
}

pub mod click {
    use rkit::{
        ecs::bevy_ecs::system::{BoxedSystem, SystemId},
        prelude::*,
    };

    #[derive(Default)]
    enum SysState {
        #[default]
        Empty,
        Boxed(BoxedSystem<In<Entity>, ()>),
        Id(SystemId<In<Entity>, ()>),
    }

    #[derive(Component, Default)]
    struct ClickSysNeedsRegister;

    #[derive(Component)]
    #[require(ClickSysNeedsRegister)]
    pub struct UIOnClick {
        state: SysState,
    }

    impl UIOnClick {
        pub fn run<S, Marker>(system: S) -> Self
        where
            S: IntoSystem<In<Entity>, (), Marker> + 'static,
        {
            let sys = Box::new(IntoSystem::into_system(system));
            UIOnClick {
                state: SysState::Boxed(sys),
            }
        }
    }

    pub(super) fn dispatch_on_click_system(world: &mut World) {
        let mut register_query =
            world.query_filtered::<(Entity, &mut UIOnClick), With<ClickSysNeedsRegister>>();

        let mut to_register = vec![];
        register_query.iter_mut(world).for_each(|(entity, mut on)| {
            let sys = std::mem::take(&mut on.state);
            if let SysState::Boxed(sys) = sys {
                to_register.push((entity, sys));
            }
        });

        to_register.into_iter().for_each(|(entity, sys)| {
            let id = world.register_boxed_system(sys);
            let mut entity_ref = world.entity_mut(entity);
            let query = entity_ref
                .remove::<ClickSysNeedsRegister>()
                .get_mut::<UIOnClick>();
            if let Some(mut on) = query {
                on.state = SysState::Id(id);
            }
        });

        let systems = world
            .query_filtered::<(Entity, &UIOnClick, &UIPointer), Without<ClickSysNeedsRegister>>()
            .iter(world)
            .filter_map(|(entity, on, pointer)| {
                if pointer.just_clicked(MouseButton::Left) {
                    if let SysState::Id(id) = on.state {
                        return Some((entity, id));
                    }
                }

                None
            })
            .collect::<Vec<_>>();

        systems.into_iter().for_each(|(entity, sys)| {
            world
                .run_system_with_input(sys, entity)
                .or_panic("Running UIOnClick System");
        });
    }
}
