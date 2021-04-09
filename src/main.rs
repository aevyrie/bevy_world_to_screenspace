use bevy::render::camera::*;
use bevy::prelude::*;
//use bevy_fly_camera::*;

fn main() {
    App::build()
        .add_plugins(DefaultPlugins)
        .add_startup_system(setup.system())
        //.add_plugin(FlyCameraPlugin)
        .add_system(update_text_position.system())
        .add_system_to_stage(CoreStage::PreUpdate, move_box.system())
        .run();
}

fn update_text_position(
    windows: Res<Windows>,
    mut text_query: Query<(&mut Style, &CalculatedSize), With<FollowText>>,
    mesh_query: Query<&Transform, With<Handle<Mesh>>>,
    camera_query: Query<(&Camera, &GlobalTransform), With<ThreeDCam>>,
) {
    for (camera, camera_transform) in camera_query.iter() {
        for mesh_position in mesh_query.iter() {
            for (mut style, calculated) in text_query.iter_mut() {
                match camera.world_to_screen(&windows, camera_transform, mesh_position.translation)
                {
                    Some(coords) => {
                        style.position.left = Val::Px(coords.x - calculated.size.width / 2.0);
                        style.position.bottom = Val::Px(coords.y - calculated.size.height / 2.0);
                    }
                    None => {
                        // A hack to hide the text when the cube is behind the camera
                        style.position.bottom = Val::Px(-1000.0);
                    }
                }
            }
        }
    }
}

fn move_box(time: Res<Time>, mut query: Query<&mut Transform, With<Handle<Mesh>>>,) {
    for mut transform in query.iter_mut() {
        let oscillator = ((time.seconds_since_startup() * 3.7).sin() * 3.0) as f32;
        let oscillator1 = ((time.seconds_since_startup() * 12.0 + 3.14/2.0).sin()) as f32;
        let oscillator2 = ((time.seconds_since_startup() * 2.2).sin()*5.0) as f32;
        transform.translation.x = oscillator;
        transform.translation.y = oscillator1;
        transform.translation.z = oscillator2;
    }
}

struct FollowText;

// Unit struct to mark the 3d camera for queries (As opposed to the 2d UI camera)
struct ThreeDCam;

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut meshes: ResMut<Assets<Mesh>>,
) {
    let font = asset_server.load("fonts/FiraSans-Bold.ttf");

    // ui camera
    commands.spawn_bundle(UiCameraBundle::default());

    commands
        .spawn_bundle(TextBundle {
            style: Style {
                align_self: AlignSelf::FlexEnd,
                position_type: PositionType::Absolute,
                position: Rect {
                    bottom: Val::Px(5.0),
                    left: Val::Px(15.0),
                    ..Default::default()
                },
                size: Size {
                    width: Val::Px(200.0),
                    ..Default::default()
                },
                ..Default::default()
            },
            text: Text::with_section(
                "A Cube".to_string(),
                TextStyle {
                    font,
                    font_size: 50.0,
                    color: Color::WHITE,
                },
                TextAlignment {
                    ..Default::default()
                }
            ),
            ..Default::default()
        })
        .insert(FollowText);

    commands
        .spawn_bundle(PerspectiveCameraBundle {
            transform: Transform::from_matrix(Mat4::face_toward(
                Vec3::new(0.0, 0.0, 8.0),
                Vec3::new(0.0, 0.0, 0.0),
                Vec3::new(0.0, 1.0, 0.0),
            )),
            ..Default::default()
        })
        .insert(ThreeDCam);
        //.with(FlyCamera::default())

    commands
        .spawn_bundle(PbrBundle {
            mesh: meshes.add(Mesh::from(shape::Cube { size: 1.0 })),
            material: materials.add(Color::rgb(1.0, 1.0, 1.0).into()),
            transform: Transform::from_translation(Vec3::new(0.0, 0.0, 0.0)),
            ..Default::default()
        });

    commands
        .spawn_bundle(LightBundle {
            transform: Transform::from_translation(Vec3::new(4.0, 8.0, 4.0)),
            ..Default::default()
        });
}
