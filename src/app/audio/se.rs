use super::*;
use phf::phf_map;

struct AudioData {
    pub volume_bias: f32,
    pub path: &'static str,
}

static SE_MAP: phf::Map<&'static str, AudioData> = phf_map! {
    "hit" => AudioData {
        path: "audio/se/hit.ogg",
        volume_bias: 0.0,
    },
    "draw" => AudioData {
        path: "audio/se/draw.ogg",
        volume_bias: -0.5,
    },
    "knob" => AudioData {
        path: "audio/se/knob.ogg",
        volume_bias: 0.0,
    },
    "success" => AudioData {
        path: "audio/se/success.ogg",
        volume_bias: 2.0,
    },
    "failure" => AudioData {
        path: "audio/se/failure.ogg",
        volume_bias: 0.0,
    },
};

pub fn play_se(
    se: &str,
    commands: &mut Commands,
    asset_server: &Res<AssetServer>,
    settings: &Persistent<settings::Settings>,
) {
    if !settings.is_enabled("se") {
        return;
    }
    if let Some(audio_data) = SE_MAP.get(se) {
        commands.spawn((AudioBundle {
            source: asset_server.load(audio_data.path),
            settings: PlaybackSettings {
                mode: bevy::audio::PlaybackMode::Despawn,
                volume: Volume::new(to_volume(settings.get_value("se"), audio_data.volume_bias)),
                paused: false,
                ..default()
            },
        },));
    }
}
