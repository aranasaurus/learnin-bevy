use crate::prelude::*;

const PADDLE_COLOR: Color = Color::rgb(0.9, 0.9, 0.9);

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(setup_paddles)
            .add_system_set(
                SystemSet::on_update(GameState::Serving)
                    .with_system(paddle_control)
            )
            .add_system_set(
                SystemSet::on_update(GameState::Resetting)
                    .with_system(player_scored)
                    .with_system(paddle_control)
            )
            .add_system_set(
                SystemSet::on_update(GameState::Playing)
                    .with_system(player_scored)
                    .with_system(paddle_control)
            );
    }
}

#[derive(Component)]
struct Score(isize);

#[derive(Component, PartialEq, Debug)]
pub enum Player {
    Left,
    Right,
}

impl Player {
    fn move_up_key(&self) -> KeyCode {
        match self {
            Player::Left => KeyCode::W,
            Player::Right => KeyCode::Up,
        }
    }

    fn move_down_key(&self) -> KeyCode {
        match self {
            Player::Left => KeyCode::S,
            Player::Right => KeyCode::Down,
        }
    }

    fn push_right_key(&self) -> KeyCode {
        match self {
            Player::Left => KeyCode::D,
            Player::Right => KeyCode::Right,
        }
    }

    fn push_left_key(&self) -> KeyCode {
        match self {
            Player::Left => KeyCode::A,
            Player::Right => KeyCode::Left,
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
    sprite: SpriteBundle,
}

fn setup_paddles(mut commands: Commands, windows: Res<Windows>) {
    let window = windows.get_primary().unwrap();
    let paddle_width = window.width() / SIZE_FACTOR;
    let paddle_height = paddle_width * 6.0;
    let paddle_offset = window.width() / 2.0 - (paddle_width * 2.0);
    let size = Vec2::new(paddle_width, paddle_height);

    commands.spawn_bundle(PlayerBundle {
        score: Score(0),
        player: Player::Left,
        bounding_box: BoundingBox {
            width: size.x,
            height: size.y,
        },
        velocity: Velocity { x: 0.0, y: 0.0 },

        sprite: SpriteBundle {
            sprite: Sprite {
                color: PADDLE_COLOR,
                ..default()
            },
            transform: Transform {
                translation: Vec3::new(-paddle_offset, 0.0, 2.0),
                scale: size.extend(1.0),
                ..default()
            },
            ..default()
        },
    });

    commands.spawn_bundle(PlayerBundle {
        score: Score(0),
        player: Player::Right,
        bounding_box: BoundingBox {
            width: size.x,
            height: size.y,
        },
        velocity: Velocity { x: 0.0, y: 0.0 },

        sprite: SpriteBundle {
            sprite: Sprite {
                color: PADDLE_COLOR,
                ..default()
            },
            transform: Transform {
                translation: Vec3::new(paddle_offset, 0.0, 2.0),
                scale: size.extend(1.0),
                ..default()
            },
            ..default()
        },
    });
}

pub fn paddle_control(
    mut paddle_q: Query<(&Player, &mut Transform, &mut Velocity)>,
    keys: Res<Input<KeyCode>>,
    time: Res<Time>,
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

fn player_scored(
    mut score_q: Query<(&mut Score, &Player)>,
    mut score_event: EventReader<ScoredEvent>,
    mut state: ResMut<State<GameState>>,
) {
    for scored in score_event.iter() {
        for (mut score, player) in score_q.iter_mut() {
            if *player == scored.player {
                if *state.current() != GameState::Scored {
                    score.0 += 1;
                    state.set(GameState::Scored).unwrap();
                    println!("{:?}: {}", *player, score.0);
                }
            }
        }
    }
}
