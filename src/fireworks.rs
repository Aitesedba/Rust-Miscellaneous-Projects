mod geometry;


use bevy::{prelude::*, sprite::MaterialMesh2dBundle};
use rand::Rng;

#[derive(Component, Debug)]
pub struct Firework {
    position: Vec2,
    velocity: Vec2,
    to_explode: bool,
    color: Color,
}

#[derive(Component, Debug)]
struct EffectFade {
    alpha: f32,
    fade_spd: f32,
    size: f32,
}

pub struct ExplosionLocation {
    pub location: Vec2,
    color: Color,
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_event::<ExplosionLocation>()
        .insert_resource(WindowDescriptor {
            title: "Fireworks".to_string(),
            ..default()
        })
        .insert_resource(ClearColor(Color::rgb(0.1, 0.1, 0.1)))
        .add_system(generate)
        .add_system(locations)
        .add_system(fade_trail)
        .add_system(fade_sides)
        .add_system(side_effects)
        .run();
}

fn fade_sides(mut query: Query<&mut EffectFade>) {
    for mut handle in query.iter_mut() {
        //let color = &mut materials.get_mut(handle).unwrap().color;
        // your color changing logic here instead:

        let fade_spd = handle.fade_spd;
        handle.alpha -= fade_spd;
    }
}

fn generate(
    mut commands: Commands,
    buttons: Res<Input<MouseButton>>,
    windows: Res<Windows>,
    mut explosion_event: EventReader<ExplosionLocation>,
) {
    let launch_strength = 14.0;
    let launch_str_difference = 3.0;
    let side_spd = 4.0;
    let window = windows.get_primary().unwrap();
    let mut rng = rand::thread_rng();

    let explode_fade_spd = 0.08;
    let explode_brightness = 2.0; //Larger means takes longer to fade
    let explode_spd = 5.0;
    let explode_min_size: u8 = 3;
    let explode_max_size: u8 = 8;

    for event in explosion_event.iter() {
        //println!("Exploded! {:?}", event.location);
        let origin_b = event.location;

        let new_color;
        if rng.gen_ratio(2, 3) {
            //2 out of 3 chance of keeping parent's color
            new_color = event.color;
        } else {
            new_color = Color::hsla(rng.gen_range(0.0..=360.0), 2.0, 2.0, 1.0);
        }

        for _i in 0..=rng.gen_range(explode_min_size..explode_max_size) {
            commands
                .spawn()
                .insert(Firework {
                    position: origin_b,
                    velocity: Vec2::new(
                        rng.gen_range(-explode_spd..=explode_spd),
                        rng.gen_range(-1.0..=explode_spd),
                    ),
                    to_explode: false,
                    color: new_color,
                })
                .insert(EffectFade {
                    alpha: explode_brightness,
                    fade_spd: explode_fade_spd,
                    size: 3.0,
                });
        }
    }

    if buttons.just_released(MouseButton::Left) {
        if let Some(_position) = window.cursor_position() {
            // cursor is inside the window, position given

            let cursor = window.cursor_position().unwrap();
            let origin = Vec2::new(
                cursor[0] - windows.primary().width() / 2.0,
                cursor[1] - windows.primary().height() / 2.0,
            );

            //println!("Clicked! {:?}", origin);

            commands.spawn().insert(Firework {
                position: origin,
                velocity: Vec2::new(
                    rng.gen_range(-side_spd..=side_spd),
                    launch_strength + rng.gen_range(-launch_str_difference..=launch_str_difference),
                ),
                to_explode: true,
                color: Color::hsla(rng.gen_range(0.0..=360.0), 2.0, 2.0, 1.0),
            });

            //launch(window.cursor_position().unwrap());
        }
    }
}

fn side_effects(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,

    windows: Res<Windows>,
    mut query: Query<(Entity, &mut Firework, &mut EffectFade)>,
) {
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());
    let grav = 0.2;

    //println!("Fireworks total: {}", query.iter().len());

    for (ent, mut particle, effects) in query.iter_mut() {
        //Cycles through every particle

        //println!("{:?}", ent);
        let velocity = particle.velocity;
        particle.position += velocity;

        particle.velocity[1] -= grav;
        particle.color.set_a(effects.alpha);

        //println!("{}",effect_mods);

        // Circle
        commands.spawn_bundle(MaterialMesh2dBundle {
            mesh: meshes
                .add(geometry::Circle::new(effects.size).into())
                .into(),
            material: materials.add(ColorMaterial::from(particle.color)),
            transform: Transform::from_translation(Vec3::new(
                particle.position[0] / 1.0,
                particle.position[1] / 1.0,
                0.0,
            )),
            ..default()
        });

        if (particle.position[1] < -windows.primary().height() / 2.0) | (effects.alpha < 0.0) {
            commands.entity(ent).despawn();
        }
    }
}

fn locations(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,

    windows: Res<Windows>,
    mut query: Query<(Entity, &mut Firework), Without<EffectFade>>,
    mut explode: EventWriter<ExplosionLocation>,
) {
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());
    let grav = 0.3;

    //println!("Fireworks total: {}", query.iter().len());

    for (ent, mut particle) in query.iter_mut() { //Cycles through every particle

        let velocity = particle.velocity;
        particle.position += velocity;

        particle.velocity[1] -= grav;

        
        commands.spawn_bundle(MaterialMesh2dBundle { // Summon Circles
            mesh: meshes.add(geometry::Circle::new(5.0).into()).into(),
            material: materials.add(ColorMaterial::from(particle.color)),
            transform: Transform::from_translation(Vec3::new(
                particle.position[0] / 1.0,
                particle.position[1] / 1.0,
                0.0,
            )),
            ..default()
        });
        if (particle.velocity[1] < 1.5) & particle.to_explode {
            //Explode if it stops going up
            explode.send(ExplosionLocation {
                location: particle.position,
                color: particle.color,
            });
            //println!("Explosion! {}", particle.position);
        }
        if (particle.position[1] < -windows.primary().height() / 2.0)
            | ((particle.velocity[1] < 1.5) & particle.to_explode)
        {
            commands.entity(ent).despawn();
        }
    }
}

fn fade_trail(
    mut commands: Commands,
    query: Query<(Entity, &Handle<ColorMaterial>)>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {

    for (ent, handle) in query.iter() {
        let color = &mut materials.get_mut(handle).unwrap().color;
        color.set_a(color.a() - 0.1);

        if color.a() <= 0.0 {
            commands.entity(ent).despawn();
        }
    }
}
