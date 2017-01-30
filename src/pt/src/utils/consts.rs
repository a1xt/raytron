use math::{Coord};

pub const UP_VEC: [Coord; 3] = [0.0, 1.0, 0.0];
pub const FORWARD_VEC: [Coord; 3] = [0.0, 0.0, -1.0];
pub const RIGHT_VEC: [Coord; 3] = [1.0, 0.0, 0.0];

pub const RAY_SHIFT_DISTANCE: Coord = 0.1e-7;