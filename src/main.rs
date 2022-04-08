use bevy::prelude::*;
use bevy_prototype_lyon::entity::ShapeBundle;
use bevy_prototype_lyon::prelude::*;

const SIZE_FACTOR: f32 = 42.0;

fn main() {
    App::new()
        .insert_resource(Msaa { samples: 4 })

        .add_plugins(DefaultPlugins)
        .add_plugin(ShapePlugin)

        .add_startup_system(setup_camera)
        .add_startup_system(setup_ball)
        .add_startup_system(setup_paddles)
        .run();
}

fn setup_camera(mut commands: Commands) {
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());
}

#[derive(Component)]
struct Ball;

fn setup_ball(mut commands: Commands, windows: Res<Windows>) {
    let window = windows.get_primary().unwrap();
    let ball_radius = window.width() / SIZE_FACTOR / 1.333;
    let shape = shapes::Circle {
        center: Vec2::new(0.0, 0.0),
        radius: ball_radius,
    };

    commands.spawn_bundle(GeometryBuilder::build_as(
        &shape,
        DrawMode::Fill {
            0: FillMode::color(Color::WHITE)
        },
        Transform::default()
    )).insert(Ball);
}

#[derive(Component)]
struct Score(isize);

#[derive(Component)]
enum Player {
    Left, Right
}

#[derive(Bundle)]
struct PlayerBundle {
    score: Score,
    player: Player,

    #[bundle]
    shape: ShapeBundle,
}

fn setup_paddles(mut commands: Commands, windows: Res<Windows>) {
    let window = windows.get_primary().unwrap();
    let paddle_width = window.width() / SIZE_FACTOR;
    let paddle_height = paddle_width * 6.0;
    let paddle_offset = window.width() / 2.0 - (paddle_width * 2.0);
    let shape = shapes::Rectangle {
        extents: Vec2::new(paddle_width, paddle_height),
        origin: RectangleOrigin::Center
    };

    commands.spawn_bundle(PlayerBundle {
        score: Score(0),
        player: Player::Left,

        shape: GeometryBuilder::build_as(
            &shape,
            DrawMode::Fill {
                0: FillMode::color(Color::WHITE)
            },
            Transform::from_xyz(-paddle_offset, 0.0, 0.0),
        )
    });

    commands.spawn_bundle(PlayerBundle {
        score: Score(0),
        player: Player::Right,

        shape: GeometryBuilder::build_as(
            &shape,
            DrawMode::Fill {
                0: FillMode::color(Color::WHITE)
            },
            Transform::from_xyz(paddle_offset, 0.0, 0.0),
        )
    });
}
