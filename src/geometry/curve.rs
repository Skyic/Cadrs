use super::point::Point;

pub trait Curve {
    fn point_at(&self, t: f64) -> Point;
    fn tangent_at(&self, t: f64) -> Point;
    fn bounding_box(&self) -> (Point, Point);
    fn parameter_range(&self) -> (f64, f64);
    fn length(&self, tolerance: f64) -> f64;
    fn is_closed(&self) -> bool;
    fn degree(&self) -> usize;
}

#[derive(Debug, Clone, PartialEq)]
pub enum CurveType {
    Line,
    Arc,
    Ellipse,
    BSpline,
    NURBS,
    Polyline,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_curve_trait_bounds() {
        let _ = CurveType::Line;
    }
}
