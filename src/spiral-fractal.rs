
//! Fibbonacci Spiral Fratal 

// In this project I have made a spiral fractal that follows the Fibbonacci Sequence, it expands
// exponentially technically forever, and is self-similar. Cools patterns emerge, but this is
// very simple. You can also change the total_spirals variable on line 28 to increase num of them

// To run it:
// cargo run --example spiral

use bevy::{input::mouse::MouseWheel, prelude::*, sprite::MaterialMesh2dBundle};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .insert_resource(ClearColor(Color::MIDNIGHT_BLUE))
        .add_startup_system(spiral_draw)
        .add_system(mouse_scroll)
        .run();
}

fn spiral_draw(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    // Relevant modifiers to edit spiral properties you reader may edit
    let shape_density = 20;
    let size_mod = 0.150;
    let total_spirals: u8 = 1;

    commands.spawn_bundle(Camera2dBundle::default());
    for spiral_num in 0..total_spirals {
        // Default only runs once, but can repeat to draw same spiral multiple times but rotated

        //DONT EDIT THESE THEY ARE NOT TO BE EDITED
        let mut prev_term: f32 = 0.0;
        let mut next_last_term: f32 = 1.0;
        let mut direction: usize = 0;
        let mut offset: Vec3 = Vec3::new(0.0, 0.0, 0.0);
        let directions = [
            Vec3::new(1.0, 0.0, 0.0),
            Vec3::new(0.0, 1.0, 0.0),
            Vec3::new(-1.0, 0.0, 0.0),
            Vec3::new(0.0, -1.0, 0.0),
        ];
        println!("{}", spiral_num as f32 / total_spirals as f32 * 360.0);
        for rotation_num in 0..=70 {
            // Each iteration of rotation_num is another full rotation
            let quarter = rotation_num as f32;

            // Calculate Fibbonacci sequence
            let mut radius = prev_term + next_last_term;
            next_last_term = prev_term;
            prev_term = radius;

            radius *= size_mod;

            let change = directions[direction] * next_last_term;

            for i in 0..=shape_density {
                // Draw Quarter of spiral

                // The precise angle of individual shape, plus which quarter it is in.
                let shape_rotation =
                    ((((i as f32) / shape_density as f32) * 90.0) + (quarter * -90.0)).to_radians();

                commands.spawn_bundle(MaterialMesh2dBundle {
                    mesh: meshes
                        .add(shape::RegularPolygon::new(
                    (prev_term + (next_last_term * (1.0 - (i as f32/ shape_density as f32)
                            ))) * size_mod/ 10.,5,).into(),).into(),
                    material: materials.add(ColorMaterial::from(Color::hsla(i as f32 / shape_density as f32 * 360.0,2.0,2.0,0.50,))),
                    transform: Transform::from_translation(
                        Quat::from_rotation_z(((spiral_num as f32 / total_spirals as f32) * 360.0).to_radians())
                        .mul_vec3(Vec3::new(shape_rotation.sin() * radius, shape_rotation.cos() * radius, 0.) + offset),
                    )
                    .with_rotation(Quat::from_rotation_z(shape_rotation)),
                    ..default()
                });
            }
            offset += change * size_mod;
            direction += 1;

            if direction > 3 {
                direction -= 4;
            }
            if rotation_num == 0 {
                direction -= 1;
            }
        }
    }
}

fn mouse_scroll(
    mut mouse_wheel_events: EventReader<MouseWheel>,

    mut query: Query<(&mut OrthographicProjection, &mut Transform), With<Camera>>,
    mut speed: Local<f32>,
) {
    // Change move speed when scrolling
    for event in mouse_wheel_events.iter() {
        *speed -= event.y * 0.03;
    }

    if *speed > 19.25 {
        *speed -= 19.25;
    } else if *speed < 0.0 {
        *speed += 19.25;
    }

    for (mut que, mut cam_trans) in query.iter_mut() {
        let mut log_scale = que.scale.ln();
        log_scale += *speed;
        
        println!("{:?}", *speed);
        // println!("Size {}, {}", log_scale, *speed);

        // Make it jump back seamlessly, absolutely no idea why this specific number works
        if log_scale > 19.25 {
            log_scale -= 19.25;
        } else if log_scale < 0.0 {
            log_scale += 19.25;
        }

        que.scale = log_scale.exp();
    }
}
