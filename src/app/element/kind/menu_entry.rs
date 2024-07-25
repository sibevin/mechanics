use super::*;
use crate::app::{theme, ui};

const MENU_ENTRY_W: f32 = ui::FONT_SIZE * 6.5;
const MENU_ENTRY_RATIO: f32 = 1.2;

pub fn build_element(
    parent: &mut ChildBuilder,
    asset_server: &Res<AssetServer>,
    bundle: impl Bundle,
    icon: &str,
    text: &str,
) -> Entity {
    parent
        .spawn((
            ButtonBundle {
                style: Style {
                    width: Val::Px(MENU_ENTRY_W),
                    justify_content: JustifyContent::Start,
                    align_items: AlignItems::Center,
                    column_gap: ui::px_p(4.0),
                    padding: UiRect::all(ui::px_p(3.0)),
                    ..default()
                },
                background_color: theme::BTN_BG.into(),
                ..default()
            },
            bundle,
            ElementData::MenuEntry,
        ))
        .with_children(|parent| {
            let icon_path = format!("images/icons/{}.png", icon);
            let icon = asset_server.load(icon_path);
            parent.spawn(ImageBundle {
                style: Style {
                    width: Val::Px(ui::ICON_SIZE * MENU_ENTRY_RATIO),
                    height: Val::Px(ui::ICON_SIZE * MENU_ENTRY_RATIO),
                    margin: UiRect::right(ui::px_p(3.0)),
                    ..default()
                },
                image: UiImage::new(icon),
                ..default()
            });
            parent.spawn(
                TextBundle::from_section(
                    text,
                    TextStyle {
                        font: asset_server.load(theme::FONT),
                        font_size: ui::FONT_SIZE * MENU_ENTRY_RATIO,
                        color: theme::FG_COLOR,
                    },
                )
                .with_style(Style {
                    margin: UiRect::top(ui::px_p(2.0)),
                    ..default()
                }),
            );
        })
        .id()
}
