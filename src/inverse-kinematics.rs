// Fixed Point Inverse Kinematics

// I made a quick and simple Inverse-Kinematics demo, it does this in the form that resembles a string,
//  there could be other methods to implement this. You can edit kinematics settings at line 32-34, you
//  can change the segment amount, length, or choose if the arm is locked to fixed point in center

// To run it:
// cargo run --example kinematics

use bevy::prelude::*;
use bevy_polyline::prelude::*;

fn main() {
    App::new()
        .insert_resource(WindowDescriptor {
            title: "Inverse Kinematics!".to_string(),
            width: 600.,
            height: 600.,
            resizable: false,
            ..default()
        })
        .insert_resource(ClearColor(Color::SEA_GREEN))
        .init_resource::<KinematicSettings>()
        .add_plugins(DefaultPlugins)
        .add_plugin(PolylinePlugin)
        .add_startup_system(setup)
        .add_system(move_line)
        .run();
}

fn setup(
    mut commands: Commands,
    mut polyline_materials: ResMut<Assets<PolylineMaterial>>,
    mut polylines: ResMut<Assets<Polyline>>,

    mut settings: ResMut<KinematicSettings>,
) {
    settings.sections = 500;
    settings.total_length = 1.2;
    settings.fixed = false;

    commands.spawn_bundle(PolylineBundle {
        polyline: polylines.add(Polyline {
            vertices: vec![Vec3::ZERO; settings.sections + 1],
            ..Default::default()
        }),
        material: polyline_materials.add(PolylineMaterial {
            width: 10.0,
            color: Color::RED,
            perspective: false,
            ..Default::default()
        }),
        ..Default::default()
    });

    // camera

    commands.spawn_bundle(Camera3dBundle {
        projection: bevy::render::camera::Projection::Orthographic(OrthographicProjection {
            scale: 0.0034,
            ..default()
        }),
        transform: Transform::from_xyz(0.0, 0.0, 5.0).looking_at(Vec3::ZERO, Vec3::Y),
        ..Camera3dBundle::default()
    });
}

fn move_line(
    query: Query<&Handle<Polyline>>,
    mut polylines: ResMut<Assets<Polyline>>,
    windows: Res<Windows>,

    settings: Res<KinematicSettings>,
    mut x: Local<f32>,
    mut y: Local<f32>,
) {
    let window = windows.get_primary().unwrap();
    if let Some(_position) = window.cursor_position() {
        for polyline in query.iter() {
            let mut verts = polylines.get_mut(polyline).unwrap().vertices.clone();

            let length = settings.total_length / ((settings.sections + 1) as f32);

            // Get X and Y position
            let cursor = window.cursor_position().unwrap();
            *x = (cursor[0] - windows.primary().width() / 2.0) / (windows.primary().width() / 2.0);
            *y =
                (cursor[1] - windows.primary().height() / 2.0) / (windows.primary().height() / 2.0);

            verts[0] = Vec3::new(*x, *y, verts[0][2]);

            // For every section in rope, start at end and work to the center
            for i in 1..verts.len() {
                // Point (Rotate) towards the direction of the cursor or next section
                let angle: f32 =
                    (verts[i - 1][0] - verts[i][0]).atan2(verts[i - 1][1] - verts[i][1]);

                // Then move over to connect perfectly to next section/cursor
                verts[i] = Vec3::new(
                    angle.sin() * -length + verts[i - 1][0],
                    angle.cos() * -length + verts[i - 1][1],
                    verts[i][2],
                );
            }

            // If it doesn't line up to (0, 0) exactly, just shift everything over so it does
            if (verts[verts.len() - 1] != Vec3::ZERO) && settings.fixed {
                let difference = verts[verts.len() - 1];
                for i in 0..verts.len() {
                    verts[i] -= difference;
                }
            }

            // Apply all the changes to the actual polyline vertices
            polylines.get_mut(polyline).unwrap().vertices = verts;
        }
    }
}

#[derive(Default)]
struct KinematicSettings {
    sections: usize,
    total_length: f32,
    fixed: bool,
}
