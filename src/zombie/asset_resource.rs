use bevy::{asset::{AssetServer, Handle}, ecs::{resource::Resource, system::{Commands, Res}}, image::Image};

#[derive(Resource)]
pub struct ZombieAnimationAssets {
    pub attack: Vec<Handle<Image>>,
    pub idle: Vec<Handle<Image>>,
    pub movement: Vec<Handle<Image>>,
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
