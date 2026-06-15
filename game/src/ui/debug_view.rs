use bevy::{color::palettes::css::GHOST_WHITE, prelude::*};

use crate::ui::{BlockPosInfo, BlockPowerInfo, Immediate, Scheduled, TickText};

pub fn debug_view_system(mut commands: Commands, asset_server: Res<AssetServer>) {
    let fonts: Handle<Font> = asset_server.load("fonts/retro_gaming.ttf");
    let root_uinode = commands
        .spawn((
            Node {
                width: percent(100),
                height: percent(100),
                justify_content: JustifyContent::SpaceBetween,
                ..default()
            },
            Pickable::IGNORE,
        ))
        .id();
    let left_column = commands
        .spawn((
            Node {
                flex_direction: FlexDirection::Column,
                justify_content: JustifyContent::FlexStart,
                align_items: AlignItems::Start,
                flex_grow: 1.,
                margin: UiRect::axes(px(15), px(5)),
                ..default()
            },
            Pickable::IGNORE,
        ))
        .with_children(|builder| {
            builder.spawn((
                Text::new("Ticks: "),
                TextFont {
                    font: fonts.clone(),
                    font_size: 17.0,
                    ..default()
                },
                TextColor(GHOST_WHITE.into()),
                children![(TextSpan::default(), TickText)],
            ));

            builder.spawn((
                Text::new("BlockInfo: "),
                TextFont {
                    font: fonts.clone(),
                    font_size: 17.0,
                    ..default()
                },
                TextColor(GHOST_WHITE.into()),
            ));

            builder.spawn((
                Text::default(),
                TextFont {
                    font: fonts.clone(),
                    font_size: 17.0,
                    ..default()
                },
                TextColor(GHOST_WHITE.into()),
                BlockPosInfo,
            ));

            builder.spawn((
                Text::new("Power: "),
                TextFont {
                    font: fonts.clone(),
                    font_size: 17.0,
                    ..default()
                },
                TextColor(GHOST_WHITE.into()),
                children![(TextSpan::new(""), BlockPowerInfo)],
            ));

            builder.spawn((
                Text::new("Scheduler: "),
                TextFont {
                    font: fonts.clone(),
                    font_size: 17.0,
                    ..default()
                },
                TextColor(GHOST_WHITE.into()),
            ));

            builder.spawn((
                Text::default(),
                TextFont {
                    font: fonts.clone(),
                    font_size: 17.0,
                    ..default()
                },
                TextColor(GHOST_WHITE.into()),
                Immediate,
            ));

            builder.spawn((
                Text::default(),
                TextFont {
                    font: fonts.clone(),
                    font_size: 17.0,
                    ..default()
                },
                TextColor(GHOST_WHITE.into()),
                Scheduled,
            ));
        })
        .id();

    commands.entity(root_uinode).add_child(left_column);
}
