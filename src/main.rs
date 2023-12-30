mod rubiks_cube;

use bevy::{
    input::mouse::MouseMotion,
    log::{self, LogPlugin},
    prelude::*,
};

#[derive(Resource, Default)]
struct RubiksCubeResource(Option<Entity>);

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(LogPlugin {
            #[cfg(debug_assertions)]
            level: log::Level::DEBUG,
            ..default()
        }))
        .init_resource::<RubiksCubeResource>()
        .add_systems(Startup, setup)
        .add_systems(Update, rubiks_cube_rotate)
        .run();
}

fn setup(
    mut commands: Commands,
    assets: Res<AssetServer>,
    mut rubiks_cube_resource: ResMut<RubiksCubeResource>,
) {
    let rubiks_cube = assets.load("rubiks_cube.glb#Scene0");

    rubiks_cube_resource.0 = Some(
        commands
            .spawn(SceneBundle {
                scene: rubiks_cube,
                transform: Transform::from_xyz(0., 0., -0.2),
                ..default()
            })
            .id(),
    );

    commands.spawn(Camera3dBundle::default());
}

fn rubiks_cube_rotate(
    mut motion_evr: EventReader<MouseMotion>,
    buttons: Res<Input<MouseButton>>,
    rubiks_cube_resource: Res<RubiksCubeResource>,
    mut query: Query<&mut Transform>,
) {
    for ev in motion_evr.read() {
        if buttons.pressed(MouseButton::Left) {
            if let Some(scene) = rubiks_cube_resource.0 {
                if let Ok(mut transform) = query.get_mut(scene) {
                    transform.rotate_x(ev.delta.y / 100.);
                    transform.rotate_y(ev.delta.x / 100.);
                } else {
                    warn!("Rubik's Cube transform not found in query");
                }
            } else {
                warn!("Rubik's Cube has not yet been spawned");
            }
        }
    }
}
