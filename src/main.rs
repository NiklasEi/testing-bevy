use bevy::prelude::*;
use bevy_kira_audio::{AudioPlugin, Audio};

fn main() {
    App::new().add_plugins(DefaultPlugins).add_plugin(AudioPlugin).add_startup_system(play).run()
}


fn play(audio: Res<Audio>, asset_server: Res<AssetServer>) {
    audio.play_looped(asset_server.load("background.ogg").into());
}
