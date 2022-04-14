use crate::*;

#[derive(Component)]
pub struct Court;

pub fn setup_court(mut commands: Commands, windows: Res<Windows>) {
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
                outline_mode: StrokeMode::new(Color::rgb(0.9, 0.9, 0.9), 7.0),
            },
            Transform::default(),
        ))
        .insert(BoundingBox { width: shape.extents.x, height: shape.extents.y })
        .insert(Court);
}

pub fn court_collisions(
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
