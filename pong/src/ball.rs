use crate::prelude::*;

const BALL_COLOR: Color = Color::rgb(0.9, 0.9, 0.9);

pub struct BallPlugin;

impl Plugin for BallPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(setup_ball)
            .add_system(bounce)
            .add_system_set(
                SystemSet::on_update(GameState::Playing)
                    .with_run_criteria(FixedTimestep::step(1.0 / 60.0))
                    .with_system(ball_movement.after(paddle_control))
            );
    }
}

#[derive(Component)]
pub struct Ball {
    pub is_active: bool,
}

impl Ball {
    pub fn calc_radius(window_width: f32) -> f32 {
        window_width / SIZE_FACTOR / 1.333
    }
}

fn setup_ball(mut commands: Commands, windows: Res<Windows>) {
    let window = windows.get_primary().unwrap();
    let ball_radius = Ball::calc_radius(window.width());
    let size = Vec2::splat(ball_radius * 2.0);

    commands
        .spawn_bundle(SpriteBundle {
            sprite: Sprite {
                color: BALL_COLOR,
                ..default()
            },
            transform: Transform {
                translation: Vec2::ZERO.extend(2.0),
                scale: size.extend(1.0),
                ..default()
            },
            ..default()
        })
        .insert(Ball { is_active: true })
        .insert(Velocity { x: 200.0, y: 80.0 })
        .insert(BoundingBox {
            width: size.x,
            height: size.y,
        });
}

pub fn ball_movement(mut ball_q: Query<(&Velocity, &mut Transform), With<Ball>>, time: Res<Time>) {
    let (velocity, mut transform) = ball_q.single_mut();

    transform.translation.x += velocity.x * time.delta_seconds();
    transform.translation.y += velocity.y * time.delta_seconds();
}

fn bounce(
    mut bounceables: Query<&mut Velocity>,
    mut collision_event: EventReader<CollisionEvent>,
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
