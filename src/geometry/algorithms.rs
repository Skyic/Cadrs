use crate::geometry::{Point, Line, Circle, Arc};
use crate::math::Vector2;
use std::cmp::Ordering;

#[inline]
pub fn distance_point_to_line(point: Point, line: Line) -> f64 {
    let start = line.start;
    let end = line.end;
    let dx = end.x - start.x;
    let dy = end.y - start.y;
    
    let length_sq = dx * dx + dy * dy;
    if length_sq < 1e-10 {
        return point.distance_to(&start);
    }
    
    let t = ((point.x - start.x) * dx + (point.y - start.y) * dy) / length_sq;
    let t = t.clamp(0.0, 1.0);
    
    let closest = Point::new(
        start.x + t * dx,
        start.y + t * dy,
        start.z,
    );
    
    point.distance_to(&closest)
}

#[inline]
pub fn distance_point_to_circle(point: Point, circle: Circle) -> f64 {
    let dist = point.distance_to(&circle.center);
    (dist - circle.radius).abs()
}

#[inline]
pub fn distance_point_to_arc(point: Point, arc: Arc) -> f64 {
    let angle = arc.angle_from_center(&point);
    let normalized_angle = arc.normalize_angle(angle);
    
    let start = arc.normalize_angle(arc.start_angle);
    let end = arc.normalize_angle(arc.end_angle);
    
    let on_arc = if arc.is_counter_clockwise {
        if end >= start {
            normalized_angle >= start && normalized_angle <= end
        } else {
            normalized_angle >= start || normalized_angle <= end
        }
    } else {
        if end <= start {
            normalized_angle <= start && normalized_angle >= end
        } else {
            normalized_angle <= start || normalized_angle >= end
        }
    };
    
    let dist_to_center = point.distance_to(&arc.center);
    let radial_dist = (dist_to_center - arc.radius).abs();
    
    if on_arc {
        radial_dist
    } else {
        let to_start = point.distance_to(&arc.start_point());
        let to_end = point.distance_to(&arc.end_point());
        radial_dist.min(to_start).min(to_end)
    }
}

#[inline]
pub fn perpendicular_point_to_line(point: Point, line: Line) -> Point {
    line.closest_point(&point)
}

#[inline]
pub fn closest_points_on_lines(line1: Line, line2: Line) -> (Point, Point) {
    let p1 = line1.start.to_vector2();
    let p2 = line1.end.to_vector2();
    let p3 = line2.start.to_vector2();
    let p4 = line2.end.to_vector2();
    
    let d1 = p2 - p1;
    let d2 = p4 - p3;
    
    let r = p1 - p3;
    let a = d1.dot(&d1);
    let e = d2.dot(&d2);
    let f = d2.dot(&r);
    
    let (s, t) = if a <= 1e-10 && e <= 1e-10 {
        (0.0, 0.0)
    } else if a <= 1e-10 {
        (0.0, f / e)
    } else {
        let c = d1.dot(&r);
        let denom = a * e - d1.dot(&d2).powi(2);
        
        if denom.abs() < 1e-10 {
            (0.0, 0.0)
        } else {
            let b = d1.dot(&d2);
            let s = (b * f - c * e) / denom;
            let t = s * b - f;
            (s.clamp(0.0, 1.0), t.clamp(0.0, 1.0))
        }
    };
    
    (
        Point::new(p1.x + d1.x * s, p1.y + d1.y * s, 0.0),
        Point::new(p3.x + d2.x * t, p3.y + d2.y * t, 0.0),
    )
}

#[inline]
pub fn convex_hull(points: &[Point]) -> Vec<Point> {
    if points.len() <= 2 {
        return points.to_vec();
    }

    let mut sorted_points: Vec<_> = points.iter().collect();
    sorted_points.sort_by(|a, b| {
        let cmp = a.x.partial_cmp(&b.x).unwrap();
        if cmp != Ordering::Equal {
            cmp
        } else {
            a.y.partial_cmp(&b.y).unwrap()
        }
    });

    let cross = |o: &Point, a: &Point, b: &Point| -> f64 {
        (a.x - o.x) * (b.y - o.y) - (a.y - o.y) * (b.x - o.x)
    };

    let mut lower = Vec::new();
    for p in &sorted_points {
        while lower.len() >= 2 && cross(&lower[lower.len()-2], &lower[lower.len()-1], p) <= 0.0 {
            lower.pop();
        }
        lower.push(**p);
    }

    let mut upper = Vec::new();
    for p in sorted_points.iter().rev() {
        while upper.len() >= 2 && cross(&upper[upper.len()-2], &upper[upper.len()-1], p) <= 0.0 {
            upper.pop();
        }
        upper.push(**p);
    }

    lower.pop();
    upper.pop();
    lower.extend(upper);
    lower
}

#[inline]
pub fn angle_between_lines(line1: Line, line2: Line) -> f64 {
    let dir1 = line1.direction();
    let dir2 = line2.direction();
    dir1.angle_to(&dir2)
}

#[inline]
pub fn offset_line(line: Line, distance: f64, side: i8) -> Line {
    let dir = line.direction();
    let normal = Vector2::new(-dir.y, dir.x);
    
    let offset_x = normal.x * (distance * side as f64);
    let offset_y = normal.y * (distance * side as f64);
    
    Line::new(
        Point::new(line.start.x + offset_x, line.start.y + offset_y, line.start.z),
        Point::new(line.end.x + offset_x, line.end.y + offset_y, line.end.z),
    )
}

#[inline]
pub fn trim_line_at_point(line: Line, trim_point: Point, keep_start: bool) -> Line {
    if keep_start {
        Line::new(line.start, trim_point)
    } else {
        Line::new(trim_point, line.end)
    }
}

#[inline]
pub fn extend_line(line: Line, extension: f64, extend_start: bool, extend_end: bool) -> Line {
    let direction = line.direction();
    
    let new_start = if extend_start {
        Point::new(line.start.x - direction.x * extension, line.start.y - direction.y * extension, line.start.z)
    } else {
        line.start
    };
    
    let new_end = if extend_end {
        Point::new(line.end.x + direction.x * extension, line.end.y + direction.y * extension, line.end.z)
    } else {
        line.end
    };
    
    Line::new(new_start, new_end)
}

#[inline]
pub fn midpoint(p1: Point, p2: Point) -> Point {
    Point::new(
        (p1.x + p2.x) / 2.0,
        (p1.y + p2.y) / 2.0,
        (p1.z + p2.z) / 2.0,
    )
}

#[inline]
pub fn bisector(p1: Point, p2: Point, p3: Point) -> Line {
    let mid1 = midpoint(p1, p2);
    let mid2 = midpoint(p2, p3);
    Line::new(mid1, mid2)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_distance_point_to_line() {
        let line = Line::new(Point::origin(), Point::new(1.0, 0.0, 0.0));
        let point = Point::new(0.5, 1.0, 0.0);
        
        let dist = distance_point_to_line(point, line);
        assert!((dist - 1.0).abs() < 1e-10);
    }

    #[test]
    fn test_convex_hull() {
        let points = vec![
            Point::new(0.0, 0.0, 0.0),
            Point::new(1.0, 0.0, 0.0),
            Point::new(1.0, 1.0, 0.0),
            Point::new(0.0, 1.0, 0.0),
            Point::new(0.5, 0.5, 0.0),
        ];
        
        let hull = convex_hull(&points);
        assert_eq!(hull.len(), 4);
    }

    #[test]
    fn test_offset_line() {
        let line = Line::new(Point::origin(), Point::new(1.0, 0.0, 0.0));
        let offset = offset_line(line, 1.0, 1);
        
        assert!((offset.start.y - 1.0).abs() < 1e-10);
    }
    
    #[test]
    fn test_closest_points_on_lines_parallel() {
        let line1 = Line::new(Point::origin(), Point::new(1.0, 0.0, 0.0));
        let line2 = Line::new(Point::new(0.0, 1.0, 0.0), Point::new(1.0, 1.0, 0.0));
        
        let (p1, p2) = closest_points_on_lines(line1, line2);
        assert!((p1.y - 0.0).abs() < 1e-10);
        assert!((p2.y - 1.0).abs() < 1e-10);
    }
    
    #[test]
    fn test_angle_between_lines() {
        let line1 = Line::new(Point::origin(), Point::new(1.0, 0.0, 0.0));
        let line2 = Line::new(Point::origin(), Point::new(0.0, 1.0, 0.0));
        
        let angle = angle_between_lines(line1, line2);
        assert!((angle - std::f64::consts::FRAC_PI_2).abs() < 1e-10);
    }
    
    #[test]
    fn test_midpoint() {
        let p1 = Point::new(0.0, 0.0, 0.0);
        let p2 = Point::new(2.0, 4.0, 0.0);
        
        let mid = midpoint(p1, p2);
        assert!((mid.x - 1.0).abs() < 1e-10);
        assert!((mid.y - 2.0).abs() < 1e-10);
    }
    
    #[test]
    fn test_trim_line() {
        let line = Line::new(Point::origin(), Point::new(2.0, 0.0, 0.0));
        let trim_point = Point::new(1.0, 0.0, 0.0);
        
        let trimmed = trim_line_at_point(line, trim_point, true);
        assert!((trimmed.end.x - 1.0).abs() < 1e-10);
        
        let trimmed2 = trim_line_at_point(line, trim_point, false);
        assert!((trimmed2.start.x - 1.0).abs() < 1e-10);
    }
    
    #[test]
    fn test_extend_line() {
        let line = Line::new(Point::origin(), Point::new(1.0, 0.0, 0.0));
        
        let extended = extend_line(line, 0.5, true, true);
        assert!((extended.start.x - (-0.5)).abs() < 1e-10);
        assert!((extended.end.x - 1.5).abs() < 1e-10);
    }
    
    #[test]
    fn test_bisector() {
        let p1 = Point::new(0.0, 0.0, 0.0);
        let p2 = Point::new(2.0, 0.0, 0.0);
        let p3 = Point::new(1.0, 1.0, 0.0);
        
        let bis = bisector(p1, p2, p3);
        assert!((bis.start.x - 1.0).abs() < 1e-10);
        assert!((bis.start.y - 0.0).abs() < 1e-10);
    }
    
    #[test]
    fn test_convex_hull_triangle() {
        let points = vec![
            Point::new(0.0, 0.0, 0.0),
            Point::new(1.0, 0.0, 0.0),
            Point::new(0.5, 0.5, 0.0),
        ];
        
        let hull = convex_hull(&points);
        assert_eq!(hull.len(), 3);
    }
    
    #[test]
    fn test_distance_point_to_line_collinear() {
        let line = Line::new(Point::origin(), Point::new(1.0, 0.0, 0.0));
        let point = Point::new(0.5, 0.0, 0.0);
        
        let dist = distance_point_to_line(point, line);
        assert!(dist < 1e-10);
    }
    
    #[test]
    fn test_distance_point_to_line_outside_segment() {
        let line = Line::new(Point::origin(), Point::new(1.0, 0.0, 0.0));
        let point = Point::new(2.0, 1.0, 0.0);
        
        let dist = distance_point_to_line(point, line);
        let expected_dist = ((2.0_f64 - 1.0).powi(2) + (1.0_f64 - 0.0).powi(2)).sqrt();
        assert!((dist - expected_dist).abs() < 1e-10);
    }
    
    #[test]
    fn test_perpendicular_point() {
        let line = Line::new(Point::origin(), Point::new(1.0, 0.0, 0.0));
        let point = Point::new(0.5, 1.0, 0.0);
        
        let perp = perpendicular_point_to_line(point, line);
        assert!((perp.x - 0.5).abs() < 1e-10);
        assert!((perp.y - 0.0).abs() < 1e-10);
    }
}
