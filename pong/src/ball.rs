use crate::prelude::*;

use bevy_prototype_lyon::{
    prelude::*,
};

const BALL_COLOR: Color = Color::rgb(0.9, 0.9, 0.9);

#[derive(Component)]
pub struct Ball {
    pub is_active: bool
}

pub fn ball_radius(window_width: f32) -> f32 {
    window_width / SIZE_FACTOR / 1.333
}

pub fn setup_ball(mut commands: Commands, windows: Res<Windows>) {
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
        .insert(Ball { is_active: true })
        .insert(Velocity { x: 200.0, y: 80.0 })
        .insert(BoundingBox {
            width: ball_radius * 2.0,
            height: ball_radius *  2.0,
        });
}

pub fn ball_movement(mut ball_q: Query<(&Velocity, &mut Transform), With<Ball>>, time: Res<Time>) {
    let (velocity, mut transform) = ball_q.single_mut();

    transform.translation.x += velocity.x * time.delta_seconds();
    transform.translation.y += velocity.y * time.delta_seconds();
}

pub fn bounce(
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
