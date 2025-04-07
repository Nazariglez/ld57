use rkit::{
    draw::{HAlign, Sprite},
    gfx::Color,
    math::{Vec2, vec2},
    prelude::*,
    random,
};

use crate::{assets::Assets, consts::*, game::ResourceKind};

use super::{
    UIGameLayout,
    tooltip::{ResInfo, TooltipContainer, create_btn_info_tooltip},
};

#[derive(Component, Clone, Copy)]
pub struct CounterInfo;

pub fn create_img_counter<C: Component, L: Component + Copy>(
    cmds: &mut Commands,
    layout: L,
    img: &Sprite,
    text_marker: C,
    color: Color,
    b: impl Bundle,
) -> Entity {
    let container = cmds
        .spawn_ui_node(
            layout,
            (
                UIContainer {
                    bg_color: Some(color),
                    border_color: Some(PICO8_LIGHT_GRAY),
                    border_size: 1.0,
                },
                UIStyle::default()
                    .flex_row()
                    .gap_x(10.0)
                    .max_height(18.0)
                    // .min_width(100.0)
                    .padding_left(4.0)
                    .padding_right(6.0)
                    .padding_y(2.0)
                    .justify_content_space_between()
                    // .align_self_center()
                    .align_items_center(),
                b,
            ),
        )
        .entity_id();

    let img_c = cmds
        .spawn_ui_node(
            layout,
            UIImage {
                sprite: img.clone(),
            },
        )
        .entity_id();

    cmds.add_ui_child(layout, container, img_c);

    let txt_c = cmds
        .spawn_ui_node(
            layout,
            (
                text_marker,
                UIText {
                    text: "0".to_string(),
                    color: PICO8_WHITE,
                    size: 8.0,
                    h_align: HAlign::Center,
                    ..Default::default()
                },
            ),
        )
        .entity_id();

    cmds.add_ui_child(layout, container, txt_c);

    container
}

pub(super) fn show_counter_info_system(
    mut cmds: Commands,
    query: Query<(&UIPointer, &UINode), With<CounterInfo>>,
    tooltip_container: Single<Entity, With<TooltipContainer>>,
    assets: Res<Assets>,
) {
    let layout = UIGameLayout;
    let tooltip_container = tooltip_container.into_inner();
    query.iter().for_each(|(pointer, node)| {
        if pointer.just_enter() {
            let pos = node.position() + node.size() * Vec2::X;
            let tooltip = create_btn_info_tooltip(
                &mut cmds,
                layout,
                "whatever",
                &[ResInfo {
                    kind: ResourceKind::Iron,
                    amount: 0.0,
                }],
                assets.as_ref(),
                (),
                pos,
            );
            cmds.add_ui_child(layout, tooltip_container, tooltip);
        } else if pointer.just_exit() {
        }
    });
}
