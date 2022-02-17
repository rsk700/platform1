use bevy::prelude::*;

use crate::aabb::IAabb;

#[derive(Component)]
pub struct Static {
    pub aabb: IAabb,
}

#[derive(Component)]
pub struct Dynamic {
    pub aabb: IAabb,
}

#[derive(Debug, PartialEq)]
pub enum CollisionStatus {
    Standing,
    FreeFall,
    WallHug,
}

#[derive(Component)]
pub struct Actor {
    pub max_speed: f32,
    pub acceleration: Vec2,
    pub velocity: Vec2,
    // position delta, future position where actor want to move
    pub dp: Vec2,
    pub collision: CollisionStatus,
}
