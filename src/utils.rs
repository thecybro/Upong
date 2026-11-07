
use crate::Rect;

#[allow(unused)]


pub fn in_range(value: f32, min: f32, max: f32) -> bool {
    /* This will break if max is less than min;
       So we check based on which one is less/more
       regardless of what was passed here
    */
    // (value >= min) && (value <= max)
    
    (value >= min.min(max)) && (value <= max.max(min))
}

pub fn ranges_overlap(min1: f32, max1: f32, min2: f32, max2: f32) -> bool {
    max1.max(min1) >= min2.min(max2)
    && 
    min1.min(max1) <= max2.max(min2)
}

pub fn rects_overlap(rect1: Rect, rect2: Rect) -> bool {
    ranges_overlap(
        rect1.x, rect1.x + rect1.width,
        rect2.x, rect2.x + rect2.width
    )
    &&
    ranges_overlap(
        rect1.y, rect1.y + rect1.height,
        rect2.y, rect2.y + rect2.height
    )
}
