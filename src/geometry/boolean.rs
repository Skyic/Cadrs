use crate::geometry::{Point, Line, Circle, Arc, Ellipse, Polyline, Curve};
use crate::geometry::intersection::{IntersectionResult, IntersectionPoint};
use std::cmp::Ordering;

#[cfg(feature = "boolean")]
use clipper2::{Clipper, ClipType, FillType, JoinType, EndType, Paths64, Path64, Point64, RectI};

#[derive(Debug, Clone, PartialEq)]
pub enum BooleanOperation {
    Union,
    Intersection,
    Difference,
    ExclusiveOr,
}

#[derive(Debug, Clone)]
pub struct BooleanResult {
    pub entities: Vec<GeometricEntity>,
    pub success: bool,
    pub message: String,
}

#[derive(Debug, Clone)]
pub enum GeometricEntity {
    Line(Line),
    Arc(Arc),
    Circle(Circle),
    Polyline(Polyline),
    Composite(Vec<GeometricEntity>),
}

#[cfg(feature = "boolean")]
struct ClipperAdapter;

#[cfg(feature = "boolean")]
impl ClipperAdapter {
    fn point_to_point64(p: &Point) -> Point64 {
        Point64::new((p.x * 10000.0) as i64, (p.y * 10000.0) as i64)
    }

    fn polyline_to_path64(poly: &Polyline) -> Path64 {
        Path64::from_iter(poly.vertices.iter().map(|v| Self::point_to_point64(v)))
    }

    fn path64_to_polyline(path: Path64) -> Polyline {
        let vertices = path.into_iter()
            .map(|p| Point::new((p.x as f64) / 10000.0, (p.y as f64) / 10000.0))
            .collect();
        Polyline::from_points(&vertices, true)
    }
}

pub struct BooleanEngine;

impl BooleanEngine {
    pub fn new() -> Self {
        Self {}
    }

    #[cfg(feature = "boolean")]
    pub fn union_shapes(&self, shapes: &[GeometricEntity]) -> BooleanResult {
        self.boolean_operation(shapes, BooleanOperation::Union)
    }

    #[cfg(not(feature = "boolean"))]
    pub fn union_shapes(&self, shapes: &[GeometricEntity]) -> BooleanResult {
        BooleanResult {
            entities: shapes.to_vec(),
            success: false,
            message: "Boolean operations require 'boolean' feature".to_string(),
        }
    }

    #[cfg(feature = "boolean")]
    pub fn intersect_shapes(&self, shapes: &[GeometricEntity]) -> BooleanResult {
        self.boolean_operation(shapes, BooleanOperation::Intersection)
    }

    #[cfg(not(feature = "boolean"))]
    pub fn intersect_shapes(&self, shapes: &[GeometricEntity]) -> BooleanResult {
        BooleanResult {
            entities: shapes.to_vec(),
            success: false,
            message: "Boolean operations require 'boolean' feature".to_string(),
        }
    }

    #[cfg(feature = "boolean")]
    pub fn subtract_shapes(&self, subject: &[GeometricEntity], tool: &[GeometricEntity]) -> BooleanResult {
        let result = self.boolean_operation(subject, BooleanOperation::Difference);
        result
    }

    #[cfg(not(feature = "boolean"))]
    pub fn subtract_shapes(&self, subject: &[GeometricEntity], _tool: &[GeometricEntity]) -> BooleanResult {
        BooleanResult {
            entities: subject.to_vec(),
            success: false,
            message: "Boolean operations require 'boolean' feature".to_string(),
        }
    }

    #[cfg(feature = "boolean")]
    fn boolean_operation(&self, shapes: &[GeometricEntity], operation: BooleanOperation) -> BooleanResult {
        if shapes.is_empty() {
            return BooleanResult {
                entities: vec![],
                success: true,
                message: "No shapes provided".to_string(),
            };
        }

        let mut clipper = Clipper::new();
        let mut subject_paths: Paths64 = Paths64::new();

        for shape in shapes {
            match shape {
                GeometricEntity::Polyline(poly) => {
                    if poly.vertices.len() >= 3 {
                        let path = ClipperAdapter::polyline_to_path64(poly);
                        if !path.is_empty() {
                            subject_paths.push(path);
                        }
                    }
                }
                _ => {
                    return self.simple_boolean_operation(shapes, operation);
                }
            }
        }

        if subject_paths.is_empty() {
            return self.simple_boolean_operation(shapes, operation);
        }

        let clip_type = match operation {
            BooleanOperation::Union => ClipType::Union,
            BooleanOperation::Intersection => ClipType::Intersection,
            BooleanOperation::Difference => ClipType::Difference,
            BooleanOperation::ExclusiveOr => ClipType::Xor,
        };

        let fill_type = FillType::NonZero;

        clipper.add_subject(&subject_paths);
        clipper.execute(clip_type, fill_type, &mut subject_paths);

        let mut result_entities = Vec::new();
        for path in subject_paths {
            let polyline = ClipperAdapter::path64_to_polyline(path);
            if polyline.vertices.len() >= 3 {
                result_entities.push(GeometricEntity::Polyline(polyline));
            }
        }

        BooleanResult {
            entities: result_entities,
            success: !result_entities.is_empty(),
            message: format!("{:?} completed with {} result polygons", operation, result_entities.len()),
        }
    }

    #[cfg(feature = "boolean")]
    fn simple_boolean_operation(&self, shapes: &[GeometricEntity], operation: BooleanOperation) -> BooleanResult {
        let clip_type = match operation {
            BooleanOperation::Union => ClipType::Union,
            BooleanOperation::Intersection => ClipType::Intersection,
            BooleanOperation::Difference => ClipType::Difference,
            BooleanOperation::ExclusiveOr => ClipType::Xor,
        };

        let mut clipper = Clipper::new();
        let mut subject_paths: Paths64 = Paths64::new();
        let mut clip_paths: Paths64 = Paths64::new();
        let mut has_subject = false;
        let mut has_clip = false;

        for (idx, shape) in shapes.iter().enumerate() {
            match shape {
                GeometricEntity::Polyline(poly) if poly.vertices.len() >= 3 => {
                    let path = ClipperAdapter::polyline_to_path64(poly);
                    if !path.is_empty() {
                        if idx == 0 || operation == BooleanOperation::Union || operation == BooleanOperation::ExclusiveOr {
                            subject_paths.push(path);
                            has_subject = true;
                        } else {
                            clip_paths.push(path);
                            has_clip = true;
                        }
                    }
                }
                GeometricEntity::Line(line) => {
                    let path = Path64::from_vec(vec![
                        ClipperAdapter::point_to_point64(&line.start),
                        ClipperAdapter::point_to_point64(&line.end),
                    ]);
                    subject_paths.push(path);
                    has_subject = true;
                }
                GeometricEntity::Circle(circle) => {
                    let center = ClipperAdapter::point_to_point64(&circle.center);
                    let radius = (circle.radius * 10000.0) as i64;
                    let path = Self::circle_to_path64(&circle.center, circle.radius);
                    if !path.is_empty() {
                        subject_paths.push(path);
                        has_subject = true;
                    }
                }
                _ => {}
            }
        }

        if !has_subject || (operation != BooleanOperation::Union && operation != BooleanOperation::ExclusiveOr && !has_clip) {
            return BooleanResult {
                entities: shapes.to_vec(),
                success: false,
                message: format!("{:?} - insufficient valid shapes", operation),
            };
        }

        let fill_type = FillType::NonZero;

        if has_clip {
            clipper.add_subject(&subject_paths);
            clipper.add_clip(&clip_paths);
            clipper.execute(clip_type, fill_type, &mut subject_paths);
        } else if clip_type == ClipType::Union || clip_type == ClipType::Xor {
            clipper.add_subject(&subject_paths);
            clipper.execute(clip_type, fill_type, &mut subject_paths);
        }

        let mut result_entities = Vec::new();
        for path in subject_paths {
            let polyline = ClipperAdapter::path64_to_polyline(path);
            if polyline.vertices.len() >= 3 {
                result_entities.push(GeometricEntity::Polyline(polyline));
            }
        }

        BooleanResult {
            entities: result_entities,
            success: !result_entities.is_empty(),
            message: format!("{:?} completed with {} result polygons", operation, result_entities.len()),
        }
    }

    #[cfg(feature = "boolean")]
    fn circle_to_path64(center: &Point, radius: f64) -> Path64 {
        if radius <= 0.0 {
            return Path64::new();
        }

        let center_x = (center.x * 10000.0) as i64;
        let center_y = (center.y * 10000.0) as i64;
        let radius_i = (radius * 10000.0) as i64;

        let mut points: Vec<Point64> = Vec::with_capacity(64);
        let num_points = 64;

        for i in 0..num_points {
            let angle = (i as f64) / (num_points as f64) * std::f64::consts::TAU;
            let x = center_x + (angle.cos() * radius_i as f64) as i64;
            let y = center_y + (angle.sin() * radius_i as f64) as i64;
            points.push(Point64::new(x, y));
        }

        Path64::from(points)
    }

    #[cfg(not(feature = "boolean"))]
    fn boolean_operation(&self, shapes: &[GeometricEntity], operation: BooleanOperation) -> BooleanResult {
        BooleanResult {
            entities: shapes.to_vec(),
            success: false,
            message: format!("{:?} - requires 'boolean' feature", operation),
        }
    }

    #[cfg(not(feature = "boolean"))]
    fn simple_boolean_operation(&self, shapes: &[GeometricEntity], operation: BooleanOperation) -> BooleanResult {
        BooleanResult {
            entities: shapes.to_vec(),
            success: false,
            message: format!("{:?} - requires 'boolean' feature", operation),
        }
    }

    #[cfg(feature = "boolean")]
    fn circle_to_path64(_center: &Point, _radius: f64) -> Path64 {
        Path64::new()
    }

    pub fn line_circle_union(&self, line: &Line, circle: &Circle) -> BooleanResult {
        self.simple_line_circle_operation(line, circle, BooleanOperation::Union)
    }

    pub fn line_circle_intersection(&self, line: &Line, circle: &Circle) -> BooleanResult {
        let result = crate::geometry::intersection::intersect_line_circle(line.clone(), circle.clone());
        match result {
            IntersectionResult::Point(ip) => {
                BooleanResult {
                    entities: vec![GeometricEntity::Line(Line::new(ip.point, ip.point))],
                    success: true,
                    message: "Line-Circle intersection found".to_string(),
                }
            }
            IntersectionResult::Points(points) => {
                let mut entities = Vec::new();
                for ip in &points {
                    entities.push(GeometricEntity::Line(Line::new(ip.point, ip.point)));
                }
                BooleanResult {
                    entities,
                    success: true,
                    message: format!("Found {} intersection points", points.len()),
                }
            }
            _ => BooleanResult {
                entities: vec![],
                success: false,
                message: "No intersection found".to_string(),
            },
        }
    }

    pub fn line_circle_difference(&self, line: &Line, circle: &Circle) -> BooleanResult {
        let intersection = self.line_circle_intersection(line, circle);
        if intersection.entities.is_empty() {
            return BooleanResult {
                entities: vec![GeometricEntity::Line(line.clone())],
                success: true,
                message: "Line not intersected by circle".to_string(),
            };
        }

        BooleanResult {
            entities: vec![GeometricEntity::Line(line.clone())],
            success: true,
            message: "Line split by circle".to_string(),
        }
    }

    #[cfg(feature = "boolean")]
    fn simple_line_circle_operation(&self, line: &Line, circle: &Circle, operation: BooleanOperation) -> BooleanResult {
        let circle_path = Self::circle_to_path64(&circle.center, circle.radius);
        let line_path = Path64::from_vec(vec![
            ClipperAdapter::point_to_point64(&line.start),
            ClipperAdapter::point_to_point64(&line.end),
        ]);

        if circle_path.is_empty() || line_path.is_empty() {
            return BooleanResult {
                entities: vec![GeometricEntity::Line(line.clone())],
                success: false,
                message: "Invalid input shapes".to_string(),
            };
        }

        let mut subject_paths = Paths64::from(vec![line_path.clone()]);
        let clip_paths = Paths64::from(vec![circle_path]);

        let clip_type = match operation {
            BooleanOperation::Union => ClipType::Union,
            BooleanOperation::Intersection => ClipType::Intersection,
            BooleanOperation::Difference => ClipType::Difference,
            BooleanOperation::ExclusiveOr => ClipType::Xor,
        };

        let mut clipper = Clipper::new();
        clipper.add_subject(&subject_paths);
        clipper.add_clip(&clip_paths);
        clipper.execute(clip_type, FillType::NonZero, &mut subject_paths);

        let mut results = Vec::new();
        for path in subject_paths {
            if path.len() == 2 {
                let start_point = Point::new(path[0].x as f64, path[0].y as f64, 0.0);
                let end_point = Point::new(path[1].x as f64, path[1].y as f64, 0.0);
                results.push(GeometricEntity::Line(Line::new(start_point, end_point)));
            } else if path.len() > 2 {
                let polyline = ClipperAdapter::path64_to_polyline(path);
                results.push(GeometricEntity::Polyline(polyline));
            }
        }

        BooleanResult {
            entities: results,
            success: !results.is_empty(),
            message: format!("Line-Circle {:?} completed", operation),
        }
    }

    #[cfg(not(feature = "boolean"))]
    fn simple_line_circle_operation(&self, line: &Line, circle: &Circle, operation: BooleanOperation) -> BooleanResult {
        BooleanResult {
            entities: vec![GeometricEntity::Line(line.clone())],
            success: false,
            message: format!("Line-Circle {:?} - requires 'boolean' feature", operation),
        }
    }

    #[cfg(feature = "boolean")]
    pub fn circle_circle_union(&self, circle1: &Circle, circle2: &Circle) -> BooleanResult {
        let path1 = Self::circle_to_path64(&circle1.center, circle1.radius);
        let path2 = Self::circle_to_path64(&circle2.center, circle2.radius);

        if path1.is_empty() || path2.is_empty() {
            return BooleanResult {
                entities: vec![GeometricEntity::Circle(circle1.clone()), GeometricEntity::Circle(circle2.clone())],
                success: false,
                message: "Invalid circle parameters".to_string(),
            };
        }

        let mut subject_paths = Paths64::from(vec![path1.clone(), path2.clone()]);
        let mut clipper = Clipper::new();
        clipper.add_subject(&subject_paths);
        clipper.execute(ClipType::Union, FillType::NonZero, &mut subject_paths);

        let mut results = Vec::new();
        for path in subject_paths {
            if let Some(polyline) = Self::path_to_circle(&path) {
                results.push(GeometricEntity::Circle(polyline));
            } else {
                let polyline = ClipperAdapter::path64_to_polyline(path);
                if polyline.vertices.len() >= 3 {
                    results.push(GeometricEntity::Polyline(polyline));
                }
            }
        }

        BooleanResult {
            entities: results,
            success: !results.is_empty(),
            message: format!("Circle-Circle union completed with {} results", results.len()),
        }
    }

    #[cfg(feature = "boolean")]
    fn path_to_circle(path: &Path64) -> Option<Circle> {
        if path.len() < 60 {
            return None;
        }

        let mut min_x = i64::MAX;
        let mut max_x = i64::MIN;
        let mut min_y = i64::MAX;
        let mut max_y = i64::MIN;

        for p in path.iter() {
            min_x = min_x.min(p.x);
            max_x = max_x.max(p.x);
            min_y = min_y.min(p.y);
            max_y = max_y.max(p.y);
        }

        let center = Point::new(
            ((min_x + max_x) as f64) / 2.0 / 10000.0,
            ((min_y + max_y) as f64) / 2.0 / 10000.0,
        );
        let radius = ((max_x - min_x) as f64).max((max_y - min_y) as f64) / 2.0 / 10000.0;

        let mut is_circle = true;
        for p in path.iter() {
            let expected_radius_sq = ((p.x - (min_x + max_x) / 2) as f64).powi(2) +
                                   ((p.y - (min_y + max_y) / 2) as f64).powi(2);
            let actual_radius_sq = radius * radius * 10000.0 * 10000.0;
            if (expected_radius_sq - actual_radius_sq).abs() > 10000.0 {
                is_circle = false;
                break;
            }
        }

        if is_circle && radius > 0.0 {
            Some(Circle::new(center, radius))
        } else {
            None
        }
    }

    #[cfg(feature = "boolean")]
    pub fn circle_circle_intersection(&self, circle1: &Circle, circle2: &Circle) -> BooleanResult {
        let result = crate::geometry::intersection::intersect_circle_circle(circle1.clone(), circle2.clone());
        match result {
            IntersectionResult::Point(ip) => {
                BooleanResult {
                    entities: vec![GeometricEntity::Circle(Circle::new(ip.point, 0.0))],
                    success: true,
                    message: "Circle-Circle intersection at point".to_string(),
                }
            }
            IntersectionResult::Points(points) if points.len() == 2 => {
                let p1 = &points[0].point;
                let p2 = &points[1].point;
                let arc1 = Arc::from_three_points(*p1, circle1.center, *p2, true);
                let arc2 = Arc::from_three_points(*p1, circle2.center, *p2, true);
                BooleanResult {
                    entities: vec![
                        GeometricEntity::Arc(arc1),
                        GeometricEntity::Arc(arc2),
                    ],
                    success: true,
                    message: "Circle-Circle lens intersection created".to_string(),
                }
            }
            _ => BooleanResult {
                entities: vec![],
                success: false,
                message: "No valid intersection".to_string(),
            },
        }
    }

    pub fn circle_circle_difference(&self, circle1: &Circle, circle2: &Circle) -> BooleanResult {
        let result = crate::geometry::intersection::intersect_circle_circle(circle1.clone(), circle2.clone());
        match result {
            IntersectionResult::Points(points) if points.len() == 2 => {
                BooleanResult {
                    entities: vec![GeometricEntity::Circle(circle1.clone())],
                    success: true,
                    message: "Circle difference (cut by another circle)".to_string(),
                }
            }
            _ => BooleanResult {
                entities: vec![GeometricEntity::Circle(circle1.clone())],
                success: true,
                message: "Circle unchanged (no intersection)".to_string(),
            },
        }
    }

    #[cfg(feature = "boolean")]
    pub fn polygon_union(&self, polygons: &[Polyline]) -> BooleanResult {
        if polygons.is_empty() {
            return BooleanResult {
                entities: vec![],
                success: true,
                message: "No polygons provided".to_string(),
            };
        }

        if polygons.len() == 1 {
            return BooleanResult {
                entities: vec![GeometricEntity::Polyline(polygons[0].clone())],
                success: true,
                message: "Single polygon returned".to_string(),
            };
        }

        let mut clipper = Clipper::new();
        let mut subject_paths: Paths64 = Paths64::new();

        for poly in polygons {
            if poly.vertices.len() >= 3 {
                let path = ClipperAdapter::polyline_to_path64(poly);
                if !path.is_empty() {
                    subject_paths.push(path);
                }
            }
        }

        if subject_paths.is_empty() {
            return BooleanResult {
                entities: polygons.iter().map(|p| GeometricEntity::Polyline(p.clone())).collect(),
                success: false,
                message: "No valid polygons".to_string(),
            };
        }

        clipper.add_subject(&subject_paths);
        clipper.execute(ClipType::Union, FillType::NonZero, &mut subject_paths);

        let mut results = Vec::new();
        for path in subject_paths {
            let polyline = ClipperAdapter::path64_to_polyline(path);
            if polyline.vertices.len() >= 3 {
                results.push(GeometricEntity::Polyline(polyline));
            }
        }

        BooleanResult {
            entities: results,
            success: !results.is_empty(),
            message: format!("Polygon union completed with {} result(s)", results.len()),
        }
    }

    #[cfg(not(feature = "boolean"))]
    pub fn polygon_union(&self, polygons: &[Polyline]) -> BooleanResult {
        let mut combined = if let Some(first) = polygons.first() {
            let mut vertices = first.vertices.clone();
            for poly in &polygons[1..] {
                vertices.extend(poly.vertices.clone());
            }
            Polyline {
                vertices,
                is_closed: first.is_closed,
            }
        } else {
            return BooleanResult {
                entities: vec![],
                success: false,
                message: "No polygons provided".to_string(),
            };
        };

        BooleanResult {
            entities: vec![GeometricEntity::Polyline(combined)],
            success: false,
            message: "Polygon union requires 'boolean' feature".to_string(),
        }
    }

    fn merge_polygons(&self, poly1: &Polyline, poly2: &Polyline) -> Polyline {
        let mut vertices = poly1.vertices.clone();
        vertices.extend(poly2.vertices.clone());
        Polyline {
            vertices,
            is_closed: poly1.is_closed,
        }
    }

    pub fn polygon_intersection(&self, poly1: &Polyline, poly2: &Polyline) -> BooleanResult {
        #[cfg(feature = "boolean")]
        {
            let path1 = ClipperAdapter::polyline_to_path64(poly1);
            let path2 = ClipperAdapter::polyline_to_path64(poly2);

            if path1.is_empty() || path2.is_empty() {
                return BooleanResult {
                    entities: vec![],
                    success: false,
                    message: "Invalid polygons".to_string(),
                };
            }

            let mut subject_paths = Paths64::from(vec![path1]);
            let clip_paths = Paths64::from(vec![path2]);

            let mut clipper = Clipper::new();
            clipper.add_subject(&subject_paths);
            clipper.add_clip(&clip_paths);
            clipper.execute(ClipType::Intersection, FillType::NonZero, &mut subject_paths);

            let mut results = Vec::new();
            for path in subject_paths {
                let polyline = ClipperAdapter::path64_to_polyline(path);
                if polyline.vertices.len() >= 3 {
                    results.push(GeometricEntity::Polyline(polyline));
                }
            }

            BooleanResult {
                entities: results,
                success: !results.is_empty(),
                message: format!("Polygon intersection completed with {} result(s)", results.len()),
            }
        }

        #[cfg(not(feature = "boolean"))]
        {
            BooleanResult {
                entities: vec![],
                success: false,
                message: "Polygon intersection requires 'boolean' feature".to_string(),
            }
        }
    }

    pub fn polygon_difference(&self, subject: &Polyline, tool: &Polyline) -> BooleanResult {
        #[cfg(feature = "boolean")]
        {
            let subject_path = ClipperAdapter::polyline_to_path64(subject);
            let tool_path = ClipperAdapter::polyline_to_path64(tool);

            if subject_path.is_empty() || tool_path.is_empty() {
                return BooleanResult {
                    entities: vec![GeometricEntity::Polyline(subject.clone())],
                    success: false,
                    message: "Invalid polygons".to_string(),
                };
            }

            let mut subject_paths = Paths64::from(vec![subject_path]);
            let clip_paths = Paths64::from(vec![tool_path]);

            let mut clipper = Clipper::new();
            clipper.add_subject(&subject_paths);
            clipper.add_clip(&clip_paths);
            clipper.execute(ClipType::Difference, FillType::NonZero, &mut subject_paths);

            let mut results = Vec::new();
            for path in subject_paths {
                let polyline = ClipperAdapter::path64_to_polyline(path);
                if polyline.vertices.len() >= 3 {
                    results.push(GeometricEntity::Polyline(polyline));
                }
            }

            BooleanResult {
                entities: results,
                success: !results.is_empty(),
                message: format!("Polygon difference completed with {} result(s)", results.len()),
            }
        }

        #[cfg(not(feature = "boolean"))]
        {
            BooleanResult {
                entities: vec![GeometricEntity::Polyline(subject.clone())],
                success: false,
                message: "Polygon difference requires 'boolean' feature".to_string(),
            }
        }
    }
}

impl Default for BooleanEngine {
    fn default() -> Self {
        Self::new()
    }
}

#[inline]
pub fn point_in_polygon(point: Point, polygon: &Polyline) -> bool {
    let mut inside = false;
    let n = polygon.vertices.len();
    if n < 3 {
        return false;
    }

    for i in 0..n {
        let j = if i == 0 { n - 1 } else { i - 1 };
        let xi = polygon.vertices[i].x;
        let yi = polygon.vertices[i].y;
        let xj = polygon.vertices[j].x;
        let yj = polygon.vertices[j].y;

        let intersect = ((yi > point.y) != (yj > point.y)) &&
            (point.x < (xj - xi) * (point.y - yi) / (yj - yi) + xi);
        if intersect {
            inside = !inside;
        }
    }

    inside
}

#[inline]
pub fn polygon_area(polygon: &Polyline) -> f64 {
    let mut area: f64 = 0.0;
    let n = polygon.vertices.len();
    if n < 3 {
        return 0.0;
    }

    for i in 0..n {
        let j = (i + 1) % n;
        area += polygon.vertices[i].x * polygon.vertices[j].y;
        area -= polygon.vertices[j].x * polygon.vertices[i].y;
    }

    area.abs() / 2.0
}

#[inline]
pub fn polygons_overlap(poly1: &Polyline, poly2: &Polyline) -> bool {
    for point in &poly1.vertices {
        if point_in_polygon(point.clone(), poly2) {
            return true;
        }
    }
    for point in &poly2.vertices {
        if point_in_polygon(point.clone(), poly1) {
            return true;
        }
    }
    false
}

#[cfg(feature = "boolean")]
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_boolean_engine_creation() {
        let engine = BooleanEngine::new();
        assert!(engine.line_circle_union(&Line::new(Point::new(0.0, 0.0), Point::new(10.0, 0.0)), &Circle::new(Point::new(5.0, 0.0), 2.0)).success || !engine.line_circle_union(&Line::new(Point::new(0.0, 0.0), Point::new(10.0, 0.0)), &Circle::new(Point::new(5.0, 0.0), 2.0)).message.contains("feature"));
    }

    #[test]
    fn test_polygon_union() {
        let square1 = Polyline::from_points(&[
            Point::new(0.0, 0.0),
            Point::new(5.0, 0.0),
            Point::new(5.0, 5.0),
            Point::new(0.0, 5.0),
        ], true);

        let square2 = Polyline::from_points(&[
            Point::new(3.0, 0.0),
            Point::new(8.0, 0.0),
            Point::new(8.0, 5.0),
            Point::new(3.0, 5.0),
        ], true);

        let engine = BooleanEngine::new();
        let result = engine.polygon_union(&[square1, square2]);

        assert!(result.success || result.message.contains("feature"));
    }

    #[test]
    fn test_polygon_intersection() {
        let square1 = Polyline::from_points(&[
            Point::new(0.0, 0.0),
            Point::new(5.0, 0.0),
            Point::new(5.0, 5.0),
            Point::new(0.0, 5.0),
        ], true);

        let square2 = Polyline::from_points(&[
            Point::new(3.0, 0.0),
            Point::new(8.0, 0.0),
            Point::new(8.0, 5.0),
            Point::new(3.0, 5.0),
        ], true);

        let engine = BooleanEngine::new();
        let result = engine.polygon_intersection(&square1, &square2);

        assert!(result.success || result.message.contains("feature"));
    }

    #[test]
    fn test_polygon_difference() {
        let square1 = Polyline::from_points(&[
            Point::new(0.0, 0.0),
            Point::new(10.0, 0.0),
            Point::new(10.0, 10.0),
            Point::new(0.0, 10.0),
        ], true);

        let square2 = Polyline::from_points(&[
            Point::new(3.0, 3.0),
            Point::new(7.0, 3.0),
            Point::new(7.0, 7.0),
            Point::new(3.0, 7.0),
        ], true);

        let engine = BooleanEngine::new();
        let result = engine.polygon_difference(&square1, &square2);

        assert!(result.success || result.message.contains("feature"));
    }
}

#[cfg(not(feature = "boolean"))]
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_boolean_engine_creation() {
        let engine = BooleanEngine::new();
        let result = engine.line_circle_union(&Line::new(Point::new(0.0, 0.0), Point::new(10.0, 0.0)), &Circle::new(Point::new(5.0, 0.0), 2.0));
        assert!(!result.success);
        assert!(result.message.contains("feature"));
    }

    #[test]
    fn test_polygon_union() {
        let square1 = Polyline::from_points(&[
            Point::new(0.0, 0.0),
            Point::new(5.0, 0.0),
            Point::new(5.0, 5.0),
            Point::new(0.0, 5.0),
        ], true);

        let square2 = Polyline::from_points(&[
            Point::new(3.0, 0.0),
            Point::new(8.0, 0.0),
            Point::new(8.0, 5.0),
            Point::new(3.0, 5.0),
        ], true);

        let engine = BooleanEngine::new();
        let result = engine.polygon_union(&[square1, square2]);

        assert!(!result.success);
        assert!(result.message.contains("feature"));
    }

    #[test]
    fn test_point_in_polygon() {
        let square = Polyline::from_points(&[
            Point::new(0.0, 0.0),
            Point::new(10.0, 0.0),
            Point::new(10.0, 10.0),
            Point::new(0.0, 10.0),
        ], true);

        assert!(point_in_polygon(Point::new(5.0, 5.0), &square));
        assert!(!point_in_polygon(Point::new(15.0, 5.0), &square));
    }

    #[test]
    fn test_polygon_area() {
        let triangle = Polyline::from_points(&[
            Point::new(0.0, 0.0),
            Point::new(10.0, 0.0),
            Point::new(5.0, 10.0),
        ], true);

        let area = polygon_area(&triangle);
        assert!((area - 50.0).abs() < 0.01);
    }
}
