use crate::*;

pub struct CourtPlugin;

impl Plugin for CourtPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(setup_court)
            .add_system_set(
                SystemSet::on_update(GameState::Resetting)
                    .with_system(court_collisions),
            )
            .add_system_set(
                SystemSet::on_update(GameState::Serving)
                    .with_system(court_collisions),
            )
            .add_system_set(
                SystemSet::on_update(GameState::Playing)
                    .with_system(court_collisions.after(ball_movement)),
            );
    }
}

#[derive(Component)]
struct Court;

fn setup_court(mut commands: Commands, windows: Res<Windows>) {
    let window = windows.get_primary().unwrap();
    let ball_radius = Ball::calc_radius(window.width());

    let size = Vec2::new(window.width() - ball_radius, window.height() - ball_radius);
    let inner_size = Vec3::new(size.x - ball_radius, size.y - ball_radius, 1.0);

    commands.spawn_bundle(SpriteBundle {
        sprite: Sprite {
            color: Color::rgb(0.9, 0.9, 0.9),
            ..default()
        },
        transform: Transform {
            translation: Vec2::ZERO.extend(1.0),
            scale: size.extend(1.0),
            ..default()
        },
        ..default()
    });

    commands
        .spawn_bundle(SpriteBundle {
            sprite: Sprite {
                color: Color::rgb(0.1, 0.1, 0.1),
                ..default()
            },
            transform: Transform {
                translation: Vec2::ZERO.extend(1.0),
                scale: inner_size,
                ..default()
            },
            ..default()
        })
        .insert(BoundingBox {
            width: inner_size.x,
            height: inner_size.y,
        })
        .insert(Court);
}

fn court_collisions(
    mut collision_event: EventWriter<CollisionEvent>,
    mut scored_event: EventWriter<ScoredEvent>,
    mut collidables_q: Query<
        (&mut Transform, Option<&mut Ball>, &BoundingBox, Entity),
        Without<Court>,
    >,
    court_q: Query<(&Transform, &BoundingBox), With<Court>>,
) {
    let (court_transform, court_box) = court_q.single();
    let court_right = court_transform.translation.x + court_box.width / 2.0;
    let court_left = court_transform.translation.x - court_box.width / 2.0;
    let court_top = court_transform.translation.y + court_box.height / 2.0;
    let court_bottom = court_transform.translation.y - court_box.height / 2.0;

    for (mut transform, opt_ball, bbox, entity) in collidables_q.iter_mut() {
        let adjusted_right = court_right - bbox.width / 2.0;
        let adjusted_left = court_left + bbox.width / 2.0;
        let adjusted_top = court_top - bbox.height / 2.0;
        let adjusted_bottom = court_bottom + bbox.height / 2.0;

        let mut location = Vec2::ZERO;
        if let Some(_ball) = opt_ball {
            if transform.translation.x > adjusted_right {
                transform.translation.x = adjusted_right;
                location.x = bbox.width / 2.0;
                scored_event.send(ScoredEvent {
                    player: Player::Left,
                });
            } else if transform.translation.x < adjusted_left {
                transform.translation.x = adjusted_left;
                location.x = -bbox.width / 2.0;
                scored_event.send(ScoredEvent {
                    player: Player::Right,
                });
            }
        }

        if transform.translation.y >= adjusted_top {
            transform.translation.y = adjusted_top;
            location.y = bbox.height / 2.0;
        } else if transform.translation.y <= adjusted_bottom {
            transform.translation.y = adjusted_bottom;
            location.y = -bbox.height / 2.0;
        }

        if location != Vec2::ZERO {
            collision_event.send(CollisionEvent {
                entity,
                location,
                other_velocity: Velocity { x: 0.0, y: 0.0 },
            });
        }
    }
}
