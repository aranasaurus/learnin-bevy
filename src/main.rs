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
        .add_startup_system(setup_court)
        .add_startup_system(setup_ball)
        .add_startup_system(setup_paddles)
        .run();
}

fn setup_camera(mut commands: Commands) {
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());
}

#[derive(Component)]
struct Court;

#[derive(Component)]
struct Size {
    width: f32,
    height: f32,
}
impl Size {
    fn as_vec2(&self) -> Vec2 {
        Vec2::new(self.width, self.height)
    }
}

fn setup_court(mut commands: Commands, windows: Res<Windows>) {
    let window = windows.get_primary().unwrap();
    let ball_radius = ball_radius(window.width());
    let size = Size {
        width: window.width() - ball_radius,
        height: window.height() - ball_radius
    };

    let shape = shapes::Rectangle {
        origin: RectangleOrigin::Center,
        extents: size.as_vec2()
    };

    commands
        .spawn_bundle(GeometryBuilder::build_as(
            &shape,
            DrawMode::Outlined {
                fill_mode: FillMode::color(Color::Rgba { alpha: 0.0, red: 0.0, green: 0.0, blue: 0.0 }),
                outline_mode: StrokeMode::new(Color::WHITE, 10.0),
            },
            Transform::default(),
        ))
        .insert(size)
        .insert(Court);
}

#[derive(Component)]
struct Ball;

#[derive(Component)]
struct Velocity {
    x: f32,
    y: f32
}

fn ball_radius(window_width: f32) -> f32 {
    window_width / SIZE_FACTOR / 1.333
}

fn setup_ball(mut commands: Commands, windows: Res<Windows>) {
    let window = windows.get_primary().unwrap();
    let ball_radius = ball_radius(window.width());
    let shape = shapes::Circle {
        center: Vec2::new(0.0, 0.0),
        radius: ball_radius,
    };

    commands
        .spawn_bundle(GeometryBuilder::build_as(
            &shape,
            DrawMode::Fill {
                0: FillMode::color(Color::WHITE)
            },
            Transform::default())
        )
        .insert(Ball)
        .insert(Velocity { x: 10.0, y: 0.0 });
}
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
