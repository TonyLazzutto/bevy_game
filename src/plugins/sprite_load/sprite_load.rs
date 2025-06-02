use bevy::prelude::*;

pub struct SpriteLoaderPlugin;


impl Plugin for SpriteLoaderPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, Self::load_sprite);
    }
}

impl SpriteLoaderPlugin {
    fn load_sprite(mut commands: Commands, asset_server: Res<AssetServer>) {

        commands.spawn(Camera2d::default());
        commands.spawn((
            Sprite {
                image: asset_server.load("tile.png"),
                ..Default::default()
            },
            Transform::default().with_scale(Vec3::splat(4.0)),
        ));
    }    
}