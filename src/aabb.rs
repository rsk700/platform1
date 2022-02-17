use bevy::prelude::*;

#[derive(Debug, Clone)]
pub struct IAabb {
    pub halfs: IVec2,
    pub position: IVec2,
}

impl IAabb {
    pub fn new(halfs: IVec2, position: IVec2) -> Self {
        debug_assert!(halfs.x > 0 && halfs.y > 0);
        Self { halfs, position }
    }

    #[inline]
    pub fn is_intersect(&self, other: &Self) -> bool {
        (self.position.x - other.position.x).abs() < self.halfs.x + other.halfs.x
            && (self.position.y - other.position.y).abs() < self.halfs.y + other.halfs.y
    }
}
