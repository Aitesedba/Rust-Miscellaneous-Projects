mod geometry;
use bevy::{input::mouse::MouseWheel, prelude::*, sprite::MaterialMesh2dBundle};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .insert_resource(ClearColor(Color::MIDNIGHT_BLUE))
        .add_startup_system(spiral_draw)
        .add_system(mouse_scroll)
        .run();
}


fn mouse_scroll(
    mut mouse_wheel_events: EventReader<MouseWheel>,

    mut query: Query<&mut OrthographicProjection, With<Camera>>,
    mut speed: Local<f32>,
) {
    for event in mouse_wheel_events.iter() {
        *speed -= event.y * 0.05;
    }

    for mut que in query.iter_mut() {
        let mut log_scale = que.scale.ln();

        log_scale += *speed;

        println!("{}", log_scale);

        // Make it jump back seamlessly, absolutelyno idea why this specific number works
        if log_scale > 19.25 {
            log_scale -= 19.25;
        } else if log_scale < 0.0 {
            log_scale += 19.25;
        }


        que.scale = log_scale.exp();
    }
}

fn spiral_draw(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());

    // Relevant modifiers to info
    let shape_density = 20;
    let size_mod = 0.150;
    

    let mut prev_term = 0.0;
    let mut next_last_term = 1.0;
    let mut direction: usize = 0;
    let mut offset: Vec3 = Vec3::new(0.0, 0.0, 0.0);
    let directions = [
        Vec3::new(1.0, 0.0, 0.0),
        Vec3::new(0.0, 1.0, 0.0),
        Vec3::new(-1.0, 0.0, 0.0),
        Vec3::new(0.0, -1.0, 0.0),
    ];
    

    
    for u in 0..=300 {
        // Each segment is one quarter spiral
        let quarter = u as f32;

        // Calculate Fibbonacci sequence
        let mut radius = prev_term + next_last_term;
        // outside_radius.0 = radius;
        next_last_term = prev_term;
        prev_term = radius;

        radius *= size_mod;

        let change = directions[direction] * next_last_term;

        for i in 0..=shape_density {
            // Draw Quarter of spiral

            // The precise angle of individual shape, plus which quarter it is in.
            let f =
                ((((i as f32) / shape_density as f32) * 90.0) + quarter * -90.0 ).to_radians();

            commands.spawn_bundle(MaterialMesh2dBundle {
                mesh: meshes
                    .add(geometry::RegularPolygon::new(
                            (prev_term+ (next_last_term * (1.0 - (i as f32 / shape_density as f32)))
                                    )* size_mod
                                    / 10.,
                            5,
                        )
                        .into(),
                    )
                    .into(),
                material: materials.add(ColorMaterial::from(Color::hsla(
                    i as f32 / shape_density as f32 * 360.0,
                    2.0,
                    2.0,
                    0.50,
                ))),
                transform: Transform::from_translation(
                    Vec3::new(f.sin() * radius, f.cos() * radius, 0.) + offset,
                )
                .with_rotation(Quat::from_rotation_z(f)),
                ..default()
            });
        }
        offset += change * size_mod;
        direction += 1;

        if direction > 3 {
            direction -= 4;
        }
        if u == 0 {
            direction -= 1;
        }
        // offset += change * radius;
    }
}
