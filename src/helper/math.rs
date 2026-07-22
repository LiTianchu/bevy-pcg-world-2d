use bevy::prelude::*;
/// Returns a vector of new IRect representing the area of rect_a that is not overlapped with rect_b.
pub fn except_rect(rect_a: &IRect, rect_b: &IRect) -> Vec<IRect> {
    // if rect_a and rect_b do not intersect, return rect_a as the only remaining area.
    if rect_a.intersect(*rect_b).is_empty() {
        return vec![*rect_a];
    }

    let intersection_min = rect_a.min.max(rect_b.min);
    let intersection_max = rect_a.max.min(rect_b.max);

    let mut result = Vec::with_capacity(4);

    // Left section
    if rect_a.min.x < intersection_min.x {
        result.push(IRect::from_corners(
            rect_a.min,
            IVec2::new(intersection_min.x, rect_a.max.y),
        ));
    }

    // Right section
    if intersection_max.x < rect_a.max.x {
        result.push(IRect::from_corners(
            IVec2::new(intersection_max.x, rect_a.min.y),
            rect_a.max,
        ));
    }

    // Bottom section, limited to the intersection's horizontal range
    if rect_a.min.y < intersection_min.y {
        result.push(IRect::from_corners(
            IVec2::new(intersection_min.x, rect_a.min.y),
            IVec2::new(intersection_max.x, intersection_min.y),
        ));
    }

    // Top section, limited to the intersection's horizontal range
    if intersection_max.y < rect_a.max.y {
        result.push(IRect::from_corners(
            IVec2::new(intersection_min.x, intersection_max.y),
            IVec2::new(intersection_max.x, rect_a.max.y),
        ));
    }

    result
}

pub trait IRectExcept {
    fn except(&self, other: &IRect) -> Vec<IRect>;
}

/// Implement the IRectExcept trait for IRect
impl IRectExcept for IRect {
    /// Returns a vector of new IRect representing the area of self that is not overlapped with other.
    fn except(&self, other: &IRect) -> Vec<IRect> {
        except_rect(self, other)
    }
}
