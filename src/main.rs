use bevy::{
    log::{Level, LogSettings},
    prelude::*,
};

fn main() {
    let mut app = App::new();
    app.insert_resource(ClearColor(Color::DARK_GRAY))
        // Bad lines here
            .insert_resource(LogSettings {
               level: Level::DEBUG,
            filter: "rampart::world::index=debug,rampart::world::navigation=debug,rampart=trace,bevy_ecs_tilemap::layer=warn,wgpu=error,info".to_string(),
        })
        .insert_resource(WindowDescriptor {
            width: 1270.0,
            height: 720.0,
            title: String::from("Rampart"),
            ..Default::default()
        })
        .add_plugins(DefaultPlugins);

    app.run();
}
