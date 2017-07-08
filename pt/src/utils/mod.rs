pub mod consts;

pub fn clamp<T: Copy + PartialOrd>(val: T, left_bound: T, right_bound: T) -> T {
    if val < left_bound {
        left_bound
    } else if val > right_bound {
        right_bound
    } else {
        val
    }
    
}