use bevy::prelude::*;
use bevy_alt_ui_navigation_lite::prelude::*;
use bevy_prototype_lyon::prelude::*;
use bevy_tweening::*;

use mechanics::*;

fn main() {
    App::new()
        .add_plugins((
            app::AppPlugin,
            ShapePlugin,
            DefaultNavigationPlugins,
            TweeningPlugin,
            book::BookPlugin,
            game::GamePlugin,
        ))
        .run();
}
