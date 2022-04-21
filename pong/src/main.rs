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
pub const UI_HEIGHT: f32 = 66.0;

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

#[derive(Component)]
pub struct Scoreboard;

fn main() {
    App::new()
        .insert_resource(Msaa { samples: 4 })
        .insert_resource(ClearColor(Color::rgb(0.1, 0.1, 0.1)))
        .insert_resource(WindowDescriptor {
            title: "Pong".to_string(),
            present_mode: PresentMode::Fifo,
            width: 1200.0,
            height: 800.0,
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

fn setup_camera(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());
    commands.spawn_bundle(UiCameraBundle::default());
    let font = asset_server.load("fonts/FiraMono-Medium.ttf");
    let text_style = TextStyle {
        font,
        font_size: 60.0,
        color: Color::WHITE,
    };
    commands
        .spawn_bundle(NodeBundle {
            style: Style {
                size: Size::new(Val::Percent(100.0), Val::Percent(100.0)),
                position_type: PositionType::Absolute,
                justify_content: JustifyContent::Center,
                align_items: AlignItems::FlexEnd,
                ..Default::default()
            },
            color: UiColor::from(Color::NONE),
            ..Default::default()
        })
        .with_children(|parent| {
            parent.spawn_bundle(TextBundle {
                text: Text {
                    sections: vec![
                        TextSection {
                            value: "0".to_string(),
                            style: text_style.clone(),
                        },
                        TextSection {
                            value: " | ".to_string(),
                            style: text_style.clone(),
                        },
                        TextSection {
                            value: "0".to_string(),
                            style: text_style.clone(),
                        },
                    ],
                    ..default()
                },
                ..default()
            }).insert(Scoreboard);
        });
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
