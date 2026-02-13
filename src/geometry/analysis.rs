use crate::geometry::{Point, Vector2, Line, Circle, Arc, Ellipse, BSpline, NURBS, Curve};
use serde::{Serialize, Deserialize};
use std::fmt;

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct CurvatureInfo {
    pub point: Point,
    pub curvature: f64,
    pub radius: f64,
    pub center: Point,
    pub normal: Vector2,
}

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct FrenetFrame {
    pub point: Point,
    pub tangent: Vector2,
    pub normal: Vector2,
    pub binormal: Vector2,
}

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct CurveDerivatives {
    pub point: Point,
    pub first_derivative: Vector2,
    pub second_derivative: Vector2,
    pub third_derivative: Vector2,
}

pub trait CurveAnalyzer {
    fn curvature_at(&self, parameter: f64) -> f64;
    fn curvature_at_point(&self, point: Point) -> f64;
    fn radius_of_curvature(&self, parameter: f64) -> f64;
    fn tangent_at(&self, parameter: f64) -> Vector2;
    fn normal_at(&self, parameter: f64) -> Vector2;
    fn frenet_frame_at(&self, parameter: f64) -> FrenetFrame;
    fn derivatives_at(&self, parameter: f64) -> CurveDerivatives;
    fn arc_length(&self, tolerance: f64) -> f64;
    fn parameter_at_arc_length(&self, length: f64) -> f64;
    fn is_g1_continuous(&self, other: &dyn Curve) -> bool;
    fn is_g2_continuous(&self, other: &dyn Curve) -> bool;
}

pub struct LineAnalyzer;

impl LineAnalyzer {
    pub fn analyze(line: &Line) -> LineAnalysis {
        let length = line.length();
        let midpoint = line.midpoint();
        let direction = line.direction();
        let normal = Vector2::new(-direction.y, direction.x);
        
        LineAnalysis {
            line: line.clone(),
            length,
            midpoint,
            direction,
            normal,
            curvature: 0.0,
            radius_of_curvature: f64::INFINITY,
            is_linear: true,
            is_horizontal: line.is_horizontal(),
            is_vertical: line.is_vertical(),
            bounding_box: (line.start, line.end),
        }
    }
}

#[derive(Debug, Clone)]
pub struct LineAnalysis {
    pub line: Line,
    pub length: f64,
    pub midpoint: Point,
    pub direction: Vector2,
    pub normal: Vector2,
    pub curvature: f64,
    pub radius_of_curvature: f64,
    pub is_linear: bool,
    pub is_horizontal: bool,
    pub is_vertical: bool,
    pub bounding_box: (Point, Point),
}

pub struct ArcAnalyzer;

impl ArcAnalyzer {
    pub fn analyze(arc: &Arc) -> ArcAnalysis {
        let length = arc.radius * (arc.end_angle - arc.start_angle).abs();
        let midpoint_angle = (arc.start_angle + arc.end_angle) / 2.0;
        let midpoint = Point::new(
            arc.center.x + arc.radius * midpoint_angle.cos(),
            arc.center.y + arc.radius * midpoint_angle.sin(),
            0.0,
        );
        
        let curvature = 1.0 / arc.radius;
        let radius_of_curvature = arc.radius;
        
        let tangent = Vector2::new(-midpoint_angle.sin(), midpoint_angle.cos());
        let normal = Vector2::new(midpoint_angle.cos(), midpoint_angle.sin());
        
        let start_point = Point::new(
            arc.center.x + arc.radius * arc.start_angle.cos(),
            arc.center.y + arc.radius * arc.start_angle.sin(),
            0.0,
        );
        let end_point = Point::new(
            arc.center.x + arc.radius * arc.end_angle.cos(),
            arc.center.y + arc.radius * arc.end_angle.sin(),
            0.0,
        );
        
        let min_x = arc.center.x - arc.radius;
        let max_x = arc.center.x + arc.radius;
        let min_y = arc.center.y - arc.radius;
        let max_y = arc.center.y + arc.radius;
        
        ArcAnalysis {
            arc: arc.clone(),
            length,
            midpoint,
            direction: tangent,
            normal,
            curvature,
            radius_of_curvature,
            center: arc.center,
            diameter: arc.radius * 2.0,
            sweep_angle: (arc.end_angle - arc.start_angle).abs(),
            start_point,
            end_point,
            bounding_box: (
                Point::new(min_x, min_y, 0.0),
                Point::new(max_x, max_y, 0.0),
            ),
            is_clockwise: !arc.is_counter_clockwise,
        }
    }
}

#[derive(Debug, Clone)]
pub struct ArcAnalysis {
    pub arc: Arc,
    pub length: f64,
    pub midpoint: Point,
    pub direction: Vector2,
    pub normal: Vector2,
    pub curvature: f64,
    pub radius_of_curvature: f64,
    pub center: Point,
    pub diameter: f64,
    pub sweep_angle: f64,
    pub start_point: Point,
    pub end_point: Point,
    pub bounding_box: (Point, Point),
    pub is_clockwise: bool,
}

pub struct CircleAnalyzer;

impl CircleAnalyzer {
    pub fn analyze(circle: &Circle) -> CircleAnalysis {
        let circumference = 2.0 * std::f64::consts::PI * circle.radius;
        let area = std::f64::consts::PI * circle.radius * circle.radius;
        let curvature = 1.0 / circle.radius;
        
        let min_x = circle.center.x - circle.radius;
        let max_x = circle.center.x + circle.radius;
        let min_y = circle.center.y - circle.radius;
        let max_y = circle.center.y + circle.radius;
        
        let point1 = Point::new(circle.center.x + circle.radius, circle.center.y, 0.0);
        let point2 = Point::new(circle.center.x, circle.center.y + circle.radius, 0.0);
        let point3 = Point::new(circle.center.x - circle.radius, circle.center.y, 0.0);
        let point4 = Point::new(circle.center.x, circle.center.y - circle.radius, 0.0);
        
        CircleAnalysis {
            circle: circle.clone(),
            circumference,
            area,
            curvature,
            radius_of_curvature: circle.radius,
            diameter: circle.radius * 2.0,
            circumference_points: vec![point1, point2, point3, point4],
            bounding_box: (
                Point::new(min_x, min_y, 0.0),
                Point::new(max_x, max_y, 0.0),
            ),
        }
    }
}

#[derive(Debug, Clone)]
pub struct CircleAnalysis {
    pub circle: Circle,
    pub circumference: f64,
    pub area: f64,
    pub curvature: f64,
    pub radius_of_curvature: f64,
    pub diameter: f64,
    pub circumference_points: Vec<Point>,
    pub bounding_box: (Point, Point),
}

pub struct BSplineAnalyzer;

impl BSplineAnalyzer {
    pub fn analyze(spline: &BSpline) -> BSplineAnalysis {
        let length = spline.length(0.001);
        let degree = spline.degree;
        let num_control_points = spline.control_points.len();
        let num_knots = spline.knots.len();
        
        let mut max_curvature = 0.0;
        let mut min_curvature = f64::INFINITY;
        let mut total_curvature = 0.0;
        let mut curvature_samples = 0;
        
        for i in 0..100 {
            let t = i as f64 / 99.0;
            let curvature = spline.curvature_at(t);
            max_curvature = max_curvature.max(curvature);
            min_curvature = min_curvature.min(curvature);
            total_curvature += curvature;
            curvature_samples += 1;
        }
        
        let avg_curvature = total_curvature / curvature_samples as f64;
        
        BSplineAnalysis {
            spline: spline.clone(),
            length,
            degree,
            num_control_points,
            num_knots,
            max_curvature,
            min_curvature,
            avg_curvature,
            is_closed: spline.is_closed(),
            is_polynomial: degree <= 1,
        }
    }
}

#[derive(Debug, Clone)]
pub struct BSplineAnalysis {
    pub spline: BSpline,
    pub length: f64,
    pub degree: usize,
    pub num_control_points: usize,
    pub num_knots: usize,
    pub max_curvature: f64,
    pub min_curvature: f64,
    pub avg_curvature: f64,
    pub is_closed: bool,
    pub is_polynomial: bool,
}

impl CurveAnalyzer for Line {
    fn curvature_at(&self, _parameter: f64) -> f64 {
        0.0
    }
    
    fn curvature_at_point(&self, _point: Point) -> f64 {
        0.0
    }
    
    fn radius_of_curvature(&self, _parameter: f64) -> f64 {
        f64::INFINITY
    }
    
    fn tangent_at(&self, parameter: f64) -> Vector2 {
        self.direction()
    }
    
    fn normal_at(&self, parameter: f64) -> Vector2 {
        let dir = self.direction();
        Vector2::new(-dir.y, dir.x)
    }
    
    fn frenet_frame_at(&self, _parameter: f64) -> FrenetFrame {
        let point = self.point_at_parameter(_parameter);
        let tangent = self.direction();
        let normal = Vector2::new(-tangent.y, tangent.x);
        let binormal = Vector2::new(0.0, 0.0);
        
        FrenetFrame { point, tangent, normal, binormal }
    }
    
    fn derivatives_at(&self, _parameter: f64) -> CurveDerivatives {
        let point = self.point_at_parameter(_parameter);
        let first = self.direction();
        let second = Vector2::new(0.0, 0.0);
        let third = Vector2::new(0.0, 0.0);
        
        CurveDerivatives { point, first_derivative: first, second_derivative: second, third_derivative: third }
    }
    
    fn arc_length(&self, _tolerance: f64) -> f64 {
        self.length()
    }
    
    fn parameter_at_arc_length(&self, length: f64) -> f64 {
        (length / self.length()).clamp(0.0, 1.0)
    }
    
    fn is_g1_continuous(&self, other: &dyn Curve) -> bool {
        match other {
            Curve::Line(other_line) => {
                let end_dir = self.direction();
                let start_dir = other_line.direction();
                (end_dir - start_dir).magnitude() < 1e-6
            }
            _ => false,
        }
    }
    
    fn is_g2_continuous(&self, other: &dyn Curve) -> bool {
        self.is_g1_continuous(other)
    }
}

impl CurveAnalyzer for Circle {
    fn curvature_at(&self, _parameter: f64) -> f64 {
        1.0 / self.radius
    }
    
    fn curvature_at_point(&self, _point: Point) -> f64 {
        1.0 / self.radius
    }
    
    fn radius_of_curvature(&self, _parameter: f64) -> f64 {
        self.radius
    }
    
    fn tangent_at(&self, parameter: f64) -> Vector2 {
        let angle = parameter * 2.0 * std::f64::consts::PI;
        Vector2::new(-angle.sin(), angle.cos())
    }
    
    fn normal_at(&self, parameter: f64) -> Vector2 {
        let angle = parameter * 2.0 * std::f64::consts::PI;
        Vector2::new(angle.cos(), angle.sin())
    }
    
    fn frenet_frame_at(&self, parameter: f64) -> FrenetFrame {
        let angle = parameter * 2.0 * std::f64::consts::PI;
        let point = Point::new(
            self.center.x + self.radius * angle.cos(),
            self.center.y + self.radius * angle.sin(),
            0.0,
        );
        let tangent = Vector2::new(-angle.sin(), angle.cos());
        let normal = Vector2::new(angle.cos(), angle.sin());
        let binormal = Vector2::new(0.0, 0.0);
        
        FrenetFrame { point, tangent, normal, binormal }
    }
    
    fn derivatives_at(&self, parameter: f64) -> CurveDerivatives {
        let angle = parameter * 2.0 * std::f64::consts::PI;
        let point = Point::new(
            self.center.x + self.radius * angle.cos(),
            self.center.y + self.radius * angle.sin(),
            0.0,
        );
        let first = Vector2::new(
            -self.radius * 2.0 * std::f64::consts::PI * angle.sin(),
            self.radius * 2.0 * std::f64::consts::PI * angle.cos(),
        );
        let second = Vector2::new(
            -self.radius * (2.0 * std::f64::consts::PI).powi(2) * angle.cos(),
            -self.radius * (2.0 * std::f64::consts::PI).powi(2) * angle.sin(),
        );
        let third = Vector2::new(
            self.radius * (2.0 * std::f64::consts::PI).powi(3) * angle.sin(),
            -self.radius * (2.0 * std::f64::consts::PI).powi(3) * angle.cos(),
        );
        
        CurveDerivatives { point, first_derivative: first, second_derivative: second, third_derivative: third }
    }
    
    fn arc_length(&self, _tolerance: f64) -> f64 {
        2.0 * std::f64::consts::PI * self.radius
    }
    
    fn parameter_at_arc_length(&self, length: f64) -> f64 {
        (length / (2.0 * std::f64::consts::PI * self.radius)).clamp(0.0, 1.0)
    }
    
    fn is_g1_continuous(&self, _other: &dyn Curve) -> bool {
        true
    }
    
    fn is_g2_continuous(&self, _other: &dyn Curve) -> bool {
        true
    }
}

impl CurveAnalyzer for Arc {
    fn curvature_at(&self, _parameter: f64) -> f64 {
        1.0 / self.radius
    }
    
    fn curvature_at_point(&self, _point: Point) -> f64 {
        1.0 / self.radius
    }
    
    fn radius_of_curvature(&self, _parameter: f64) -> f64 {
        self.radius
    }
    
    fn tangent_at(&self, parameter: f64) -> Vector2 {
        let angle = self.start_angle + parameter * (self.end_angle - self.start_angle);
        Vector2::new(-angle.sin(), angle.cos())
    }
    
    fn normal_at(&self, parameter: f64) -> Vector2 {
        let angle = self.start_angle + parameter * (self.end_angle - self.start_angle);
        Vector2::new(angle.cos(), angle.sin())
    }
    
    fn frenet_frame_at(&self, parameter: f64) -> FrenetFrame {
        let angle = self.start_angle + parameter * (self.end_angle - self.start_angle);
        let point = Point::new(
            self.center.x + self.radius * angle.cos(),
            self.center.y + self.radius * angle.sin(),
            0.0,
        );
        let tangent = Vector2::new(-angle.sin(), angle.cos());
        let normal = Vector2::new(angle.cos(), angle.sin());
        let binormal = Vector2::new(0.0, 0.0);
        
        FrenetFrame { point, tangent, normal, binormal }
    }
    
    fn derivatives_at(&self, parameter: f64) -> CurveDerivatives {
        let angle = self.start_angle + parameter * (self.end_angle - self.start_angle);
        let d_angle = self.end_angle - self.start_angle;
        let point = Point::new(
            self.center.x + self.radius * angle.cos(),
            self.center.y + self.radius * angle.sin(),
            0.0,
        );
        let first = Vector2::new(
            -self.radius * d_angle * angle.sin(),
            self.radius * d_angle * angle.cos(),
        );
        let second = Vector2::new(
            -self.radius * d_angle.powi(2) * angle.cos(),
            -self.radius * d_angle.powi(2) * angle.sin(),
        );
        let third = Vector2::new(
            self.radius * d_angle.powi(3) * angle.sin(),
            -self.radius * d_angle.powi(3) * angle.cos(),
        );
        
        CurveDerivatives { point, first_derivative: first, second_derivative: second, third_derivative: third }
    }
    
    fn arc_length(&self, _tolerance: f64) -> f64 {
        self.radius * (self.end_angle - self.start_angle).abs()
    }
    
    fn parameter_at_arc_length(&self, length: f64) -> f64 {
        let total_length = self.arc_length(0.001);
        let t = (length / total_length).clamp(0.0, 1.0);
        let angle_span = self.end_angle - self.start_angle;
        if angle_span > 0.0 {
            t
        } else {
            1.0 - t
        }
    }
    
    fn is_g1_continuous(&self, _other: &dyn Curve) -> bool {
        true
    }
    
    fn is_g2_continuous(&self, _other: &dyn Curve) -> bool {
        true
    }
}

pub fn compute_curve_length(curve: &dyn Curve, tolerance: f64) -> f64 {
    match curve {
        Curve::Line(line) => line.length(),
        Curve::Circle(circle) => 2.0 * std::f64::consts::PI * circle.radius,
        Curve::Arc(arc) => arc.radius * (arc.end_angle - arc.start_angle).abs(),
        Curve::Ellipse(_) => 0.0,
        Curve::BSpline(spline) => spline.length(tolerance),
        Curve::NURBS(nurbs) => nurbs.length(tolerance),
        Curve::Polyline(polyline) => {
            let mut length = 0.0;
            for i in 0..polyline.vertices.len().saturating_sub(if polyline.is_closed { 0 } else { 1 }) {
                let next_i = if polyline.is_closed && i + 1 >= polyline.vertices.len() { 0 } else { i + 1 };
                if next_i < polyline.vertices.len() {
                    length += polyline.vertices[i].distance_to(&polyline.vertices[next_i]);
                }
            }
            length
        }
    }
}

pub fn compute_point_on_curve(curve: &dyn Curve, parameter: f64) -> Point {
    match curve {
        Curve::Line(line) => line.point_at_parameter(parameter),
        Curve::Circle(circle) => {
            let angle = parameter * 2.0 * std::f64::consts::PI;
            Point::new(
                circle.center.x + circle.radius * angle.cos(),
                circle.center.y + circle.radius * angle.sin(),
                0.0,
            )
        }
        Curve::Arc(arc) => {
            let angle = arc.start_angle + parameter * (arc.end_angle - arc.start_angle);
            Point::new(
                arc.center.x + arc.radius * angle.cos(),
                arc.center.y + arc.radius * angle.sin(),
                0.0,
            )
        }
        Curve::Ellipse(ellipse) => {
            let angle = parameter * 2.0 * std::f64::consts::PI;
            Point::new(
                ellipse.center.x + ellipse.semi_major * angle.cos(),
                ellipse.center.y + ellipse.semi_minor * angle.sin(),
                0.0,
            )
        }
        _ => Point::origin(),
    }
}
