use rand::Rng;

use bevy::{
    prelude::*,
    sprite::collide_aabb::{collide, Collision},
    window::PresentMode,
};

use crate::ball::*;
use crate::court::*;
use crate::paddles::*;

mod ball;
mod court;
mod paddles;
mod prelude {
    pub use crate::*;
    pub use bevy::prelude::*;
}

pub const SIZE_FACTOR: f32 = 42.0;

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub enum GameState {
    Resetting,
    Serving,
    Playing,
    Scored,
    GameOver
}

pub struct CollisionEvent {
    entity: Entity,
    location: Vec2,
    other_velocity: Velocity,
}

pub struct ScoredEvent {
    player: Player,
}

#[derive(Component)]
pub struct BoundingBox {
    width: f32,
    height: f32,
}

impl BoundingBox {
    pub fn half_width(&self) -> f32 {
        self.width / 2.0
    }

    pub fn half_height(&self) -> f32 {
        self.height / 2.0
    }

    pub fn as_vec2(&self) -> Vec2 {
        Vec2::new(self.width, self.height)
    }
}

#[derive(Component, Copy, Clone)]
pub struct Velocity {
    x: f32,
    y: f32,
}

impl Velocity {
    pub fn random() -> Velocity {
        let mut rng = rand::thread_rng();
        let mut x = rng.gen_range(200.0..400.0);
        if rng.gen_bool(0.5) {
            x *= -1.0;
        }

        let mut y = rng.gen_range(80.0..300.0);
        if rng.gen_bool(0.5) {
            y *= -1.0;
        }

        Velocity { x, y }
    }
}

fn main() {
    App::new()
        .insert_resource(Msaa { samples: 4 })
        .insert_resource(ClearColor(Color::rgb(0.1, 0.1, 0.1)))
        .insert_resource(WindowDescriptor {
            title: "Pong".to_string(),
            present_mode: PresentMode::Fifo,
            ..default()
        })
        .add_plugins(DefaultPlugins)
        .add_plugin(BallPlugin)
        .add_plugin(CourtPlugin)
        .add_plugin(PlayerPlugin)
        .add_state(GameState::Serving)
        .add_startup_system(setup_camera)
        .add_event::<CollisionEvent>()
        .add_event::<ScoredEvent>()
        .add_system_set(
            SystemSet::on_update(GameState::Playing)
                .with_system(paddle_control)
                .with_system(paddle_ball_collisions.after(ball_movement)),
        )
        .run();
}

fn setup_camera(mut commands: Commands) {
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());
}

fn paddle_ball_collisions(
    mut collision_event: EventWriter<CollisionEvent>,
    mut ball_q: Query<(&mut Transform, &BoundingBox, Entity), With<Ball>>,
    paddle_q: Query<(&Transform, &BoundingBox, &Velocity), (With<Player>, Without<Ball>)>,
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
                    ball_t.translation.x =
                        paddle_t.translation.x + paddle_bbox.half_width() + ball_bbox.half_width();
                    collision_event.send(CollisionEvent {
                        entity: ball_entity,
                        location: Vec2::new(-ball_bbox.half_width(), 0.0),
                        other_velocity: *paddle_v,
                    })
                }
                Collision::Right => {
                    ball_t.translation.x =
                        paddle_t.translation.x - paddle_bbox.half_width() - ball_bbox.half_width();
                    collision_event.send(CollisionEvent {
                        entity: ball_entity,
                        location: Vec2::new(ball_bbox.half_width(), 0.0),
                        other_velocity: *paddle_v,
                    })
                }
                Collision::Top => {
                    ball_t.translation.y = paddle_t.translation.y
                        - paddle_bbox.half_height()
                        - ball_bbox.half_height();
                    collision_event.send(CollisionEvent {
                        entity: ball_entity,
                        location: Vec2::new(0.0, ball_bbox.half_height()),
                        other_velocity: *paddle_v,
                    })
                }
                Collision::Bottom => {
                    ball_t.translation.y = paddle_t.translation.y
                        + paddle_bbox.half_height()
                        + ball_bbox.half_height();
                    collision_event.send(CollisionEvent {
                        entity: ball_entity,
                        location: Vec2::new(0.0, -ball_bbox.half_height()),
                        other_velocity: *paddle_v,
                    })
                }
                Collision::Inside => {} // Shouldn't be possible...
            }
        }
    }
}
