use bevy::{prelude::*, sprite::MaterialMesh2dBundle};
use bevy_prototype_lyon::prelude::*;
use rand::Rng;

fn main() {
    App::new()
        .insert_resource(WindowDescriptor {
            title: "Trilateration".to_string(),
            width: 800.,
            height: 550.,
            ..default()
        })
        .insert_resource(ClearColor(Color::BISQUE))
        .add_plugins(DefaultPlugins)
        .add_startup_system(spawn_camera)
        .add_system(detect_input)
        .add_plugin(ShapePlugin)
        .run();
}

fn trilaterate(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,

    mut marker_pos: Vec2,
    points: [Vec2; 3],
) {
    // Fixed Markers
    // let points: [Vec2; 3] = [
    //     Vec2::new(-100.0, 100.0),
    //     Vec2::new(100.0, 100.0),
    //     Vec2::new(0.0, -100.0),
    // ];

    // let mut marker_pos = Vec2::new(rng.gen_range(-100.0..100.0), rng.gen_range(0.0..100.0));

    let distances: [f32; 3] = [
        points[0].distance(marker_pos),
        points[1].distance(marker_pos),
        points[2].distance(marker_pos),
    ];

    marker_pos = Vec2::new(0.0, 0.0); //Clear position data

    for i in 0..points.len() {
        // Draw Radius Outline
        let shape = shapes::RegularPolygon {
            sides: 50,
            feature: shapes::RegularPolygonFeature::Radius(distances[i]),
            ..shapes::RegularPolygon::default()
        };
        commands.spawn_bundle(GeometryBuilder::build_as(
            &shape,
            DrawMode::Outlined {
                fill_mode: FillMode::color(Color::rgba(0.0, 0.0, 0.0, 0.0)),
                outline_mode: StrokeMode::new(Color::rgba(0.0, 0.0, 0.0, 0.7), 3.0),
            },
            Transform::from_translation(points[i].extend(0.0)),
        ));

        commands.spawn_bundle(MaterialMesh2dBundle {
            mesh: meshes.add(shape::Circle::new(6.0).into()).into(),
            material: materials.add(ColorMaterial::from(Color::RED)),
            transform: Transform::from_translation(points[i].extend(1.0)),
            ..default()
        });
    }

    let mut intersects = Vec::new();
    for i in 0..points.len() {
        let mut u: usize = 0;

        // Always ensure it has a circle to pair with
        if i < points.len() - 1 {
            u = i + 1
        }

        // Welcome to formula hell
        let rd1: f32 = distances[i].abs();
        let rd2: f32 = distances[u].abs();

        let x1: f32 = points[i].x;
        let y1: f32 = points[i].y;

        let x2: f32 = points[u].x;
        let y2: f32 = points[u].y;

        let circ_dist_x: f32 = x1 - x2;
        let circ_dist_y: f32 = y1 - y2;

        let dist = (circ_dist_x * circ_dist_x + circ_dist_y * circ_dist_y).sqrt();

        let a = (rd1 * rd1 - rd2 * rd2) / (2.0 * dist.powf(2.0));
        let r1r2 = rd1 * rd1 - rd2 * rd2;
        let c =
            (2.0 * (rd1 * rd1 + rd2 * rd2) / dist.powf(2.0) - (r1r2 * r1r2) / dist.powf(4.0) - 1.0)
                .sqrt();

        let fx = (x1 + x2) / 2.0 + a * (x2 - x1);
        let gx = c * (y2 - y1) / 2.0;

        let fy = (y1 + y2) / 2.0 + a * (y2 - y1);
        let gy = c * (x1 - x2) / 2.0;

        intersects.push(Vec2::new(fx + gx, fy + gy));
        intersects.push(Vec2::new(fx - gx, fy - gy));
    }

    // println!("Intersections: {:?}", intersects);

    for i in 0..intersects.len() {
        intersects[i] = intersects[i].round();
    }
    let mut unrendered_set = intersects.clone();
    for i in intersects.iter() {
        //Summon Marker
        commands.spawn_bundle(MaterialMesh2dBundle {
            mesh: meshes.add(shape::Circle::new(4.0).into()).into(),
            material: materials.add(ColorMaterial::from(Color::BLUE)),
            transform: Transform::from_translation(i.extend(1.0)),
            ..default()
        });

        unrendered_set.remove(0);
        println!("Set: {:?} Current: {}", unrendered_set, i);
        if unrendered_set.contains(i) {
            println!("{}", i);
            commands.spawn_bundle(MaterialMesh2dBundle {
                mesh: meshes.add(shape::Circle::new(5.0).into()).into(),
                material: materials.add(ColorMaterial::from(Color::GREEN)),
                transform: Transform::from_translation(i.extend(2.0)),
                ..default()
            });
        }
    }
}

fn detect_input(
    mut commands: Commands,
    meshes: ResMut<Assets<Mesh>>,
    materials: ResMut<Assets<ColorMaterial>>,

    keyboard_input: Res<Input<KeyCode>>,
    buttons: Res<Input<MouseButton>>,
    query: Query<(Entity, &Transform, Without<Camera2d>)>,
    windows: Res<Windows>,
) {
    // println!("{}", query.iter().len());

    let window = windows.get_primary().unwrap();

    if keyboard_input.just_pressed(KeyCode::Space) {
        for (ent, _transform, _que) in query.iter() {
            commands.entity(ent).despawn();
        }

        let mut rng = rand::thread_rng();

        // Window Width and Height, can generate within 0.7 of the screen
        let wid = (windows.primary().width() / 2.0) * 0.7;
        let hei = (windows.primary().height() / 2.0) * 0.7;

        // Calls the Bevy system as event
        trilaterate(
            commands,
            meshes,
            materials,
            Vec2::new(rng.gen_range(-100.0..100.0), rng.gen_range(0.0..100.0)),
            [
                Vec2::new(rng.gen_range(-wid..wid), rng.gen_range(-hei..hei)),
                Vec2::new(rng.gen_range(-wid..wid), rng.gen_range(-hei..hei)),
                Vec2::new(rng.gen_range(-wid..wid), rng.gen_range(-hei..hei)),
            ],
        );
    } else if buttons.pressed(MouseButton::Left) {
        let mut positions = Vec::new();
        for (ent, transform, _que) in query.iter() {
            positions.push(transform.translation.truncate());
            commands.entity(ent).despawn();
        }

        let cursor = window.cursor_position().unwrap();
        let origin = Vec2::new(
            cursor[0] - windows.primary().width() / 2.0,
            cursor[1] - windows.primary().height() / 2.0,
        );

        // Calls the Bevy system as event
        trilaterate(
            commands,
            meshes,
            materials,
            origin,
            [positions[0], positions[1], positions[2]],
        );
    }
}

fn spawn_camera(
    mut commands: Commands,
    meshes: ResMut<Assets<Mesh>>,
    materials: ResMut<Assets<ColorMaterial>>,
) {
    commands.spawn_bundle(Camera2dBundle::default());

    let mut rng = rand::thread_rng();

    trilaterate(
        commands,
        meshes,
        materials,
        Vec2::new(rng.gen_range(-100.0..100.0), rng.gen_range(0.0..100.0)),
        [
            Vec2::new(rng.gen_range(-150.0..100.0), rng.gen_range(-150.0..100.0)),
            Vec2::new(rng.gen_range(-150.0..100.0), rng.gen_range(-150.0..100.0)),
            Vec2::new(rng.gen_range(-150.0..100.0), rng.gen_range(-150.0..100.0)),
        ],
    );
}
