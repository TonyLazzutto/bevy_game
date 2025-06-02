use bevy::prelude::*;

mod plugins;
use plugins::sprite_load::sprite_load::SpriteLoaderPlugin;

fn main() {
    let mut app = App::new();

    app.add_plugins(
        DefaultPlugins.set(WindowPlugin{
            primary_window: Some(Window{
            resolution: [800., 600.].into(),
                title: "Hello Bevy!".to_string(),
                ..default()
            }),
            ..default()})
        .set(ImagePlugin::default_nearest()))
        .add_plugins(SpriteLoaderPlugin);
        app.run();
}

