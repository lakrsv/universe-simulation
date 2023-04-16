use bevy::prelude::*;
use bevy_rapier2d::prelude::*;
use components::Celestial;
use constants::CelestialBodyConfig;

mod components;
mod constants;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                // fill the entire browser window
                fit_canvas_to_parent: true,
                // don't hijack keyboard shortcuts like F5, F6, F12, Ctrl+R etc.
                prevent_default_event_handling: false,
                ..default()
            }),
            ..default()
        }))
        .add_plugin(RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(100.0))
        .add_plugin(RapierDebugRenderPlugin::default())
        .insert_resource(RapierConfiguration {
            gravity: Vect::ZERO,
            ..default()
        })
        .add_startup_system(setup_graphics)
        .add_startup_system(setup_universe)
        .add_system(player_camera_control)
        .run();
}

fn setup_graphics(mut commands: Commands) {
    // Add a camera so we can see the debug-render.
    commands.spawn(Camera2dBundle::default());
}

fn setup_universe(mut commands: Commands) {
    // Create the sun
    spawn_celestial_body(&mut commands, &constants::SAGITTARIUS_A_STAR);
    spawn_celestial_body(&mut commands, &constants::SUN);
}

fn spawn_celestial_body(commands: &mut Commands, config: &CelestialBodyConfig) {
    commands
    .spawn(RigidBody::Dynamic)
    .insert(Collider::ball(config.radius))
    .insert(TransformBundle::from(Transform::from_xyz(0.0, 5.0, 0.0)))
    .insert(Velocity {
        linvel: Vec2::new(0.0, 0.0),
        angvel: config.angular_velocity,
    })
    .insert(AdditionalMassProperties::Mass(config.mass));
}


fn player_camera_control(kb: Res<Input<KeyCode>>, time: Res<Time>, mut query: Query<&mut OrthographicProjection, With<Camera>>) {
    let dist = 10.0 * time.delta().as_secs_f32();

    for mut projection in query.iter_mut() {
        let mut log_scale = projection.scale.ln();

        if kb.pressed(KeyCode::Z) {
            log_scale -= dist;
        }
        if kb.pressed(KeyCode::X) {
            log_scale += dist;
        }

        projection.scale = log_scale.exp();
    }
}
