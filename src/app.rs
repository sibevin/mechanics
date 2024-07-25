pub mod anime_effect;
pub mod audio;
pub mod cursor;
pub mod cursor_icon;
pub mod element;
pub mod interaction;
pub mod key_binding;
pub mod layer;
pub mod plugin;
pub mod screenshot;
pub mod settings;
pub mod startup;
pub mod theme;
pub mod timer;
pub mod ui;

pub use plugin::AppPlugin;
pub use startup::startup;

pub const WINDOW_W: f32 = 1280.0;
pub const WINDOW_H: f32 = 720.0;

pub const APP_CODE: &str = "mechanics";
pub const APP_NAME: &str = "Mechanics";
pub const APP_SLOGAN: &str = "A mechanics game about kinematics and dynamics.";
pub const APP_ITCH_URL: &str = "https://sibevin.itch.io/mechanics";
pub const APP_GITHUB_URL: &str = "https://github.com/sibevin/mechanics";
