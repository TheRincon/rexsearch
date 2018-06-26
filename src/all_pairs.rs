use std::cmp::Ordering;

use abundance;
use align;

#[inline]
pub fn compare(x: hit, y: hit) -> i64 {
    let a = x.id.cmp(y.id);
    if a == Ordering::Equal {

    }
}

struct hit {
    id: i64,
    target: i64
}