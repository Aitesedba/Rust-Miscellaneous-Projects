use bevy::{prelude::*};
use bevy_polyline::prelude::*;

#[derive(Default)]
struct KinematicSettings{
    sections: usize,
    total_length: f32,
}

fn main() {
    App::new()
        .insert_resource(WindowDescriptor {
            title: "Inverse Kinematics!".to_string(),
            width:  600.,
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

    settings.sections = 100;
    settings.total_length = 1.0;

    commands.spawn_bundle(PolylineBundle {
        polyline: polylines.add(Polyline {
            vertices: vec![Vec3::ZERO; settings.sections],
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
    // commands.spawn_bundle(OrthographicCameraBundle::new_2d());
    commands.spawn_bundle(OrthographicCameraBundle {
        transform: Transform::from_xyz(0.0, 0.0, 5.0)
            // .with_scale(Vec3::new(250.0, 250.0, 200.0))
            .looking_at(Vec3::ZERO, Vec3::Y),
        ..OrthographicCameraBundle::new_3d()
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

            let length = settings.total_length/(settings.sections as f32);

            let cursor = window.cursor_position().unwrap();

            *x = (cursor[0] - windows.primary().width() / 2.0) / (windows.primary().width() / 2.0);
            *y =
                (cursor[1] - windows.primary().height() / 2.0) / (windows.primary().height() / 2.0);


            verts[0] = Vec3::new(*x, *y, verts[0][2]);

                
                for i in 1..verts.len() {
                    let angle: f32 =
                        (verts[i - 1][0] - verts[i][0]).atan2(verts[i - 1][1] - verts[i][1]);
                    verts[i] = Vec3::new(
                        angle.sin() * -length + verts[i - 1][0],
                        angle.cos() * -length + verts[i - 1][1],
                        verts[i][2],
                    );
                }
                if verts[verts.len()-1] != Vec3::ZERO {
                    let difference = verts[verts.len()-1];
                    for i in 0..verts.len() {
                        verts[i] -= difference;
                    }
                }

            polylines.get_mut(polyline).unwrap().vertices = verts;
        }
    }
}
