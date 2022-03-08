use bevy::{
    prelude::*,
    input::keyboard::*,
    input::ElementState,
    sprite::collide_aabb::*,
};
use bevy_kira_audio::{Audio};

use crate::config::*;
use crate::components::*;
use crate::events::*;
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
    mut ball_hit_paddle_event: EventWriter<BallHitPaddleEvent>,
    collider_query: Query<(&Collider, &Transform, &SoundEmitter), Without<Ball>>,
    config: Res<Config>,
    audio: Res<Audio>,
) {
    for (collider, collider_transform, collider_sound) in collider_query.iter() {
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
                        Collision::Top => {
                            collision_resolved = false;
                            ball_hit_paddle_event.send(BallHitPaddleEvent());
                        }
                        Collision::Bottom => {
                            collision_resolved = false;
                            ball_hit_paddle_event.send(BallHitPaddleEvent());
                        }
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

                            ball_hit_paddle_event.send(BallHitPaddleEvent());
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

            audio.play(collider_sound.source.clone());
        }
    }
}

pub fn check_ball_out_system(
    mut ball_out_event: EventWriter<BallOutEvent>,
    mut ball_query: Query<&Transform, With<Ball>>,
    window: Res<WindowDescriptor>,
    config: Res<Config>,
    game_data: Res<GameData>,
) {
    if game_data.game_over.is_some() {
        return;
    }

    let ball_transform = ball_query.single_mut();

    if ball_transform.translation.x < -window.width / 2. - config.game_ball_oob_x {
        ball_out_event.send(BallOutEvent(Side::Left));
    } else if ball_transform.translation.x > window.width / 2. + config.game_ball_oob_x {
        ball_out_event.send(BallOutEvent(Side::Right));
    }
}

pub fn cleanup_entities<T: Component>(
    mut commands: Commands,
    query: Query<Entity, With<T>>,
) {
    for entity in query.iter() {
        commands.entity(entity).despawn_recursive();
    }
}

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
