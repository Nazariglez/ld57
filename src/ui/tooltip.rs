use rkit::{draw::*, math::Vec2, prelude::*};

use crate::{
    assets::{self, Assets},
    consts::*,
    game::ResourceKind,
};

pub fn tooltip_plugin(app: &mut App) {
    // TODO
}

#[derive(Component, Clone, Copy)]
pub struct TooltipContainer;

pub struct ResInfo {
    pub kind: ResourceKind,
    pub amount: f32,
}

pub fn create_btn_info_tooltip<L: Component + Copy>(
    cmds: &mut Commands,
    layout: L,
    name: &str,
    info: &[ResInfo],
    assets: &Assets,
    comp: impl Bundle,
    pos: Vec2,
) -> Entity {
    let container = cmds
        .spawn_ui_node(
            layout,
            (
                UIContainer {
                    bg_color: Some(PICO8_DARK_BLUE),
                    border_color: Some(PICO8_LIGHT_GRAY),
                    border_size: 1.0,
                },
                UIStyle::default()
                    .left(pos.x)
                    .top(pos.y)
                    .flex_col()
                    .padding(6.0)
                    .gap_y(2.0)
                    .opacity(1.0)
                    .align_content_center()
                    .justify_content_center(),
            ),
        )
        .entity_id();

    let title = cmds
        .spawn_ui_node(
            layout,
            UIText {
                text: name.to_string(),
                color: PICO8_WHITE,
                size: 8.0,
                h_align: HAlign::Center,
                ..Default::default()
            },
        )
        .entity_id();

    cmds.add_ui_child(layout, container, title);

    info.iter().for_each(|info| {
        let row_container = cmds
            .spawn_ui_node(
                layout,
                (
                    UIContainer {
                        bg_color: Some(PICO8_BLACK),
                        border_color: Some(PICO8_LIGHT_GRAY),
                        border_size: 1.0,
                    },
                    UIStyle::default()
                        .flex_row()
                        .gap_x(10.0)
                        .min_width(100.0)
                        .max_height(18.0)
                        .padding_left(4.0)
                        .padding_right(6.0)
                        .padding_y(2.0)
                        .justify_content_space_between()
                        .align_items_center(),
                ),
            )
            .entity_id();

        cmds.add_ui_child(layout, container, row_container);

        let img = match info.kind {
            ResourceKind::Copper => &assets.copper,
            ResourceKind::Iron => &assets.iron,
            ResourceKind::Silver => &assets.silver,
            ResourceKind::Gold => &assets.gold,
            ResourceKind::Woord => &assets.wood,
            ResourceKind::Food => &assets.food,
            ResourceKind::People => &assets.people,
            ResourceKind::Ring => &assets.ring,
        };

        let img_c = cmds
            .spawn_ui_node(
                layout,
                UIImage {
                    sprite: img.clone(),
                },
            )
            .entity_id();

        cmds.add_ui_child(layout, row_container, img_c);

        let (color, txt) = if info.amount == 0.0 {
            (PICO8_WHITE, format!("{:.0}/s", info.amount))
        } else if info.amount < 0.0 {
            (PICO8_RED, format!("{:.2}/s", info.amount))
        } else {
            (PICO8_GREEN, format!("+{:.2}/s", info.amount))
        };

        let txt_c = cmds
            .spawn_ui_node(
                layout,
                (UIText {
                    font: Some(assets.font.clone()),
                    text: txt,
                    color,
                    size: 12.0,
                    h_align: HAlign::Center,
                    ..Default::default()
                },),
            )
            .entity_id();

        cmds.add_ui_child(layout, row_container, txt_c);
    });

    container
}
