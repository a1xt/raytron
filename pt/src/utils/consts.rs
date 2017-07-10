use math::Real;

pub const UP_VEC: [Real; 3] = [0.0, 1.0, 0.0];
pub const FORWARD_VEC: [Real; 3] = [0.0, 0.0, -1.0];
pub const RIGHT_VEC: [Real; 3] = [1.0, 0.0, 0.0];

pub const REAL_EPSILON: Real = ::std::f32::EPSILON as Real;
pub const POSITION_EPSILON: Real = ::std::f32::EPSILON as Real;
pub const TEXTURE_INTEGRAL_STEP: Real = 0.1e-2;
