use super::*;
use crate::app;
use bevy_alt_ui_navigation_lite::NavRequestSystem;
use webbrowser;

const PAGE_CODE: &str = "about_main";
const PAGE_NAME: &str = "ABOUT";
const PAGE_ICON: &str = "star-light";

pub struct Page;

impl PageBase for Page {
    fn code(&self) -> &str {
        PAGE_CODE
    }
    fn name(&self) -> &str {
        PAGE_NAME
    }
    fn icon(&self) -> &str {
        PAGE_ICON
    }
    fn state(&self) -> PageState {
        PageState::AboutMain
    }
    fn build(&self, app: &mut App) {
        app.add_systems(
            OnEnter(self.state()),
            (app::interaction::reset_default_focus, page_enter),
        )
        .add_systems(
            Update,
            (
                handle_ui_navigation,
                handle_hidden_button_click,
                app::interaction::handle_default_focus,
            )
                .after(NavRequestSystem)
                .run_if(in_state(self.state())),
        )
        .add_systems(
            OnExit(self.state()),
            (
                app::anime_effect::clear_anime_effect,
                app::ui::despawn_ui::<OnPage>,
            ),
        );
    }
}

#[derive(Component)]
struct OnPage;

fn page_enter(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands
        .spawn((build_page_layout(), OnPage))
        .with_children(|parent| {
            parent
                .spawn(NodeBundle {
                    style: Style {
                        width: Val::Percent(100.0),
                        flex_direction: FlexDirection::Column,
                        align_items: AlignItems::Center,
                        justify_content: JustifyContent::SpaceBetween,
                        ..default()
                    },
                    ..default()
                })
                .with_children(|parent| {
                    build_game_title(parent, &asset_server);
                    build_page_title(parent, &asset_server, PAGE_NAME, PAGE_ICON);
                    parent
                        .spawn(NodeBundle {
                            style: Style {
                                flex_grow: 1.0,
                                flex_direction: FlexDirection::Row,
                                align_items: AlignItems::Center,
                                justify_content: JustifyContent::Center,
                                column_gap: app::ui::px_p(10.0),
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
                                        ..default()
                                    },
                                    ..default()
                                })
                                .with_children(|parent| {
                                    app::ui::build_link(
                                        parent,
                                        &asset_server,
                                        (
                                            Button,
                                            Interaction::default(),
                                            ButtonAction::MoveToPage(PageState::Dev),
                                        ),
                                        env!("CARGO_PKG_VERSION"),
                                        None,
                                        "default",
                                        false,
                                    );
                                    build_sep_title(parent, &asset_server, "Link", "link-bold");
                                    app::ui::build_link(
                                        parent,
                                        &asset_server,
                                        ButtonAction::Link(String::from(app::APP_ITCH_URL)),
                                        "itch.io",
                                        Some("house-line-light"),
                                        "default",
                                        true,
                                    );
                                    app::ui::build_link(
                                        parent,
                                        &asset_server,
                                        ButtonAction::Link(String::from(app::APP_GITHUB_URL)),
                                        "github.com",
                                        Some("github-logo-light"),
                                        "default",
                                        true,
                                    );
                                });
                            parent
                                .spawn(NodeBundle {
                                    style: Style {
                                        flex_direction: FlexDirection::Column,
                                        align_items: AlignItems::Center,
                                        ..default()
                                    },
                                    ..default()
                                })
                                .with_children(|parent| {
                                    build_sep_title(
                                        parent,
                                        &asset_server,
                                        "Design",
                                        "compass-tool-fill",
                                    );
                                    app::ui::build_link(
                                        parent,
                                        &asset_server,
                                        (),
                                        "Kait Wang",
                                        None,
                                        "default",
                                        false,
                                    );
                                    build_sep_title(
                                        parent,
                                        &asset_server,
                                        "Programming",
                                        "code-bold",
                                    );
                                    app::ui::build_link(
                                        parent,
                                        &asset_server,
                                        (),
                                        "Kait Wang",
                                        None,
                                        "default",
                                        false,
                                    );
                                    build_sep_title(parent, &asset_server, "Art", "palette-fill");
                                    app::ui::build_link(
                                        parent,
                                        &asset_server,
                                        (),
                                        "Miya",
                                        None,
                                        "default",
                                        false,
                                    );
                                });
                        });
                    build_about_nav_bar(parent, &asset_server, PageState::AboutMain);
                });
        });
}

fn handle_ui_navigation(
    mut actions: Query<&mut ButtonAction>,
    mut events: EventReader<NavEvent>,
    mut page_state: ResMut<NextState<PageState>>,
) {
    events.nav_iter().activated_in_query_foreach_mut(
        &mut actions,
        |mut action| match &mut *action {
            ButtonAction::Link(url) => {
                let _ = webbrowser::open(url);
            }
            ButtonAction::MoveToPage(state) => page_state.set(*state),
        },
    );
}

type InteractionButtonCondition = (Changed<Interaction>, With<Button>);

fn handle_hidden_button_click(
    mut interaction_query: Query<(&Interaction, &ButtonAction), InteractionButtonCondition>,
    mut page_state: ResMut<NextState<PageState>>,
) {
    for (interaction, action) in interaction_query.iter_mut() {
        if *interaction == Interaction::Pressed {
            if let ButtonAction::MoveToPage(state) = action {
                page_state.set(*state)
            };
        }
    }
}
