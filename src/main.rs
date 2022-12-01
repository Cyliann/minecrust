use bevy::diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin};
use bevy::{prelude::*, render::texture::ImageSettings};
// use bevy::window::PresentMode::Immediate;
use crate::voxel_data::WORLD_SIZE_IN_CHUNKS;
use bevy_atmosphere::prelude::*;
use bevy_flycam::{FlyCam, NoCameraPlayerPlugin};
use bevy_inspector_egui::WorldInspectorPlugin;

pub const HEIGHT: f32 = 1080.0;
pub const WIDTH: f32 = 1920.0;

mod block_types;
mod chunk;
mod voxel_data;
mod voxel_map;
mod world;

fn main() {
    App::new()
        // Resources
        // .insert_resource(Msaa { samples: 4 })
        .insert_resource(ClearColor(Color::rgb(0.2, 0.2, 0.2)))
        .insert_resource(ImageSettings::default_nearest()) // Fix blurred textures
        .insert_resource(WindowDescriptor {
            width: WIDTH,
            height: HEIGHT,
            title: "MinecRust".to_string(),
            resizable: true,
            // present_mode: Immediate, // Disables V-Sync
            //mode: WindowMode::BorderlessFullscreen,
            ..Default::default()
        })
        .insert_resource(voxel_map::VoxelMap::new())
        .insert_resource(world::ChunkMap::new())
        .insert_resource(world::ActiveChunks::new())
        .insert_resource(world::PlayerLastChunk::new())
        .insert_resource(world::GeneratedChunks {
            chunks: [[false; WORLD_SIZE_IN_CHUNKS]; WORLD_SIZE_IN_CHUNKS],
        })
        // Plugins
        .add_plugins(DefaultPlugins)
        .add_plugin(WorldInspectorPlugin::new()) // Inspector setup
        .add_plugin(NoCameraPlayerPlugin) // Flycam setup
        .add_plugin(AtmospherePlugin) // Atmosphere setup
        .add_plugin(LogDiagnosticsPlugin::default()) // Diagnostics setup
        .add_plugin(FrameTimeDiagnosticsPlugin::default())
        // Events
        .add_event::<world::SpawnChunkEvent>()
        // Systems
        .add_startup_system(spawn_light)
        .add_startup_system(spawn_camera)
        .add_startup_system(world::spawn_world)
        .add_system(world::check_render_distance)
        .add_system(world::spawn_chunk)
        .run();
}

#[derive(Component)]
pub struct Player;

fn spawn_camera(mut commands: Commands) {
    commands
        .spawn_bundle(Camera3dBundle {
            transform: Transform::from_xyz(1.0, 30.0, 5.0)
                .looking_at(Vec3::new(16.0, 16.0, 16.0), Vec3::Y),
            ..Default::default()
        })
        .insert(FlyCam)
        .insert(Name::new("Camera"))
        .insert(AtmosphereCamera(None))
        .insert(Player);
}

fn spawn_light(mut commands: Commands) {
    commands
        // .spawn_bundle(PointLightBundle {
        //     point_light: PointLight {
        //         intensity: 3000.0,
        //         shadows_enabled: true,
        //         range: 1000.0,
        //         radius: 1000.0,
        //         ..default()
        //     },
        //     transform: Transform::from_xyz(32.0, 50.0, 32.0),
        //     ..default()
        // })
        .insert_resource(AmbientLight {
            color: Color::WHITE,
            brightness: 0.5,
        })
    // .insert(Name::new("Light"));
}
