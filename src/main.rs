mod aabb;
mod components;
mod image;
mod tags;

use aabb::IAabb;
use bevy::prelude::*;
use components::{Actor, CollisionStatus, Dynamic, Static};
use image::image_1px;

const COLLISIONS: &str = "collisions";

fn main() {
    let mut app = App::new();
    app.insert_resource(ClearColor(Color::rgb(0.767, 0.773, 0.616)))
        .insert_resource(Msaa { samples: 4 })
        .add_plugins(DefaultPlugins)
        .add_stage_after(CoreStage::Update, COLLISIONS, SystemStage::parallel())
        .add_startup_system(setup)
        .add_system_to_stage(COLLISIONS, update_position_delta.label("position_update"))
        .add_system_to_stage(COLLISIONS, resolve_collisions.after("position_update"))
        .add_system(sync_dynamic_position)
        .add_system(apply_gravity)
        .add_system(player_control);
    app.run();
}

fn setup(mut cmd: Commands, mut images: ResMut<Assets<Image>>, mut windows: ResMut<Windows>) {
    let static_img = images.add(image_1px(Color::rgb(0.12, 0.224, 0.299)));
    let actor_img = images.add(image_1px(Color::rgb(0.773, 0.205, 0.034)));
    {
        let static_aabb = IAabb::new(IVec2::new(500, 15), IVec2::new(0, -300));
        cmd.spawn_bundle(SpriteBundle {
            texture: static_img.clone(),
            transform: Transform::from_scale(Vec3::new(
                static_aabb.halfs.x as f32 * 2.0,
                static_aabb.halfs.y as f32 * 2.0,
                1.0,
            ))
            .with_translation(static_aabb.position.as_vec2().extend(0.0)),
            ..Default::default()
        })
        .insert(Static { aabb: static_aabb });
    }
    {
        let static_aabb = IAabb::new(IVec2::new(100, 15), IVec2::new(0, -280));
        cmd.spawn_bundle(SpriteBundle {
            texture: static_img.clone(),
            transform: Transform::from_scale(Vec3::new(
                static_aabb.halfs.x as f32 * 2.0,
                static_aabb.halfs.y as f32 * 2.0,
                1.0,
            ))
            .with_translation(static_aabb.position.as_vec2().extend(0.0)),
            ..Default::default()
        })
        .insert(Static { aabb: static_aabb });
    }
    {
        let static_aabb = IAabb::new(IVec2::new(15, 200), IVec2::new(-500, -100));
        cmd.spawn_bundle(SpriteBundle {
            texture: static_img.clone(),
            transform: Transform::from_scale(Vec3::new(
                static_aabb.halfs.x as f32 * 2.0,
                static_aabb.halfs.y as f32 * 2.0,
                1.0,
            ))
            .with_translation(static_aabb.position.as_vec2().extend(0.0)),
            ..Default::default()
        })
        .insert(Static { aabb: static_aabb });
    }
    {
        let static_aabb = IAabb::new(IVec2::new(100, 15), IVec2::new(-300, -150));
        cmd.spawn_bundle(SpriteBundle {
            texture: static_img,
            transform: Transform::from_scale(Vec3::new(
                static_aabb.halfs.x as f32 * 2.0,
                static_aabb.halfs.y as f32 * 2.0,
                1.0,
            ))
            .with_translation(static_aabb.position.as_vec2().extend(0.0)),
            ..Default::default()
        })
        .insert(Static { aabb: static_aabb });
    }
    {
        let actor_aabb = IAabb::new(IVec2::new(15, 30), IVec2::new(0, -100));
        cmd.spawn_bundle(SpriteBundle {
            texture: actor_img,
            transform: Transform::from_scale(Vec3::new(
                actor_aabb.halfs.x as f32 * 2.0,
                actor_aabb.halfs.y as f32 * 2.0,
                1.0,
            )),
            ..Default::default()
        })
        .insert(Dynamic { aabb: actor_aabb })
        .insert(Actor {
            max_speed: 2000.0,
            acceleration: Vec2::ZERO,
            velocity: Vec2::ZERO,
            dp: Vec2::ZERO,
            collision: CollisionStatus::FreeFall,
        });
    }
    windows
        .get_primary_mut()
        .unwrap()
        .set_title("bevy: platform1".to_owned());
    cmd.spawn_bundle(OrthographicCameraBundle::new_2d());
}

fn sync_dynamic_position(mut dynamic_q: Query<(&mut Transform, &Dynamic)>) {
    for (mut t, d) in dynamic_q.iter_mut() {
        t.translation.x = d.aabb.position.x as f32;
        t.translation.y = d.aabb.position.y as f32;
    }
}

fn apply_gravity(mut actors_q: Query<&mut Actor>) {
    let gravity = Vec2::new(0.0, -3000.0);
    for mut a in actors_q.iter_mut() {
        if a.collision != CollisionStatus::Standing {
            a.acceleration += gravity;
        }
    }
}

fn player_control(input: Res<Input<KeyCode>>, mut actors_q: Query<&mut Actor>, time: Res<Time>) {
    let speed = 500.0;
    let jump_speed = 900.0;
    for mut a in actors_q.iter_mut() {
        // todo: try implement moving with acceleration and max speed
        let mut velocity = Vec2::ZERO;
        if input.pressed(KeyCode::A) {
            velocity.x += -1.0;
        };
        if input.pressed(KeyCode::D) {
            velocity.x += 1.0;
        }
        if input.just_pressed(KeyCode::Space) && a.collision == CollisionStatus::Standing {
            a.velocity = Vec2::Y * jump_speed;
        }
        a.dp += velocity * speed * time.delta_seconds();
    }
}

fn update_position_delta(mut actors_q: Query<&mut Actor>, time: Res<Time>) {
    for mut a in actors_q.iter_mut() {
        let acceleration = a.acceleration;
        a.velocity += acceleration * time.delta_seconds();
        a.velocity = a.velocity.clamp_length_max(a.max_speed);
        let v = a.velocity;
        a.dp += v * time.delta_seconds();
        // reset acceleration for next tick
        a.acceleration = Vec2::ZERO;
    }
}

#[inline]
fn is_intersect(aabb: &IAabb, static_q: &Query<&Static>) -> bool {
    for s in static_q.iter() {
        if aabb.is_intersect(&s.aabb) {
            return true;
        }
    }
    false
}

// todo: debug view for all collision statuses
// todo: try what if assign big diagonal dp?
fn resolve_collisions(mut actors_q: Query<(&mut Actor, &mut Dynamic)>, static_q: Query<&Static>) {
    for (mut a, mut d) in actors_q.iter_mut() {
        let x_abs = a.dp.x.abs();
        let y_abs = a.dp.y.abs();
        if x_abs < 1.0 && y_abs < 1.0 {
            continue;
        }
        let (dx, dy, steps) = if x_abs >= y_abs {
            let fraction = x_abs.trunc() / x_abs;
            (
                a.dp.x.signum(),
                a.dp.y * fraction / x_abs.trunc(),
                x_abs as u32,
            )
        } else {
            let fraction = y_abs.trunc() / y_abs;
            (
                a.dp.x * fraction / y_abs.trunc(),
                a.dp.y.signum(),
                y_abs as u32,
            )
        };
        let mut next_aabb = d.aabb.clone();
        let mut safe_pos = next_aabb.position;
        let mut collision_along_x = false;
        let mut collision_along_y = false;
        // start from first, because for i=0 is current actor position
        for i in 1..=steps {
            next_aabb.position = IVec2::new(
                d.aabb.position.x + (dx * (i as f32)) as i32,
                d.aabb.position.y + (dy * (i as f32)) as i32,
            );
            if is_intersect(&next_aabb, &static_q) {
                // move along x
                next_aabb.position = safe_pos;
                for ix in i..=steps {
                    next_aabb.position.x = d.aabb.position.x + (dx * (ix as f32)) as i32;
                    if is_intersect(&next_aabb, &static_q) {
                        collision_along_x = true;
                        break;
                    }
                    safe_pos = next_aabb.position;
                }
                // move along y
                next_aabb.position = safe_pos;
                for iy in i..=steps {
                    next_aabb.position.y = d.aabb.position.y + (dy * (iy as f32)) as i32;
                    if is_intersect(&next_aabb, &static_q) {
                        collision_along_y = true;
                        break;
                    }
                    safe_pos = next_aabb.position;
                }
                break;
            }
            safe_pos = next_aabb.position;
        }
        // check if standing on the ground
        let is_standing = {
            next_aabb.position = safe_pos;
            next_aabb.position.y -= 1;
            is_intersect(&next_aabb, &static_q)
        };
        let dp = (safe_pos - d.aabb.position).as_vec2();
        a.dp -= dp;
        d.aabb.position = safe_pos;
        if collision_along_x {
            a.velocity.x = 0.0;
            a.dp.x = 0.0;
        }
        if collision_along_y {
            a.velocity.y = 0.0;
            a.dp.y = 0.0;
        }
        a.collision = match (collision_along_x, is_standing) {
            (true, false) => CollisionStatus::WallHug,
            (_, true) => CollisionStatus::Standing,
            (false, false) => CollisionStatus::FreeFall,
        };
    }
}
