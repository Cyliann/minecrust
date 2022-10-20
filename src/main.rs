use bevy::{prelude::*, render::texture::ImageSettings};
use bevy_atmosphere::prelude::*;
use bevy_flycam::{FlyCam, NoCameraPlayerPlugin};
use bevy_inspector_egui::WorldInspectorPlugin;
use bevy::diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin, EntityCountDiagnosticsPlugin};

pub const HEIGHT: f32 = 1080.0;
pub const WIDTH: f32 = 1920.0;

mod chunk;
mod voxel_data;
mod world;

fn main() {
    App::new()
        .insert_resource(Msaa { samples: 4 })
        .insert_resource(ClearColor(Color::rgb(0.2, 0.2, 0.2)))
        .insert_resource(ImageSettings::default_nearest())
        .insert_resource(WindowDescriptor {
            width: WIDTH,
            height: HEIGHT,
            title: "Jebać Knura".to_string(),
            resizable: true,
            //mode: WindowMode::BorderlessFullscreen,
            ..Default::default()
        })
        // .insert_resource(Atmosphere::default()) // Default Atmosphere material, we can edit it to simulate another planet
        // .insert_resource(CycleTimer(Timer::new(
        //     bevy::utils::Duration::from_millis(100), // Update our atmosphere every 50ms (in a real game, this would be much slower, but for the sake of an example we use a faster update)
        //     true,
        // )))
        .add_plugins(DefaultPlugins)
        // Inspector setup
        .add_plugin(WorldInspectorPlugin::new())
        .add_plugin(NoCameraPlayerPlugin)
        // .add_plugin(WireframePlugin)
        .add_plugin(AtmospherePlugin)
        .add_plugin(LogDiagnosticsPlugin::default())
        .add_plugin(FrameTimeDiagnosticsPlugin::default())
        .add_startup_system(spawn_light)
        .add_startup_system(spawn_camera)
        .add_startup_system(chunk::spawn_chunk)
        // .add_system(daylight_cycle)
        .run();
}

fn spawn_camera(mut commands: Commands) {
    commands
        .spawn_bundle(Camera3dBundle {
            transform: Transform::from_xyz(-2.0, 5.0, 5.0).looking_at(Vec3::ZERO, Vec3::Y),
            ..Default::default()
        })
        .insert(FlyCam)
        .insert(Name::new("Camera"))
        .insert(AtmosphereCamera(None));
}

// //
// // Daylight cycle
// //
// // Marker for updating the position of the light, not needed unless we have multiple lights
// #[derive(Component)]
// struct Sun;
//
// // Timer for updating the daylight cycle (updating the atmosphere every frame is slow, so it's better to do incremental changes)
// struct CycleTimer(Timer);
//
// // We can edit the Atmosphere resource and it will be updated automatically
// fn daylight_cycle(
//     mut atmosphere: ResMut<Atmosphere>,
//     mut query: Query<(&mut Transform, &mut DirectionalLight), With<Sun>>,
//     mut timer: ResMut<CycleTimer>,
//     time: Res<Time>,
// ) {
//     timer.0.tick(time.delta());
//
//     if timer.0.finished() {
//         let t = time.time_since_startup().as_millis() as f32 / 2000.0;
//         atmosphere.sun_position = Vec3::new(0., t.sin(), t.cos());
//
//         if let Some((mut light_trans, mut directional)) = query.single_mut().into() {
//             light_trans.rotation = Quat::from_rotation_x(-t.sin().atan2(t.cos()));
//             directional.illuminance = t.sin().max(0.0).powf(2.0) * 100000.0;
//         }
//     }
// }
//
// fn spawn_light(mut commands: Commands){
//     // Our Sun
//     commands
//         .spawn_bundle(DirectionalLightBundle {
//             ..Default::default()
//         })
//         .insert(Sun); // Marks the light as Sun
// }


fn spawn_light(mut commands: Commands) {
    commands
        .spawn_bundle(PointLightBundle {
            point_light: PointLight {
                intensity: 30000.0,
                shadows_enabled: true,
                range: 100.0,
                radius: 100.0,
                ..default()
            },
            transform: Transform::from_xyz(16.0, 30.0, 16.0),
            ..default()
        })
        .insert(Name::new("Light"));
}


