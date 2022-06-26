//! Shows how to render simple primitive shapes with a single color.

mod geometry;
use bevy::{
    input::mouse::MouseWheel, prelude::*, sprite::MaterialMesh2dBundle, window::CursorMoved,
};



fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_startup_system(setup)
        .add_system(draw_shapes)
        .add_system(record_mouse_scroll)
        .run();
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());




    // Circle
    commands.spawn_bundle(MaterialMesh2dBundle {
        mesh: meshes.add(geometry::Circle::new(50.).into()).into(),
        material: materials.add(ColorMaterial::from(Color::PURPLE)),
        transform: Transform::from_translation(Vec3::new(-100., 0., 0.)),
        ..default()
    });

    // Hexagon
    commands.spawn_bundle(MaterialMesh2dBundle {
        mesh: meshes
            .add(geometry::RegularPolygon::new(50., 6).into())
            .into(),
        material: materials.add(ColorMaterial::from(Color::TURQUOISE)),
        transform: Transform::from_translation(Vec3::new(100., 0., 0.)),
        ..default()
    });
}

fn draw_shapes(
    mut query: Query<&mut OrthographicProjection, With<Camera>>,
    winfo: Res<Windows>,

    mut cursor_moved_events: EventReader<CursorMoved>,

    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,

    mut color: Local<f32>,
) {
    if *color > 360.0 {
        *color = 0.0;
    } else {
        *color += 1.0;
    }

    let mut y_cord = 0.0;
    let mut x_cord = 0.0;
    for event in cursor_moved_events.iter() {
        y_cord = event.position[1];
        x_cord = event.position[0];
    }

    /*// Rectangle
    commands.spawn_bundle(SpriteBundle {
        sprite: Sprite {
            color: Color::hsla(*color,2.0,2.0, 1.0),
            custom_size: Some(Vec2::new(50.0, y_cord)),

            ..default()
        },
        ..default()
    });*/

    let height = winfo.primary().height();
    let width = winfo.primary().width();

    //println!("XY: {}, {}", width, height);
    for que in query.iter_mut() {
        let scale = que.scale;

        //println!("{}", scale);
        // Hexagon
        commands.spawn_bundle(MaterialMesh2dBundle {
            mesh: meshes
                .add(geometry::RegularPolygon::new(50., 6).into())
                .into(),
            material: materials.add(ColorMaterial::from(Color::hsla(*color, 2.0, 2.0, 0.5))),
            transform: Transform::from_translation(Vec3::new(
                (x_cord - width / 2.0) * scale,
                (y_cord - height / 2.0) * scale,
                0.,
            ))
            .with_scale(Vec3::new(0.5, 0.5, 0.5)),

            ..default()
        });
    }
}

/// This system prints out all mouse events as they come in
fn record_mouse_scroll(
    mut mouse_wheel_events: EventReader<MouseWheel>,

    mut query: Query<&mut OrthographicProjection, With<Camera>>,
    mut clear_color: ResMut<ClearColor>,
) {
    for event in mouse_wheel_events.iter() {
        //println!("{:?}", event);

        for mut que in query.iter_mut() {
            let mut log_scale = que.scale.ln();

            log_scale += event.y * 0.5;

            que.scale = log_scale.exp();

            //println!("{:?}", que.scale);
        }
    }

    //println!("{:?}", color);

    clear_color.0 = Color::RED;
}
