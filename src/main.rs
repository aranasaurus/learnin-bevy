use bevy::{
    prelude::*,
    core::FixedTimestep,
    sprite::collide_aabb::{ collide, Collision },
};

use bevy_prototype_lyon::{
    prelude::*,
    entity::ShapeBundle
};

const SIZE_FACTOR: f32 = 42.0;
const COURT_COLOR: Color = Color::rgb(0.9, 0.9, 0.9);
const BALL_COLOR: Color = Color::rgb(0.9, 0.9, 0.9);
const PADDLE_COLOR: Color = Color::rgb(0.9, 0.9, 0.9);

fn main() {
    App::new()
        .insert_resource(Msaa { samples: 4 })
        .insert_resource(ClearColor(Color::rgb(0.1, 0.1, 0.1)))

        .add_plugins(DefaultPlugins)
        .add_plugin(ShapePlugin)

        .add_startup_system(setup_camera)
        .add_startup_system(setup_court)
        .add_startup_system(setup_ball)
        .add_startup_system(setup_paddles)

        .add_event::<CollisionEvent>()

        .add_system_set(
            SystemSet::new()
                .with_run_criteria(FixedTimestep::step(1.0/60.0))
                .with_system(paddle_control.label("controls"))
                .with_system(ball_movement.after("controls").label("movement"))
                .with_system(court_collisions.after("movement"))
                .with_system(paddle_ball_collisions.after("movement")),
        )
        .add_system(bounce)

        .run();
}

fn setup_camera(mut commands: Commands) {
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());
}

#[derive(Component)]
struct Court;

fn setup_court(mut commands: Commands, windows: Res<Windows>) {
    let window = windows.get_primary().unwrap();
    let ball_radius = ball_radius(window.width());

    let shape = shapes::Rectangle {
        origin: RectangleOrigin::Center,
        extents: Vec2::new(
            window.width() - ball_radius,
            window.height() - ball_radius
        )
    };

    commands
        .spawn_bundle(GeometryBuilder::build_as(
            &shape,
            DrawMode::Outlined {
                fill_mode: FillMode::color(Color::Rgba { alpha: 0.0, red: 0.0, green: 0.0, blue: 0.0 }),
                outline_mode: StrokeMode::new(COURT_COLOR, 7.0),
            },
            Transform::default(),
        ))
        .insert(BoundingBox { width: shape.extents.x, height: shape.extents.y })
        .insert(Court);
}

#[derive(Component)]
struct Ball;

#[derive(Component, Copy, Clone)]
struct Velocity {
    x: f32,
    y: f32
}

#[derive(Component)]
struct BoundingBox {
    width: f32,
    height: f32
}

impl BoundingBox {
    fn half_width(&self) -> f32 {
        self.width / 2.0
    }

    fn half_height(&self) -> f32 {
        self.height / 2.0
    }

    fn as_vec2(&self) -> Vec2 {
        Vec2::new(self.width, self.height)
    }
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
                0: FillMode::color(BALL_COLOR)
            },
            Transform::default())
        )
        .insert(Ball)
        .insert(Velocity { x: 200.0, y: 80.0 })
        .insert(BoundingBox {
            width: ball_radius * 2.0,
            height: ball_radius *  2.0,
        });
}

fn ball_movement(mut ball_q: Query<(&Velocity, &mut Transform), With<Ball>>, time: Res<Time>) {
    let (velocity, mut transform) = ball_q.single_mut();

    transform.translation.x += velocity.x * time.delta_seconds();
    transform.translation.y += velocity.y * time.delta_seconds();
}

struct CollisionEvent {
    entity: Entity,
    location: Vec2,
    other_velocity: Velocity
}

fn court_collisions(
    mut collision_event: EventWriter<CollisionEvent>,
    mut movables: Query<(&mut Transform, &BoundingBox, Entity), Without<Court>>,
    court_q: Query<(&Transform, &BoundingBox), With<Court>>
) {
    let (court_transform, court_box) = court_q.single();
    let court_right = court_transform.translation.x + court_box.width / 2.0;
    let court_left = court_transform.translation.x - court_box.width / 2.0;
    let court_top = court_transform.translation.y + court_box.height / 2.0;
    let court_bottom = court_transform.translation.y - court_box.height / 2.0;

    for (mut transform, bbox, entity) in movables.iter_mut() {
        let adjusted_right = court_right - bbox.width / 2.0;
        let adjusted_left = court_left + bbox.width / 2.0;
        let adjusted_top = court_top - bbox.height / 2.0;
        let adjusted_bottom = court_bottom + bbox.height / 2.0;

        let mut location = Vec2::ZERO;
        if transform.translation.x >= adjusted_right {
            transform.translation.x = adjusted_right;
            location.x = bbox.width / 2.0;
        } else if transform.translation.x <= adjusted_left {
            transform.translation.x = adjusted_left;
            location.x = -bbox.width / 2.0;
        }

        if transform.translation.y >= adjusted_top {
            transform.translation.y = adjusted_top;
            location.y = bbox.height / 2.0;
        } else if transform.translation.y <= adjusted_bottom {
            transform.translation.y = adjusted_bottom;
            location.y = -bbox.height / 2.0;
        }

        if location != Vec2::ZERO {
            collision_event.send(CollisionEvent { entity, location, other_velocity: Velocity { x: 0.0, y: 0.0 } });
        }
    }
}

fn bounce(
    mut bounceables: Query<&mut Velocity>,
    mut collision_event: EventReader<CollisionEvent>
) {
    for collision in collision_event.iter() {
        if let Ok(mut velocity) = bounceables.get_mut(collision.entity) {
            if velocity.x > 0.0 && collision.location.x > 0.0 {
                velocity.x *= -1.0;
            } else if velocity.x < 0.0 && collision.location.x < 0.0 {
                velocity.x *= -1.0;
            }

            if velocity.y > 0.0 && collision.location.y > 0.0 {
                velocity.y *= -1.0;
            } else if velocity.y < 0.0 && collision.location.y < 0.0 {
                velocity.y *= -1.0;
            }

            velocity.y += collision.other_velocity.y;
            velocity.x += collision.other_velocity.x;
        }
    }
}

#[derive(Component)]
struct Score(isize);

#[derive(Component)]
enum Player {
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
struct PlayerBundle {
    score: Score,
    player: Player,
    bounding_box: BoundingBox,
    velocity: Velocity,

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

fn paddle_control(
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

fn paddle_ball_collisions(
    mut collision_event: EventWriter<CollisionEvent>,
    mut ball_q: Query<(&mut Transform, &BoundingBox, Entity), With<Ball>>,
    paddle_q: Query<(&Transform, &BoundingBox, &Velocity), (With<Player>, Without<Ball>)>
) {
    let (mut ball_t, ball_bbox, ball_entity) = ball_q.single_mut();

    for (paddle_t, paddle_bbox, paddle_v) in paddle_q.iter() {
        if let Some(collision) = collide(
            paddle_t.translation,
            paddle_bbox.as_vec2(),
            ball_t.translation,
            ball_bbox.as_vec2(),
        ) {
            match collision {
                Collision::Left => {
                    ball_t.translation.x = paddle_t.translation.x + paddle_bbox.half_width() + ball_bbox.half_width();
                    collision_event.send(
                        CollisionEvent {
                            entity: ball_entity,
                            location: Vec2::new(-ball_bbox.half_width(), 0.0),
                            other_velocity: *paddle_v,
                        }
                    )
                },
                Collision::Right => {
                    ball_t.translation.x = paddle_t.translation.x - paddle_bbox.half_width() - ball_bbox.half_width();
                    collision_event.send(
                        CollisionEvent {
                            entity: ball_entity,
                            location: Vec2::new(ball_bbox.half_width(), 0.0),
                            other_velocity: *paddle_v,
                        }
                    )
                },
                Collision::Top => {
                    ball_t.translation.y = paddle_t.translation.y - paddle_bbox.half_height() - ball_bbox.half_height();
                    collision_event.send(
                        CollisionEvent {
                            entity: ball_entity,
                            location: Vec2::new(0.0, ball_bbox.half_height()),
                            other_velocity: *paddle_v,
                        }
                    )
                },
                Collision::Bottom => {
                    ball_t.translation.y = paddle_t.translation.y + paddle_bbox.half_height() + ball_bbox.half_height();
                    collision_event.send(
                        CollisionEvent {
                            entity: ball_entity,
                            location: Vec2::new(0.0, -ball_bbox.half_height()),
                            other_velocity: *paddle_v,
                        }
                    )
                }
            }
        }
    }
}