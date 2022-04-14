use crate::prelude::*;

const PADDLE_COLOR: Color = Color::rgb(0.9, 0.9, 0.9);

#[derive(Component)]
pub struct Score(isize);

#[derive(Component)]
pub enum Player {
    Left, Right
}

impl Player {
    fn move_up_key(&self) -> KeyCode {
        match self {
            Player::Left => KeyCode::W,
            Player::Right => KeyCode::Up
        }
    }

    fn move_down_key(&self) -> KeyCode {
        match self {
            Player::Left => KeyCode::S,
            Player::Right => KeyCode::Down
        }
    }

    fn push_right_key(&self) -> KeyCode {
        match self {
            Player::Left => KeyCode::D,
            Player::Right => KeyCode::Right
        }
    }

    fn push_left_key(&self) -> KeyCode {
        match self {
            Player::Left => KeyCode::A,
            Player::Right => KeyCode::Left
        }
    }
}

#[derive(Bundle)]
pub struct PlayerBundle {
    score: Score,
    player: Player,
    bounding_box: BoundingBox,
    velocity: Velocity,

    #[bundle]
    shape: ShapeBundle,
}

pub fn setup_paddles(mut commands: Commands, windows: Res<Windows>) {
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
        bounding_box: BoundingBox { width: shape.extents.x, height: shape.extents.y },
        velocity: Velocity { x: 0.0, y: 0.0 },

        shape: GeometryBuilder::build_as(
            &shape,
            DrawMode::Fill {
                0: FillMode::color(PADDLE_COLOR)
            },
            Transform::from_xyz(-paddle_offset, 0.0, 0.0),
        )
    });

    commands.spawn_bundle(PlayerBundle {
        score: Score(0),
        player: Player::Right,
        bounding_box: BoundingBox { width: shape.extents.x, height: shape.extents.y },
        velocity: Velocity { x: 0.0, y: 0.0 },

        shape: GeometryBuilder::build_as(
            &shape,
            DrawMode::Fill {
                0: FillMode::color(PADDLE_COLOR)
            },
            Transform::from_xyz(paddle_offset, 0.0, 0.0),
        )
    });
}

pub fn paddle_control(
    mut paddle_q: Query<(&Player, &mut Transform, &mut Velocity)>,
    keys: Res<Input<KeyCode>>,
    time: Res<Time>
) {
    let player_speed = 300.0;
    let push_speed = 133.0;
    for (player, mut transform, mut velocity) in paddle_q.iter_mut() {
        if keys.pressed(player.move_up_key()) {
            transform.translation.y += player_speed * time.delta_seconds();
            velocity.y = player_speed;
        } else if keys.pressed(player.move_down_key()) {
            transform.translation.y -= player_speed * time.delta_seconds();
            velocity.y = -player_speed;
        } else {
            velocity.y = 0.0;
        }

        if keys.pressed(player.push_right_key()) {
            velocity.x = push_speed;
        } else if keys.pressed(player.push_left_key()) {
            velocity.x = -push_speed;
        } else {
            velocity.x = 0.0;
        }
    }
}