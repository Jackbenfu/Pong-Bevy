use bevy::{
    prelude::*,
    input::keyboard::*,
    input::ElementState,
    sprite::collide_aabb::*,
};

use crate::config::*;
use crate::components::*;
use crate::state::*;

pub fn move_left_paddle_with_keyboard_system(
    mut paddle_query: Query<(&LeftPaddle, &mut Transform)>,
    keyboard: Res<Input<KeyCode>>,
    window: Res<WindowDescriptor>,
    time: Res<Time>,
    config: Res<Config>,
) {
    let mut direction = 0.;

    if keyboard.pressed(KeyCode::S) {
        direction += 1.;
    }

    if keyboard.pressed(KeyCode::X) {
        direction -= 1.;
    }

    let (paddle_entity, mut paddle_transform) = paddle_query.single_mut();
    let bound_y = window.height / 2. - config.sprite_unit_size - paddle_transform.scale.y / 2.;

    let translation = &mut paddle_transform.translation;
    translation.y += direction * paddle_entity.speed * time.delta_seconds();
    translation.y = translation.y.min(bound_y).max(-bound_y);
}

pub fn move_ball_system(
    mut ball_query: Query<(&Ball, &mut Transform)>,
    time: Res<Time>,
) {
    let (ball, mut transform) = ball_query.single_mut();
    let velocity = ball.velocity;

    if velocity.x != 0. || velocity.y != 0. {
        transform.translation += velocity * time.delta_seconds();
    }
}

pub fn check_ball_collision_system(
    mut ball_query: Query<(&mut Ball, &mut Transform)>,
    collider_query: Query<(&Collider, &Transform), Without<Ball>>,
    config: Res<Config>,
) {
    for (collider, collider_transform) in collider_query.iter() {
        let (mut ball, mut ball_transform) = ball_query.single_mut();

        let bx = ball_transform.translation.x;
        let by = ball_transform.translation.y;
        let bwh = ball_transform.scale.x / 2.;
        let bhh = ball_transform.scale.y / 2.;

        let px = collider_transform.translation.x;
        let py = collider_transform.translation.y;
        let pwh = collider_transform.scale.x / 2.;
        let phh = collider_transform.scale.y / 2.;

        if !(bx - bwh >= px + pwh || bx + bwh <= px - pwh || by - bhh >= py + phh || by + bhh <= py - phh) {
            let velocity1 = ball.velocity;
            let v_x1 = velocity1.x;
            let v_y1 = velocity1.y;

            // Required move to go back to the position just before the collision
            let x_to_collision = if v_x1 > 0. { (px - pwh) - (bx + bwh) } else { (px + pwh) - (bx - bwh) };
            let y_to_collision = if v_y1 > 0. { (py - phh) - (by + bhh) } else { (py + phh) - (by - bhh) };

            // Same as above expressed in percentage (value from 0 to 1)
            let x_offset_to_collision = if 0. == v_x1 { -f32::INFINITY } else { x_to_collision / v_x1 };
            let y_offset_to_collision = if 0. == v_y1 { -f32::INFINITY } else { y_to_collision / v_y1 };

            // Collision time is the latest among the two axes
            let collision_time = x_offset_to_collision.max(y_offset_to_collision);

            // Collision normals to find on which AABB side the collision occurred
            let normal_x: f32;
            let normal_y: f32;
            let collision_side: Collision;
            if x_offset_to_collision > y_offset_to_collision {
                normal_x = if x_to_collision < 0. { -1. } else { 1. };
                normal_y = 0.;

                collision_side = if -1. == normal_x { Collision::Left } else { Collision::Right };
            } else {
                normal_y = if y_to_collision < 0. { -1. } else { 1. };
                normal_x = 0.;

                collision_side = if -1. == normal_y { Collision::Top } else { Collision::Bottom };
            }

            // Position where the collision occurred
            let x_collision = bx + v_x1 * collision_time;
            let y_collision = by + v_y1 * collision_time;

            ball_transform.translation.x = x_collision;
            ball_transform.translation.y = y_collision;

            let collision_resolved: bool;
            match *collider {
                Collider::Paddle => {
                    match collision_side {
                        Collision::Top => { collision_resolved = false }
                        Collision::Bottom => { collision_resolved = false }
                        _ => {
                            let hit_factor = (ball_transform.translation.y - collider_transform.translation.y) / collider_transform.scale.y;

                            let mut new_ball_vel = Vec3::default();
                            new_ball_vel.x = if ball.velocity.x > 0. { -1. } else { 1. };
                            new_ball_vel.y = hit_factor * 2.;
                            new_ball_vel = new_ball_vel.normalize();

                            ball.velocity.x = new_ball_vel.x * ball.speed;
                            ball.velocity.y = new_ball_vel.y * ball.speed;

                            if config.game_ball_speed_max > ball.speed {
                                ball.speed += config.game_ball_speed_incr;
                            }

                            collision_resolved = true;
                        }
                    }
                }
                _ => {
                    collision_resolved = false
                }
            }

            // Default behavior if collision not resolved by user
            if !collision_resolved
            {
                // Setting new velocity for "bounce" effect
                if 0. != normal_x
                {
                    ball.velocity.x = -v_x1;
                }

                if 0. != normal_y
                {
                    ball.velocity.y = -v_y1;
                }
            }
        }
    }
}

pub fn check_ball_out_system(
    mut commands: Commands,
    mut game_over_event: EventWriter<GameOverEvent>,
    mut ball_query: Query<(&mut Ball, &Transform)>,
    mut left_paddle_query: Query<(Entity, &mut Transform), (With<LeftPaddle>, Without<RightPaddle>, Without<Ball>)>,
    mut right_paddle_query: Query<(Entity, &mut Transform), (With<RightPaddle>, Without<LeftPaddle>, Without<Ball>)>,
    mut left_score_query: Query<&mut Text, (With<LeftScore>, Without<RightScore>)>,
    mut right_score_query: Query<&mut Text, (With<RightScore>, Without<LeftScore>)>,
    mut game_data: ResMut<GameData>,
    window: Res<WindowDescriptor>,
    config: Res<Config>,
) {
    if game_data.game_over.is_some() {
        return;
    }

    let mut ball_out = false;

    let (mut ball, ball_transform) = ball_query.single_mut();
    let (right_paddle_entity, mut right_paddle_transform) = right_paddle_query.single_mut();
    let (left_paddle_entity, mut left_paddle_transform) = left_paddle_query.single_mut();

    if ball_transform.translation.x < -window.width / 2. - config.game_ball_oob_x {
        game_data.right_score += 1;

        if game_data.right_score == config.game_score_to_win {
            game_data.game_over = Some(Side::Right);
        } else {
            commands.entity(right_paddle_entity).insert(Service {});
        }

        right_score_query.single_mut().sections[0].value = format!("{}", game_data.right_score);
        ball_out = true;
    } else if ball_transform.translation.x > window.width / 2. + config.game_ball_oob_x {
        game_data.left_score += 1;

        if game_data.left_score == config.game_score_to_win {
            game_data.game_over = Some(Side::Left);
        } else {
            commands.entity(left_paddle_entity).insert(Service {});
        }

        left_score_query.single_mut().sections[0].value = format!("{}", game_data.left_score);
        ball_out = true;
    }

    match &game_data.game_over {
        None => {
            if ball_out {
                left_paddle_transform.translation.y = 0.;
                right_paddle_transform.translation.y = 0.;
                ball.velocity = Vec3::default();
            }
        }
        Some(side) => {
            game_over_event.send(GameOverEvent(*side))
        }
    }
}

/// Despawns all entities that have a component of type T.
pub fn cleanup_entities<T: Component>(
    mut commands: Commands,
    query: Query<Entity, With<T>>,
) {
    for entity in query.iter() {
        commands.entity(entity).despawn_recursive();
    }
}

/// Navigates from game modes to main menu with Escape key.
pub fn back_to_menu_system(
    mut state: ResMut<State<GameState>>,
    mut keyboard_input_events: EventReader<KeyboardInput>,
) {
    for event in keyboard_input_events.iter() {
        if let Some(key_code) = event.key_code {
            if event.state == ElementState::Released && key_code == KeyCode::Escape {
                state.set(GameState::Menu).unwrap();
            }
        }
    }
}
