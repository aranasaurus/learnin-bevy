use std::time::Duration;
use crate::prelude::*;

const BALL_COLOR: Color = Color::rgb(0.9, 0.9, 0.9);

pub struct BallPlugin;

impl Plugin for BallPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(setup_ball)
            .add_system(bounce)
            .add_system_set(
                SystemSet::on_enter(GameState::Serving)
                    .with_system(serving_ball_enter)
            )
            .add_system_set(
                SystemSet::on_update(GameState::Serving)
                    .with_system(serve_ball)
                    .with_system(blink_ball)
            )
            .add_system_set(
                SystemSet::on_enter(GameState::Scored)
                    .with_system(scored_ball_enter)
            )
            .add_system_set(
                SystemSet::on_update(GameState::Scored)
                    .with_system(scored_ball_update)
                    .with_system(blink_ball)
            )
            .add_system_set(
                SystemSet::on_enter(GameState::Resetting)
                    .with_system(reset_ball_enter)
            )
            .add_system_set(
                SystemSet::on_update(GameState::Resetting)
                    .with_system(reset_ball_update)
                    .with_system(blink_ball)
            )
            .add_system_set(
                SystemSet::on_update(GameState::Playing)
                    .with_system(ball_movement.after(paddle_control))
            );
    }
}

#[derive(Component)]
pub struct Ball {
    pub hold_timer: Timer,
    pub blink_timer: Timer,
    pub reset_timer: Timer
}

impl Ball {
    pub fn calc_radius(window_width: f32) -> f32 {
        window_width / SIZE_FACTOR / 1.333
    }
    pub fn reset_animation(&self, name: &Name, transform: &Transform, duration_seconds: f32) -> AnimationClip {
        // Creating the animation
        let mut animation = AnimationClip::default();
        // A curve can modify a single part of a transform, here the translation
        animation.add_curve_to_path(
            EntityPath {
                parts: vec![name.clone()],
            },
            VariableCurve {
                keyframe_timestamps: vec![0.0, duration_seconds],
                keyframes: Keyframes::Translation(vec![
                    transform.translation,
                    Vec3::new(0.0, 0.0, 2.0),
                ]),
            },
        );
        animation
    }
}

fn setup_ball(mut commands: Commands, windows: Res<Windows>) {
    let window = windows.get_primary().unwrap();
    let ball_radius = Ball::calc_radius(window.width());
    let size = Vec2::splat(ball_radius * 2.0);

    // The animation API uses the `Name` component to target entities
    let name = Name::new("ball");
    let player = AnimationPlayer::default();

    commands
        .spawn_bundle(SpriteBundle {
            sprite: Sprite {
                color: BALL_COLOR,
                ..default()
            },
            transform: Transform {
                translation: Vec3::new(0.0, 0.0, 2.0),
                scale: size.extend(1.0),
                ..default()
            },
            ..default()
        })
        .insert(Ball {
            hold_timer: Timer::from_seconds(1.0, false),
            blink_timer: Timer::from_seconds(0.25, false),
            reset_timer: Timer::from_seconds(2.0, false)
        })
        .insert(Velocity { x: 200.0, y: 80.0 })
        .insert(Visibility { is_visible: true })
        .insert(name)
        .insert(player)
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

fn reset_ball_enter(
    mut ball_q: Query<(&mut Ball, &Transform, &Name, &mut AnimationPlayer)>,
    mut animations: ResMut<Assets<AnimationClip>>
) {
    let (mut ball, transform, name, mut player) = ball_q.single_mut();
    let animation = animations.add(ball.reset_animation(name, transform, ball.reset_timer.duration().as_secs_f32()));
    player.play(animation);
    ball.reset_timer.reset();
    ball.reset_timer.unpause();

    ball.blink_timer.set_duration(Duration::from_secs_f32(0.33));
    ball.blink_timer.reset();
    ball.blink_timer.unpause();
}

fn reset_ball_update(
    mut ball_q: Query<&mut Ball>,
    mut state: ResMut<State<GameState>>,
    time: Res<Time>,
) {
    let mut ball = ball_q.single_mut();
    ball.reset_timer.tick(time.delta());
    if ball.reset_timer.finished() {
        ball.reset_timer.pause();
        ball.reset_timer.reset();
        state.set(GameState::Serving).unwrap();
        return;
    }
}

fn blink_ball(
    mut ball_q: Query<(&mut Ball, &mut Visibility)>,
    time: Res<Time>,
) {
    let (mut ball, mut visibility)  = ball_q.single_mut();
    ball.blink_timer.tick(time.delta());

    if ball.blink_timer.just_finished() {
        visibility.is_visible = !visibility.is_visible;
        ball.blink_timer.reset();
    }
}

fn serving_ball_enter(
    mut ball_q: Query<&mut Ball>,
    mut animations: ResMut<Assets<AnimationClip>>
) {
    animations.clear();
    let mut ball = ball_q.single_mut();
    ball.blink_timer.pause();
    ball.blink_timer.set_duration(Duration::from_secs_f32(0.16));
    ball.blink_timer.unpause();
}

fn serve_ball(
    mut ball_q: Query<(&mut Ball, &mut Velocity, &mut Visibility)>,
    keys: Res<Input<KeyCode>>,
    mut state: ResMut<State<GameState>>,
) {
    let (mut ball, mut velocity, mut visiblity) = ball_q.single_mut();
    if keys.just_released(KeyCode::Space) {
        velocity.x = velocity.x.clamp(-200.0,200.0);
        velocity.y = velocity.y.clamp(-200.0,200.0);
        ball.blink_timer.pause();
        visiblity.is_visible = true;
        state.set(GameState::Playing).unwrap();
    }
}

fn scored_ball_enter(
    mut ball_q: Query<&mut Ball>,
) {
    let mut ball = ball_q.single_mut();
    ball.hold_timer.reset();
    ball.hold_timer.unpause();

    ball.blink_timer.reset();
    ball.blink_timer.set_duration(Duration::from_secs_f32(0.08));
    ball.blink_timer.unpause();
}

fn scored_ball_update(
    mut ball_q: Query<&mut Ball>,
    mut state: ResMut<State<GameState>>,
    time: Res<Time>,
) {
    let mut ball = ball_q.single_mut();
    ball.hold_timer.tick(time.delta());

    if ball.hold_timer.finished() {
        ball.hold_timer.reset();
        ball.hold_timer.pause();
        state.set(GameState::Resetting).unwrap();
    }
}
