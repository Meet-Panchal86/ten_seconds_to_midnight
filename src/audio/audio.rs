use bevy::{asset::{AssetServer, Handle}, audio::{AudioPlayer, AudioSource, PlaybackMode, PlaybackSettings, Volume}, ecs::{resource::Resource, system::{Commands, Res}}, utils::default};

#[derive(Resource)]
pub struct AudioAssets {
    pub gunshot: Handle<AudioSource>,
    pub zombie_attack: Handle<AudioSource>,
    pub zombie_crunch: Handle<AudioSource>,
    pub gamesound: Handle<AudioSource>
}


pub fn load_audio_assets(asset_server: Res<AssetServer>, mut commands: Commands) {
    let gunshot = asset_server.load("audio/gun_shot.wav");
    let zombie_attack = asset_server.load("audio/zombie_attack.wav");
    let zombie_crunch = asset_server.load("audio/zombie_crunch.wav");
    let gamesound = asset_server.load("audio/game_sound.mp3");

    commands.insert_resource(AudioAssets { gunshot, zombie_attack, zombie_crunch, gamesound});
}

pub fn game_sound(audio_assets: Res<AudioAssets>, mut commands: Commands) {
    commands.spawn((
        AudioPlayer::new(audio_assets.gamesound.clone()),
        PlaybackSettings {
            mode: PlaybackMode::Loop,
            volume: Volume::Linear(0.3),
            ..default()
        }
    ));
}