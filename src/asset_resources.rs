use bevy::{asset::{AssetServer, Handle}, audio::AudioSource, ecs::{resource::Resource, system::{Commands, Res}}, image::Image};

#[derive(Resource)]
pub struct AudioAssets {
    pub gunshot: Handle<AudioSource>,
    pub zombie_attack: Handle<AudioSource>,
    pub zombie_crunch: Handle<AudioSource>,
    pub gamesound: Handle<AudioSource>
}

#[derive(Resource)]
pub struct ZombieAnimationAssets {
    pub attack: Vec<Handle<Image>>,
    pub idle: Vec<Handle<Image>>,
    pub movement: Vec<Handle<Image>>,
}

#[derive(Resource)]
pub struct PlayerAnimationAssets {
    pub idle: Vec<Handle<Image>>,
    pub movement: Vec<Handle<Image>>,
    pub shoot: Vec<Handle<Image>>,
}

pub fn load_zombie_animation(asset_server: Res<AssetServer>, mut commands: Commands) {
    let idle = (0..17)
    .map(|i| {
        asset_server.load(format!("zombie/idle/skeleton-idle_{}.png", i))
    }).collect::<Vec<Handle<Image>>>();

    let movement = (0..17)
    .map(|i| {
        asset_server.load(format!("zombie/move/skeleton-move_{}.png", i))
    }).collect::<Vec<Handle<Image>>>();

    let attack = (0..9)
    .map(|i| {
        asset_server.load(format!("zombie/attack/skeleton-attack_{}.png", i))
    }).collect::<Vec<Handle<Image>>>();


    commands.insert_resource(ZombieAnimationAssets { attack, idle, movement});
}

pub fn load_player_animation(asset_server: Res<AssetServer>, mut commands: Commands) {
    let idle = (0..20)
    .map(|i| {
        asset_server.load(format!(
            "player/idle/survivor-idle_handgun_{i}.png"
        ))
    })
    .collect::<Vec<Handle<Image>>>();

    let movement = (0..20)
    .map(|i|{
        asset_server.load(format!(
            "player/move/survivor-move_handgun_{i}.png"
        ))
    })
    .collect::<Vec<Handle<Image>>>();

    let shoot = (0..3)
    .map(|i|{
        asset_server.load(format!(
            "player/shoot/survivor-shoot_handgun_{i}.png"
        ))
    })
    .collect::<Vec<Handle<Image>>>();

    commands.insert_resource( PlayerAnimationAssets { idle, movement, shoot});
}