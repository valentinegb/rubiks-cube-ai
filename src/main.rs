mod rubiks_cube;

use std::f32::consts::PI;

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
        .insert_resource(ClearColor(Color::hex("1E2227").unwrap()))
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
    let mut rubiks_cube_transform = Transform::from_xyz(0., 0., -0.2);

    rubiks_cube_transform.rotate_y(PI - 0.5);
    rubiks_cube_transform.rotate_x(0.5);

    rubiks_cube_resource.0 = Some(
        commands
            .spawn(SceneBundle {
                scene: rubiks_cube,
                transform: rubiks_cube_transform,
                ..default()
            })
            .id(),
    );

    let point_light = PointLight {
        radius: 0.2,
        shadows_enabled: true,
        ..default()
    };

    commands.spawn(PointLightBundle {
        point_light,
        transform: Transform::from_xyz(0., 4., 0.),
        ..default()
    });
    commands.spawn(PointLightBundle {
        point_light,
        transform: Transform::from_xyz(-4., 4., 0.),
        ..default()
    });
    commands.spawn(PointLightBundle {
        point_light,
        transform: Transform::from_xyz(4., 4., 0.),
        ..default()
    });
    commands.spawn(PointLightBundle {
        point_light,
        transform: Transform::from_xyz(0., 4., -4.),
        ..default()
    });
    commands.spawn(PointLightBundle {
        point_light,
        transform: Transform::from_xyz(0., 4., 4.),
        ..default()
    });
    commands
        .spawn(NodeBundle {
            style: Style {
                width: Val::Percent(100.),
                height: Val::Percent(100.),
                justify_content: JustifyContent::End,
                flex_direction: FlexDirection::Row,
                ..default()
            },
            ..default()
        })
        .with_children(|parent| {
            parent.spawn(NodeBundle {
                style: Style {
                    width: Val::Px(256.),
                    margin: UiRect::all(Val::Px(24.)),
                    padding: UiRect::all(Val::Px(24.)),
                    border: UiRect::all(Val::Px(6.)),
                    ..default()
                },
                background_color: Color::hex("21252B").unwrap().into(),
                border_color: Color::hex("282C34").unwrap().into(),
                ..default()
            });
        });
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
