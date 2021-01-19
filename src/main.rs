use bevy::prelude::*;
use bevy::render::camera::*;
use bevy_fly_camera::*;

fn main() {
    App::build()
        .add_plugins(DefaultPlugins)
        .add_startup_system(setup.system())
        .add_plugin(FlyCameraPlugin)
        .add_system(update_text_position.system())
        .run();
}

fn update_text_position(
    windows: Res<Windows>,
    mut text_query: Query<(&mut Style, &CalculatedSize), With<FollowText>>,
    mesh_query: Query<&GlobalTransform, With<Handle<Mesh>>>,
    camera_query: Query<(&Camera, &GlobalTransform), With<ThreeDCam>>,
) {
    for mesh_position in mesh_query.iter() {
        for camera in camera_query.iter() {
            for (mut style, calculated) in text_query.iter_mut() {
                if let Some(coords) = world_to_screen(mesh_position, camera, &windows) {
                    style.position.left = Val::Px(coords.x - calculated.size.width/2.0);
                    style.position.bottom = Val::Px(coords.y - calculated.size.height/2.0);
                }
            }
        }
    }
}

/// Given coordinates in world space, use the camera and window information to compute the
/// screen space coordinates.
pub fn world_to_screen(
    world_space_coords: &GlobalTransform,
    camera: (&Camera, &GlobalTransform),
    window_resource: &Res<Windows>,
) -> Option<Vec2> {
    let projection_matrix = camera.0.projection_matrix;
    let window = window_resource.get(camera.0.window)?;
    let window_size = Vec2::new(window.width(), window.height());
    // Build a transform to convert from world to NDC using camera data
    let world_to_ndc: Mat4 = projection_matrix * camera.1.compute_matrix().inverse();
    let ndc_space_coords: Vec3 = world_to_ndc.transform_point3(world_space_coords.translation);
    // NDC z-values outside of 0 < z < 1 are behind the camera and are thus not in screen space
    if ndc_space_coords.z < 0.0 || ndc_space_coords.z > 1.0 {
        return None;
    }
    // Once in NDC space, we can discard the z element and rescale x/y to fit the screen
    let screen_space_coords = (ndc_space_coords.truncate() + Vec2::one()) / 2.0 * window_size;
    Some(screen_space_coords)
}

struct FollowText;

// Unit struct to mark the 3d camera for queries (As opposed to the 2d UI camera)
struct ThreeDCam;

fn setup(
    commands: &mut Commands,
    asset_server: Res<AssetServer>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut meshes: ResMut<Assets<Mesh>>,
) {
    let font = asset_server.load("fonts/FiraSans-Bold.ttf");

    commands
        // ui camera
        .spawn(CameraUiBundle::default())
        .spawn(TextBundle {
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
            text: Text {
                value: "A Cube".to_string(),
                font,
                style: TextStyle {
                    font_size: 50.0,
                    color: Color::WHITE,
                    alignment: TextAlignment::default(),
                },
            },
            ..Default::default()
        })
        .with(FollowText);

    commands
        .spawn(Camera3dBundle {
            transform: Transform::from_matrix(Mat4::face_toward(
                Vec3::new(-3.0, 5.0, 8.0),
                Vec3::new(0.0, 0.0, 0.0),
                Vec3::new(0.0, 1.0, 0.0),
            )),
            ..Default::default()
        })
        .with(ThreeDCam)
        .with(FlyCamera::default())
        .spawn(PbrBundle {
            mesh: meshes.add(Mesh::from(shape::Cube { size: 1.0 })),
            material: materials.add(Color::rgb(1.0, 1.0, 1.0).into()),
            transform: Transform::from_translation(Vec3::new(0.0, 1.0, 0.0)),
            ..Default::default()
        })
        .spawn(LightBundle {
            transform: Transform::from_translation(Vec3::new(4.0, 8.0, 4.0)),
            ..Default::default()
        });
}
