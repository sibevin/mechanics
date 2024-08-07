use crate::{app::theme, app::ui, book::page::*};
use bevy_alt_ui_navigation_lite::prelude::*;

pub mod audio;
pub mod main;
pub mod visual;

#[derive(Component, Debug)]
pub enum ButtonAction {
    Link(String),
    MoveToPage(PageState),
}

pub fn build_about_nav_bar(
    parent: &mut ChildBuilder,
    asset_server: &Res<AssetServer>,
    page: PageState,
) -> Entity {
    parent
        .spawn(NodeBundle {
            style: Style {
                position_type: PositionType::Absolute,
                bottom: Val::Px(0.0),
                left: Val::Px(0.0),
                right: Val::Px(0.0),
                align_items: AlignItems::End,
                justify_content: JustifyContent::SpaceBetween,
                ..default()
            },
            ..default()
        })
        .with_children(|parent| {
            ui::build_icon_btn(
                parent,
                &asset_server,
                (
                    ButtonAction::MoveToPage(PageState::Menu),
                    app::interaction::IaButton,
                    Focusable::default(),
                    app::interaction::IaDefaultFocus,
                ),
                Style::default(),
                "arrow-left-light_1.5x",
            );
            parent
                .spawn(NodeBundle {
                    style: Style {
                        flex_grow: 1.0,
                        align_items: AlignItems::End,
                        justify_content: JustifyContent::Center,
                        column_gap: ui::px_p(4.0),
                        margin: UiRect::right(ui::px_p(12.0)),
                        ..default()
                    },
                    ..default()
                })
                .with_children(|parent| {
                    parent
                        .spawn(NodeBundle {
                            style: Style {
                                flex_direction: FlexDirection::Column,
                                align_items: AlignItems::Center,
                                justify_content: JustifyContent::End,
                                row_gap: ui::px_p(1.0),
                                ..default()
                            },
                            ..default()
                        })
                        .with_children(|parent| {
                            if page == PageState::AboutMain {
                                build_current_tab(parent, &asset_server, "circuitry-light_1.5x");
                            } else {
                                ui::build_icon_btn(
                                    parent,
                                    &asset_server,
                                    (
                                        ButtonAction::MoveToPage(PageState::AboutMain),
                                        app::interaction::IaButton,
                                        Focusable::default(),
                                    ),
                                    Style::default(),
                                    "circuitry-light_1.5x",
                                );
                            }
                        });
                    parent
                        .spawn(NodeBundle {
                            style: Style {
                                flex_direction: FlexDirection::Column,
                                align_items: AlignItems::Center,
                                justify_content: JustifyContent::End,
                                row_gap: ui::px_p(1.0),
                                ..default()
                            },
                            ..default()
                        })
                        .with_children(|parent| {
                            if page == PageState::AboutAudio {
                                build_current_tab(
                                    parent,
                                    &asset_server,
                                    "cassette-tape-light_1.5x",
                                );
                            } else {
                                ui::build_icon_btn(
                                    parent,
                                    &asset_server,
                                    (
                                        ButtonAction::MoveToPage(PageState::AboutAudio),
                                        app::interaction::IaButton,
                                        Focusable::default(),
                                    ),
                                    Style::default(),
                                    "cassette-tape-light_1.5x",
                                );
                            }
                        });
                    parent
                        .spawn(NodeBundle {
                            style: Style {
                                flex_direction: FlexDirection::Column,
                                align_items: AlignItems::Center,
                                justify_content: JustifyContent::End,
                                row_gap: ui::px_p(1.0),
                                ..default()
                            },
                            ..default()
                        })
                        .with_children(|parent| {
                            if page == PageState::AboutVisual {
                                build_current_tab(parent, &asset_server, "paint-brush-light_1.5x");
                            } else {
                                ui::build_icon_btn(
                                    parent,
                                    &asset_server,
                                    (
                                        ButtonAction::MoveToPage(PageState::AboutVisual),
                                        app::interaction::IaButton,
                                        Focusable::default(),
                                    ),
                                    Style::default(),
                                    "paint-brush-light_1.5x",
                                );
                            }
                        });
                });
        })
        .id()
}

fn build_current_tab(parent: &mut ChildBuilder, asset_server: &Res<AssetServer>, tab_icon: &str) {
    parent.spawn(NodeBundle {
        style: Style {
            width: Val::Px(ui::ICON_SIZE * 0.35),
            height: Val::Px(ui::ICON_SIZE * 0.35),
            ..default()
        },
        background_color: theme::MUTE_COLOR.into(),
        border_radius: BorderRadius::MAX,
        ..default()
    });
    parent
        .spawn((NodeBundle {
            style: Style {
                width: Val::Auto,
                height: Val::Auto,
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                padding: UiRect::new(
                    ui::px_p(ui::BTN_PADDING * 0.6),
                    ui::px_p(ui::BTN_PADDING * 0.6),
                    ui::px_p(ui::BTN_PADDING * 0.3),
                    ui::px_p(ui::BTN_PADDING * 0.6),
                ),
                ..default()
            },
            background_color: theme::BTN_BG.into(),
            ..default()
        },))
        .with_children(|parent| {
            let icon_path = format!("images/icons/{}.png", tab_icon);
            let icon = asset_server.load(icon_path);
            parent.spawn(ImageBundle {
                style: Style {
                    width: Val::Px(ui::ICON_SIZE * 1.5),
                    height: Val::Px(ui::ICON_SIZE * 1.5),
                    ..default()
                },
                image: UiImage::new(icon),
                ..default()
            });
        });
}
