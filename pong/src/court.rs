use crate::*;

#[derive(Component)]
pub struct Court;

pub fn setup_court(mut commands: Commands, windows: Res<Windows>) {
    let window = windows.get_primary().unwrap();
    let ball_radius = ball_radius(window.width());

    let size = Vec2::new(
        window.width() - ball_radius,
        window.height() - ball_radius
    );
    let inner_size = Vec3::new(size.x - ball_radius, size.y - ball_radius, 1.0);

    commands
        .spawn_bundle(
            SpriteBundle {
                sprite: Sprite {
                    color: Color::rgb(0.9, 0.9, 0.9),
                    ..Sprite::default()
                },
                transform: Transform {
                    translation: Vec2::ZERO.extend(1.0),
                    scale: size.extend(1.0),
                    ..Transform::default()
                },
                ..SpriteBundle::default()
            }
        );

    commands
        .spawn_bundle(
            SpriteBundle {
                sprite: Sprite {
                    color: Color::rgb(0.1, 0.1, 0.1),
                    ..Sprite::default()
                },
                transform: Transform {
                    translation: Vec2::ZERO.extend(1.0),
                    scale: inner_size,
                    ..Transform::default()
                },
                ..SpriteBundle::default()
            }
        )
        .insert(BoundingBox { width: inner_size.x, height: inner_size.y })
        .insert(Court);
}

pub fn court_collisions(
    mut collision_event: EventWriter<CollisionEvent>,
    mut scored_event: EventWriter<ScoredEvent>,
    mut ball_q: Query<(&mut Transform, &Ball, &BoundingBox, Entity), Without<Court>>,
    court_q: Query<(&Transform, &BoundingBox), With<Court>>
) {
    let (court_transform, court_box) = court_q.single();
    let court_right = court_transform.translation.x + court_box.width / 2.0;
    let court_left = court_transform.translation.x - court_box.width / 2.0;
    let court_top = court_transform.translation.y + court_box.height / 2.0;
    let court_bottom = court_transform.translation.y - court_box.height / 2.0;

    let (mut transform, ball, bbox, entity) = ball_q.single_mut();
    let adjusted_right = court_right - bbox.width / 2.0;
    let adjusted_left = court_left + bbox.width / 2.0;
    let adjusted_top = court_top - bbox.height / 2.0;
    let adjusted_bottom = court_bottom + bbox.height / 2.0;

    let mut location = Vec2::ZERO;
    if transform.translation.x > adjusted_right {
        transform.translation.x = adjusted_right;
        location.x = bbox.width / 2.0;
        if ball.is_active {
            scored_event.send(ScoredEvent { player: Player::Left });
        }
    } else if transform.translation.x < adjusted_left {
        transform.translation.x = adjusted_left;
        location.x = -bbox.width / 2.0;
        if ball.is_active {
            scored_event.send(ScoredEvent { player: Player::Right });
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
        collision_event.send(CollisionEvent { entity, location, other_velocity: Velocity { x: 0.0, y: 0.0 } });
    }
}
