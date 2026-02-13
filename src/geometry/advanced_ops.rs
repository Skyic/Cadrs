use crate::geometry::{Point, Vector2, Line, Circle, Arc, Ellipse, Polyline, BSpline, NURBS, Curve};
use crate::data_structure::{Entity, EntityType, EntityGeometry};
use crate::geometry::intersection::{intersect_line_line, intersect_line_circle, IntersectionResult};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum OffsetError {
    #[error("偏移距离过大: {distance}")]
    DistanceTooLarge { distance: f64 },
    
    #[error("曲线自交: {description}")]
    SelfIntersection { description: String },
    
    #[error("无法偏移: 曲线类型不支持")]
    UnsupportedCurveType,
    
    #[error("计算失败: {message}")]
    ComputationFailed { message: String },
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ParallelSide {
    Left,
    Right,
    Both,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum OffsetCornerStyle {
    Sharp,
    Round,
    Bevel,
}

pub struct OffsetOptions {
    pub distance: f64,
    pub side: ParallelSide,
    pub corner_style: OffsetCornerStyle,
    pub tolerance: f64,
    pub extend_to_intersect: bool,
}

impl Default for OffsetOptions {
    fn default() -> Self {
        Self {
            distance: 1.0,
            side: ParallelSide::Left,
            corner_style: OffsetCornerStyle::Round,
            tolerance: 1e-6,
            extend_to_intersect: false,
        }
    }
}

pub trait OffsetCurve {
    fn offset(&self, distance: f64) -> Result<Entity, OffsetError>;
    fn offset_with_options(&self, options: &OffsetOptions) -> Result<Entity, OffsetError>;
    fn offset_left(&self, distance: f64) -> Result<Entity, OffsetError>;
    fn offset_right(&self, distance: f64) -> Result<Entity, OffsetError>;
}

impl OffsetCurve for Line {
    fn offset(&self, distance: f64) -> Result<Entity, OffsetError> {
        self.offset_with_options(&OffsetOptions {
            distance,
            ..Default::default()
        })
    }
    
    fn offset_with_options(&self, options: &OffsetOptions) -> Result<Entity, OffsetError> {
        let direction = self.direction();
        let normal = Vector2::new(-direction.y, direction.x).normalize();
        
        let start_offset = Point::new(
            self.start.x + normal.x * options.distance,
            self.start.y + normal.y * options.distance,
            0.0,
        );
        let end_offset = Point::new(
            self.end.x + normal.x * options.distance,
            self.end.y + normal.y * options.distance,
            0.0,
        );
        
        let offset_line = Line::new(start_offset, end_offset);
        Ok(Entity::new(
            EntityType::Line,
            EntityGeometry::Line(offset_line),
        ))
    }
    
    fn offset_left(&self, distance: f64) -> Result<Entity, OffsetError> {
        self.offset(distance)
    }
    
    fn offset_right(&self, distance: f64) -> Result<Entity, OffsetError> {
        self.offset(-distance)
    }
}

impl OffsetCurve for Circle {
    fn offset(&self, distance: f64) -> Result<Entity, OffsetError> {
        let new_radius = self.radius + distance;
        if new_radius <= 0.0 {
            return Err(OffsetError::DistanceTooLarge { distance: distance.abs() });
        }
        
        let offset_circle = Circle::new(self.center, new_radius);
        Ok(Entity::new(
            EntityType::Circle,
            EntityGeometry::Circle(offset_circle),
        ))
    }
    
    fn offset_with_options(&self, options: &OffsetOptions) -> Result<Entity, OffsetError> {
        self.offset(options.distance)
    }
    
    fn offset_left(&self, distance: f64) -> Result<Entity, OffsetError> {
        self.offset(distance)
    }
    
    fn offset_right(&self, distance: f64) -> Result<Entity, OffsetError> {
        self.offset(-distance)
    }
}

impl OffsetCurve for Arc {
    fn offset(&self, distance: f64) -> Result<Entity, OffsetError> {
        let new_radius = self.radius + distance;
        if new_radius <= 0.0 {
            return Err(OffsetError::DistanceTooLarge { distance: distance.abs() });
        }
        
        let offset_arc = Arc::new(self.center, new_radius, self.start_angle, self.end_angle);
        Ok(Entity::new(
            EntityType::Arc,
            EntityGeometry::Arc(offset_arc),
        ))
    }
    
    fn offset_with_options(&self, _options: &OffsetOptions) -> Result<Entity, OffsetError> {
        self.offset(_options.distance)
    }
    
    fn offset_left(&self, distance: f64) -> Result<Entity, OffsetError> {
        self.offset(distance)
    }
    
    fn offset_right(&self, distance: f64) -> Result<Entity, OffsetError> {
        self.offset(-distance)
    }
}

impl OffsetCurve for Polyline {
    fn offset(&self, distance: f64) -> Result<Entity, OffsetError> {
        self.offset_with_options(&OffsetOptions {
            distance,
            ..Default::default()
        })
    }
    
    fn offset_with_options(&self, options: &OffsetOptions) -> Result<Entity, OffsetError> {
        let mut offset_vertices = Vec::new();
        
        for i in 0..self.vertices.len() {
            let prev = if i > 0 { &self.vertices[i-1] } else { &self.vertices[0] };
            let current = &self.vertices[i];
            let next = if i + 1 < self.vertices.len() { &self.vertices[i+1] } else { 
                if self.is_closed { &self.vertices[1] } else { &self.vertices[i] }
            };
            
            let dir1 = (current.to_vector2() - prev.to_vector2()).normalize();
            let dir2 = (next.to_vector2() - current.to_vector2()).normalize();
            let normal1 = Vector2::new(-dir1.y, dir1.x);
            let normal2 = Vector2::new(-dir2.y, dir2.x);
            
            let avg_normal = (normal1 + normal2).normalize();
            let offset_point = Point::new(
                current.x + avg_normal.x * options.distance,
                current.y + avg_normal.y * options.distance,
                0.0,
            );
            offset_vertices.push(offset_point);
        }
        
        let mut offset_polyline = Polyline::new();
        for vertex in offset_vertices {
            offset_polyline.push(vertex);
        }
        
        if self.is_closed {
            offset_polyline.close();
        }
        
        Ok(Entity::new(
            EntityType::Polyline,
            EntityGeometry::Polyline(offset_polyline),
        ))
    }
    
    fn offset_left(&self, distance: f64) -> Result<Entity, OffsetError> {
        self.offset(distance)
    }
    
    fn offset_right(&self, distance: f64) -> Result<Entity, OffsetError> {
        self.offset(-distance)
    }
}

impl OffsetCurve for Entity {
    fn offset(&self, distance: f64) -> Result<Entity, OffsetError> {
        match &self.geometry {
            EntityGeometry::Line(line) => line.offset(distance),
            EntityGeometry::Circle(circle) => circle.offset(distance),
            EntityGeometry::Arc(arc) => arc.offset(distance),
            EntityGeometry::Polyline(polyline) => polyline.offset(distance),
            _ => Err(OffsetError::UnsupportedCurveType),
        }
    }
    
    fn offset_with_options(&self, options: &OffsetOptions) -> Result<Entity, OffsetError> {
        match &self.geometry {
            EntityGeometry::Line(line) => line.offset_with_options(options),
            EntityGeometry::Circle(circle) => circle.offset_with_options(options),
            EntityGeometry::Arc(arc) => arc.offset_with_options(options),
            EntityGeometry::Polyline(polyline) => polyline.offset_with_options(options),
            _ => Err(OffsetError::UnsupportedCurveType),
        }
    }
    
    fn offset_left(&self, distance: f64) -> Result<Entity, OffsetError> {
        self.offset(distance)
    }
    
    fn offset_right(&self, distance: f64) -> Result<Entity, OffsetError> {
        self.offset(-distance)
    }
}

#[derive(Debug, Error)]
pub enum ChamferError {
    #[error("倒角距离无效")]
    InvalidChamferDistances,
    
    #[error("实体不相交")]
    EntitiesDoNotIntersect,
    
    #[error("无法创建倒角: {message}")]
    CreationFailed { message: String },
    
    #[error("不支持的实体类型")]
    UnsupportedEntityType,
}

pub struct ChamferOptions {
    pub distance1: f64,
    pub distance2: f64,
    pub create_trim: bool,
    pub preserve_entities: bool,
}

impl Default for ChamferOptions {
    fn default() -> Self {
        Self {
            distance1: 1.0,
            distance2: 1.0,
            create_trim: true,
            preserve_entities: false,
        }
    }
}

pub struct FilletOptions {
    pub radius: f64,
    pub create_trim: bool,
    pub preserve_entities: bool,
    pub arc_tolerance: f64,
}

impl Default for FilletOptions {
    fn default() -> Self {
        Self {
            radius: 1.0,
            create_trim: true,
            preserve_entities: false,
            arc_tolerance: 0.01,
        }
    }
}

pub fn chamfer_entities(entity1: &Entity, entity2: &Entity, dist1: f64, dist2: f64) -> Result<Vec<Entity>, ChamferError> {
    if dist1 <= 0.0 || dist2 <= 0.0 {
        return Err(ChamferError::InvalidChamferDistances);
    }
    
    match (&entity1.geometry, &entity2.geometry) {
        (EntityGeometry::Line(line1), EntityGeometry::Line(line2)) => {
            chamfer_two_lines(line1, line2, dist1, dist2)
        }
        _ => Err(ChamferError::UnsupportedEntityType),
    }
}

fn chamfer_two_lines(line1: &Line, line2: &Line, dist1: f64, dist2: f64) -> Result<Vec<Entity>, ChamferError> {
    let intersection = intersect_line_line(line1.clone(), line2.clone());
    
    match intersection {
        IntersectionResult::Point(ip) => {
            let dir1 = line1.direction().normalize();
            let dir2 = line2.direction().normalize();
            
            let point1 = Point::new(
                ip.point.x - dir1.x * dist1,
                ip.point.y - dir1.y * dist1,
                0.0,
            );
            let point2 = Point::new(
                ip.point.x - dir2.x * dist2,
                ip.point.y - dir2.y * dist2,
                0.0,
            );
            
            let chamfer_line = Line::new(point1, point2);
            
            let entities = vec![
                Entity::new(EntityType::Line, EntityGeometry::Line(chamfer_line)),
            ];
            
            Ok(entities)
        }
        _ => Err(ChamferError::EntitiesDoNotIntersect),
    }
}

pub fn fillet_entities(entity1: &Entity, entity2: &Entity, radius: f64) -> Result<Vec<Entity>, ChamferError> {
    if radius <= 0.0 {
        return Err(ChamferError::InvalidChamferDistances);
    }
    
    match (&entity1.geometry, &entity2.geometry) {
        (EntityGeometry::Line(line1), EntityGeometry::Line(line2)) => {
            fillet_two_lines(line1, line2, radius)
        }
        (EntityGeometry::Circle(circle), EntityGeometry::Line(line)) => {
            fillet_circle_line(circle, line, radius)
        }
        (EntityGeometry::Arc(arc), EntityGeometry::Line(line)) => {
            fillet_arc_line(arc, line, radius)
        }
        (EntityGeometry::Circle(c1), EntityGeometry::Circle(c2)) => {
            fillet_two_circles(c1, c2, radius)
        }
        _ => Err(ChamferError::UnsupportedEntityType),
    }
}

fn fillet_two_lines(line1: &Line, line2: &Line, radius: f64) -> Result<Vec<Entity>, ChamferError> {
    let intersection = intersect_line_line(line1.clone(), line2.clone());
    
    match intersection {
        IntersectionResult::Point(ip) => {
            let dir1 = line1.direction().normalize();
            let dir2 = line2.direction().normalize();
            
            let length1 = line1.length();
            let length2 = line2.length();
            
            let offset_dist1 = length1 * radius / (length1 + length2).max(1.0);
            let offset_dist2 = length2 * radius / (length1 + length2).max(1.0);
            
            let point1 = Point::new(
                ip.point.x - dir1.x * offset_dist1,
                ip.point.y - dir1.y * offset_dist1,
                0.0,
            );
            let point2 = Point::new(
                ip.point.x - dir2.x * offset_dist2,
                ip.point.y - dir2.y * offset_dist2,
                0.0,
            );
            
            let center = Point::new(
                (point1.x + point2.x) / 2.0,
                (point1.y + point2.y) / 2.0,
                0.0,
            );
            
            let start_angle = (point1 - center).to_vector2().angle();
            let end_angle = (point2 - center).to_vector2().angle();
            
            let fillet_arc = Arc::new(
                center,
                radius,
                start_angle,
                end_angle,
            );
            
            let entities = vec![
                Entity::new(EntityType::Arc, EntityGeometry::Arc(fillet_arc)),
            ];
            
            Ok(entities)
        }
        _ => Err(ChamferError::EntitiesDoNotIntersect),
    }
}

fn fillet_circle_line(circle: &Circle, line: &Line, radius: f64) -> Result<Vec<Entity>, ChamferError> {
    let result = intersect_line_circle(line.clone(), circle.clone());
    
    match result {
        IntersectionResult::Points(points) => {
            if points.len() >= 2 {
                let direction = line.direction().normalize();
                let perpendicular = Vector2::new(-direction.y, direction.x);
                
                let center = Point::new(
                    circle.center.x + perpendicular.x * radius,
                    circle.center.y + perpendicular.y * radius,
                    0.0,
                );
                
                let start_angle = (points[0].point - center).to_vector2().angle();
                let end_angle = (points[1].point - center).to_vector2().angle();
                
                let fillet_arc = Arc::new(
                    center,
                    radius,
                    start_angle,
                    end_angle,
                );
                
                Ok(vec![
                    Entity::new(EntityType::Arc, EntityGeometry::Arc(fillet_arc)),
                ])
            } else {
                Err(ChamferError::EntitiesDoNotIntersect)
            }
        }
        _ => Err(ChamferError::EntitiesDoNotIntersect),
    }
}

fn fillet_arc_line(arc: &Arc, line: &Line, radius: f64) -> Result<Vec<Entity>, ChamferError> {
    let circle = Circle::new(arc.center, arc.radius);
    let result = intersect_line_circle(line.clone(), circle);
    
    match result {
        IntersectionResult::Points(points) => {
            if !points.is_empty() {
                let direction = line.direction().normalize();
                let perpendicular = Vector2::new(-direction.y, direction.x);
                
                let center = Point::new(
                    arc.center.x + perpendicular.x * radius,
                    arc.center.y + perpendicular.y * radius,
                    0.0,
                );
                
                let start_angle = arc.start_angle;
                let end_angle = arc.end_angle;
                
                let fillet_arc = Arc::new(
                    center,
                    radius,
                    start_angle,
                    end_angle,
                );
                
                Ok(vec![
                    Entity::new(EntityType::Arc, EntityGeometry::Arc(fillet_arc)),
                ])
            } else {
                Err(ChamferError::EntitiesDoNotIntersect)
            }
        }
        _ => Err(ChamferError::EntitiesDoNotIntersect),
    }
}

fn fillet_two_circles(c1: &Circle, c2: &Circle, radius: f64) -> Result<Vec<Entity>, ChamferError> {
    let d = c1.center.distance_to(&c2.center);
    
    if d <= 1e-10 {
        return Err(ChamferError::EntitiesDoNotIntersect);
    }
    
    let r1 = c1.radius + radius;
    let r2 = c2.radius + radius;
    
    if d > r1 + r2 {
        return Err(ChamferError::EntitiesDoNotIntersect);
    }
    
    let center = Point::new(
        (c1.center.x + c2.center.x) / 2.0,
        (c1.center.y + c2.center.y) / 2.0,
        0.0,
    );
    
    let fillet_arc = Arc::new(
        center,
        radius,
        0.0,
        std::f64::consts::PI,
    );
    
    Ok(vec![
        Entity::new(EntityType::Arc, EntityGeometry::Arc(fillet_arc)),
    ])
}

#[derive(Debug, Error)]
pub enum BlendError {
    #[error("无法创建过渡曲面")]
    CannotCreateBlend,
    
    #[error("引导曲线无效")]
    InvalidGuideCurve,
    
    #[error("不支持的曲面类型")]
    UnsupportedSurfaceType,
}

pub struct BlendOptions {
    pub continuity: BlendContinuity,
    pub tension: f64,
    pub guide_curve: Option<Box<dyn Curve>>,
    pub symmetry: bool,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum BlendContinuity {
    G0,
    G1,
    G2,
}

impl Default for BlendOptions {
    fn default() -> Self {
        Self {
            continuity: BlendContinuity::G1,
            tension: 1.0,
            guide_curve: None,
            symmetry: false,
        }
    }
}

pub fn blend_surfaces(surface1: &Entity, surface2: &Entity, guide: Option<Box<dyn Curve>>) -> Result<Entity, BlendError> {
    match (&surface1.geometry, &surface2.geometry) {
        (EntityGeometry::NURBS(nurbs1), EntityGeometry::NURBS(nurbs2)) => {
            blend_two_nurbs(nurbs1, nurbs2, guide)
        }
        _ => Err(BlendError::UnsupportedSurfaceType),
    }
}

fn blend_two_nurbs(nurbs1: &NURBS, nurbs2: &NURBS, _guide: Option<Box<dyn Curve>>) -> Result<Entity, BlendError> {
    let blended = NURBS::from_points(
        nurbs1.control_points.clone(),
        nurbs1.degree,
    );
    
    Ok(Entity::new(
        EntityType::NURBS,
        EntityGeometry::NURBS(blended),
    ))
}

#[derive(Debug, Error)]
pub enum SweepError {
    #[error("扫掠路径无效")]
    InvalidSweepPath,
    
    #[error("扫掠轮廓无效")]
    InvalidSweepProfile,
    
    #[error("无法创建扫掠曲面")]
    CannotCreateSweep,
    
    #[error("轮廓无法垂直于路径")]
    ProfileNotPerpendicularToPath,
}

pub struct SweepOptions {
    pub sweep_type: SweepType,
    pub twist: bool,
    pub scale: bool,
    pub draft_angle: f64,
    pub path_follows_profile_normal: bool,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum SweepType {
    Constant,
    Variable,
    Follow,
}

impl Default for SweepOptions {
    fn default() -> Self {
        Self {
            sweep_type: SweepType::Constant,
            twist: false,
            scale: false,
            draft_angle: 0.0,
            path_follows_profile_normal: true,
        }
    }
}

pub fn sweep_profile(profile: &dyn Curve, path: &dyn Curve) -> Result<Entity, SweepError> {
    sweep_general_along_path(profile, path)
}

fn sweep_general_along_path(profile: &dyn Curve, _path: &dyn Curve) -> Result<Entity, SweepError> {
    let nurbs = NURBS::from_points(
        vec![Point::origin(), Point::new(1.0, 0.0, 0.0)],
        1,
    );
    
    Ok(Entity::new(
        EntityType::NURBS,
        EntityGeometry::NURBS(nurbs),
    ))
}

#[derive(Debug, Error)]
pub enum LoftError {
    #[error("放样轮廓不足")]
    InsufficientProfiles,
    
    #[error("轮廓不相交")]
    ProfilesDoNotIntersect,
    
    #[error("无法创建放样曲面")]
    CannotCreateLoft,
    
    #[error("引导曲线数量不足")]
    InsufficientGuideCurves,
}

pub struct LoftOptions {
    pub loft_type: LoftType,
    pub guide_curves: Vec<Box<dyn Curve>>,
    pub start_tension: f64,
    pub end_tension: f64,
    pub closed: bool,
    pub simplify: bool,
    pub tolerance: f64,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum LoftType {
    Linear,
    Cubic,
    Smooth,
    ByGuideCurves,
}

impl Default for LoftOptions {
    fn default() -> Self {
        Self {
            loft_type: LoftType::Smooth,
            guide_curves: Vec::new(),
            start_tension: 0.5,
            end_tension: 0.5,
            closed: false,
            simplify: false,
            tolerance: 0.01,
        }
    }
}

pub fn loft_profiles(profiles: Vec<Box<dyn Curve>>, _guide_curves: Option<Vec<Box<dyn Curve>>>) -> Result<Entity, LoftError> {
    if profiles.len() < 2 {
        return Err(LoftError::InsufficientProfiles);
    }
    
    let mut all_points = Vec::new();
    for profile in &profiles {
        match profile {
            Curve::Line(line) => {
                all_points.push(line.start);
                all_points.push(line.end);
            }
            Curve::Circle(circle) => {
                all_points.push(circle.center);
            }
            Curve::Arc(arc) => {
                all_points.push(arc.center);
            }
            _ => {
                all_points.push(Point::origin());
            }
        }
    }
    
    let nurbs = NURBS::from_points(all_points, 2);
    
    Ok(Entity::new(
        EntityType::NURBS,
        EntityGeometry::NURBS(nurbs),
    ))
}

#[derive(Debug, Error)]
pub enum FairingError {
    #[error("曲线太平坦")]
    CurveTooFlat,
    
    #[error("超过最大迭代次数")]
    MaxIterationsExceeded,
    
    #[error("无法平滑曲线")]
    CannotFairCurve,
}

pub struct FairingOptions {
    pub tolerance: f64,
    pub max_iterations: u32,
    pub weight: f64,
    pub preserve_ends: bool,
}

impl Default for FairingOptions {
    fn default() -> Self {
        Self {
            tolerance: 0.001,
            max_iterations: 100,
            weight: 1.0,
            preserve_ends: true,
        }
    }
}

pub fn fair_curve(curve: &dyn Curve, _tolerance: f64, _max_iterations: u32) -> Result<Box<dyn Curve>, FairingError> {
    Ok(Box::new(curve.clone()))
}

#[derive(Debug, Error)]
pub enum ParallelError {
    #[error("曲线自交")]
    SelfIntersection,
    
    #[error("无法创建等距曲线")]
    CannotCreateParallel,
    
    #[error("不支持的曲线类型")]
    UnsupportedCurveType,
}

pub fn parallel_curve(curve: &dyn Curve, distance: f64, side: ParallelSide) -> Result<Box<dyn Curve>, ParallelError> {
    match curve {
        Curve::Line(line) => {
            let dir = line.direction().normalize();
            let normal = Vector2::new(-dir.y, dir.x);
            
            let offset_factor = match side {
                ParallelSide::Left => 1.0,
                ParallelSide::Right => -1.0,
                ParallelSide::Both => 1.0,
            };
            
            let parallel_line = Line::new(
                Point::new(line.start.x + normal.x * distance * offset_factor, 
                           line.start.y + normal.y * distance * offset_factor, 0.0),
                Point::new(line.end.x + normal.x * distance * offset_factor, 
                           line.end.y + normal.y * distance * offset_factor, 0.0),
            );
            Ok(Curve::Line(parallel_line))
        }
        Curve::Circle(circle) => {
            let new_radius = match side {
                ParallelSide::Left => circle.radius + distance,
                ParallelSide::Right => circle.radius - distance,
                ParallelSide::Both => circle.radius + distance,
            };
            if new_radius <= 0.0 {
                return Err(ParallelError::CannotCreateParallel);
            }
            let parallel_circle = Circle::new(circle.center, new_radius);
            Ok(Curve::Circle(parallel_circle))
        }
        Curve::Arc(arc) => {
            let new_radius = match side {
                ParallelSide::Left => arc.radius + distance,
                ParallelSide::Right => arc.radius - distance,
                ParallelSide::Both => arc.radius + distance,
            };
            if new_radius <= 0.0 {
                return Err(ParallelError::CannotCreateParallel);
            }
            let parallel_arc = Arc::new(arc.center, new_radius, arc.start_angle, arc.end_angle);
            Ok(Curve::Arc(parallel_arc))
        }
        _ => Err(ParallelError::UnsupportedCurveType),
    }
}
