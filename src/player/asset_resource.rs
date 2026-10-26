use bevy::{asset::{AssetServer, Handle}, ecs::{resource::Resource, system::{Commands, Res}}, image::Image};

#[derive(Resource)]
pub struct PlayerAnimationAssets {
    pub idle: Vec<Handle<Image>>,
    pub movement: Vec<Handle<Image>>,
    pub shoot: Vec<Handle<Image>>,
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