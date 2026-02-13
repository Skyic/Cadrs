use super::geometry::{Point, Line, Circle, Arc};
use std::collections::{HashMap, HashSet, VecDeque};

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum GeometricConstraint {
    Coincident(PointConstraint, PointConstraint),
    Horizontal(PointConstraint, PointConstraint),
    Vertical(PointConstraint, PointConstraint),
    Parallel(LineConstraint, LineConstraint),
    Perpendicular(LineConstraint, LineConstraint),
    Tangent(CurveConstraint, CurveConstraint),
    Concentric(CircleConstraint, CircleConstraint),
    EqualLength(LineConstraint, LineConstraint),
    EqualRadius(CircleConstraint, CircleConstraint),
    Midpoint(PointConstraint, LineConstraint),
    PointOnLine(PointConstraint, LineConstraint),
    PointOnCircle(PointConstraint, CircleConstraint),
    PointOnArc(PointConstraint, ArcConstraint),
    Symmetry(PointConstraint, PointConstraint, LineConstraint),
    Angle(LineConstraint, LineConstraint, f64),
    Collinear(LineConstraint, LineConstraint),
    ParallelX(LineConstraint),
    ParallelY(LineConstraint),
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum PointConstraint {
    EntityPoint(usize, usize),
    FreePoint(usize),
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum LineConstraint {
    EntityLine(usize),
    ThroughPoints(PointConstraint, PointConstraint),
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum CircleConstraint {
    EntityCircle(usize),
    CenterRadius(PointConstraint, f64),
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ArcConstraint {
    EntityArc(usize),
    CenterRadiusAngles(PointConstraint, f64, f64, f64),
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum CurveConstraint {
    Line(LineConstraint),
    Circle(CircleConstraint),
    Arc(ArcConstraint),
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum DimensionalConstraint {
    Distance(PointConstraint, PointConstraint, f64),
    Angle(LineConstraint, LineConstraint, f64),
    Radius(CircleConstraint, f64),
    Diameter(CircleConstraint, f64),
    Length(LineConstraint, f64),
}

#[derive(Debug, Clone)]
pub struct ConstraintSystem {
    pub geometric_constraints: Vec<GeometricConstraint>,
    pub dimensional_constraints: Vec<DimensionalConstraint>,
    pub entities: Vec<ConstrainedEntity>,
    pub solver_settings: SolverSettings,
}

impl ConstraintSystem {
    pub fn new() -> Self {
        Self {
            geometric_constraints: Vec::new(),
            dimensional_constraints: Vec::new(),
            entities: Vec::new(),
            solver_settings: SolverSettings::default(),
        }
    }

    pub fn add_entity(&mut self, entity: ConstrainedEntity) -> usize {
        let id = self.entities.len();
        self.entities.push(entity);
        id
    }

    pub fn add_geometric_constraint(&mut self, constraint: GeometricConstraint) {
        self.geometric_constraints.push(constraint);
    }

    pub fn add_dimensional_constraint(&mut self, constraint: DimensionalConstraint) {
        self.dimensional_constraints.push(constraint);
    }

    pub fn solve(&mut self) -> SolverResult {
        let mut solver = ConstraintSolver::new(self);
        solver.solve()
    }

    pub fn get_entity(&self, id: usize) -> Option<&ConstrainedEntity> {
        self.entities.get(id)
    }

    pub fn get_entity_mut(&mut self, id: usize) -> Option<&mut ConstrainedEntity> {
        self.entities.get_mut(id)
    }

    pub fn is_fully_constrained(&self) -> bool {
        let degrees_of_freedom = self.calculate_degrees_of_freedom();
        degrees_of_freedom == 0
    }

    pub fn is_under_constrained(&self) -> bool {
        let degrees_of_freedom = self.calculate_degrees_of_freedom();
        degrees_of_freedom > 0
    }

    pub fn is_over_constrained(&self) -> bool {
        let degrees_of_freedom = self.calculate_degrees_of_freedom();
        degrees_of_freedom < 0
    }

    pub fn calculate_degrees_of_freedom(&self) -> i32 {
        let mut dof = 0;

        for entity in &self.entities {
            dof += entity.degrees_of_freedom();
        }

        let num_constraints = self.geometric_constraints.len() + self.dimensional_constraints.len();
        dof -= num_constraints as i32;

        dof
    }

    pub fn get_constraint_graph(&self) -> ConstraintGraph {
        let mut graph = ConstraintGraph::new();

        for (id, _) in self.entities.iter().enumerate() {
            graph.add_node(id);
        }

        for constraint in &self.geometric_constraints {
            let entities = constraint.get_constrained_entities();
            for &entity_id in &entities {
                for &other_id in &entities {
                    if entity_id != other_id {
                        graph.add_edge(entity_id, other_id);
                    }
                }
            }
        }

        for constraint in &self.dimensional_constraints {
            let entities = constraint.get_constrained_entities();
            for &entity_id in &entities {
                for &other_id in &entities {
                    if entity_id != other_id {
                        graph.add_edge(entity_id, other_id);
                    }
                }
            }
        }

        graph
    }

    pub fn validate_constraints(&self) -> Vec<ValidationError> {
        let mut errors = Vec::new();

        for (i, constraint) in self.geometric_constraints.iter().enumerate() {
            if !constraint.is_valid() {
                errors.push(ValidationError::InvalidGeometricConstraint(i));
            }
        }

        for (i, constraint) in self.dimensional_constraints.iter().enumerate() {
            if !constraint.is_valid() {
                errors.push(ValidationError::InvalidDimensionalConstraint(i));
            }
        }

        let dof = self.calculate_degrees_of_freedom();
        if dof < 0 {
            errors.push(ValidationError::OverConstrained {
                excess_constraints: -dof as usize,
            });
        }

        errors
    }
}

impl Default for ConstraintSystem {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Debug, Clone)]
pub struct ConstrainedEntity {
    pub id: usize,
    pub entity_type: EntityType,
    pub parameters: Vec<f64>,
    pub is_fixed: bool,
    pub is_reference: bool,
}

impl ConstrainedEntity {
    pub fn new(id: usize, entity_type: EntityType) -> Self {
        let parameters = entity_type.get_parameters();
        Self {
            id,
            entity_type,
            parameters,
            is_fixed: false,
            is_reference: false,
        }
    }

    pub fn as_fixed(mut self) -> Self {
        self.is_fixed = true;
        self
    }

    pub fn as_reference(mut self) -> Self {
        self.is_reference = true;
        self
    }

    pub fn degrees_of_freedom(&self) -> i32 {
        if self.is_fixed {
            return 0;
        }
        self.entity_type.degrees_of_freedom()
    }

    pub fn get_point(&self) -> Option<Point> {
        self.entity_type.get_point(&self.parameters)
    }

    pub fn get_line(&self) -> Option<(Point, Point)> {
        self.entity_type.get_line(&self.parameters)
    }

    pub fn get_circle(&self) -> Option<(Point, f64)> {
        self.entity_type.get_circle(&self.parameters)
    }

    pub fn get_arc(&self) -> Option<(Point, f64, f64, f64)> {
        self.entity_type.get_arc(&self.parameters)
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum EntityType {
    Point,
    Line,
    Circle,
    Arc,
}

impl EntityType {
    fn get_parameters(&self) -> Vec<f64> {
        match self {
            EntityType::Point => vec![0.0, 0.0],
            EntityType::Line => vec![0.0, 0.0, 1.0, 0.0],
            EntityType::Circle => vec![0.0, 0.0, 1.0],
            EntityType::Arc => vec![0.0, 0.0, 1.0, 0.0, 1.0],
        }
    }

    fn degrees_of_freedom(&self) -> i32 {
        match self {
            EntityType::Point => 2,
            EntityType::Line => 4,
            EntityType::Circle => 3,
            EntityType::Arc => 5,
        }
    }

    fn get_point(&self, parameters: &[f64]) -> Option<Point> {
        match self {
            EntityType::Point => Some(Point::new(parameters[0], parameters[1])),
            _ => None,
        }
    }

    fn get_line(&self, parameters: &[f64]) -> Option<(Point, Point)> {
        match self {
            EntityType::Line => Some((
                Point::new(parameters[0], parameters[1]),
                Point::new(parameters[2], parameters[3]),
            )),
            _ => None,
        }
    }

    fn get_circle(&self, parameters: &[f64]) -> Option<(Point, f64)> {
        match self {
            EntityType::Circle => Some((
                Point::new(parameters[0], parameters[1]),
                parameters[2],
            )),
            _ => None,
        }
    }

    fn get_arc(&self, parameters: &[f64]) -> Option<(Point, f64, f64, f64)> {
        match self {
            EntityType::Arc => Some((
                Point::new(parameters[0], parameters[1]),
                parameters[2],
                parameters[3],
                parameters[4],
            )),
            _ => None,
        }
    }
}

#[derive(Debug, Clone)]
pub struct SolverSettings {
    pub max_iterations: usize,
    pub tolerance: f64,
    pub damping_factor: f64,
    pub use_damping: bool,
    pub constraint_weight: f64,
    pub use_gauss_newton: bool,
    pub debug_output: bool,
}

impl SolverSettings {
    pub fn new() -> Self {
        Self {
            max_iterations: 100,
            tolerance: 1e-6,
            damping_factor: 0.5,
            use_damping: true,
            constraint_weight: 1.0,
            use_gauss_newton: true,
            debug_output: false,
        }
    }

    pub fn with_max_iterations(mut self, iterations: usize) -> Self {
        self.max_iterations = iterations;
        self
    }

    pub fn with_tolerance(mut self, tolerance: f64) -> Self {
        self.tolerance = tolerance;
        self
    }

    pub fn with_damping(mut self, damping: f64) -> Self {
        self.damping_factor = damping;
        self
    }

    pub fn with_debug(mut self) -> Self {
        self.debug_output = true;
        self
    }
}

impl Default for SolverSettings {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Debug, Clone)]
pub struct SolverResult {
    pub success: bool,
    pub iterations: usize,
    pub residual_error: f64,
    pub converged: bool,
    pub message: String,
}

impl SolverResult {
    pub fn success(iterations: usize, residual: f64) -> Self {
        Self {
            success: true,
            iterations,
            residual_error: residual,
            converged: true,
            message: String::from("求解成功"),
        }
    }

    pub fn failure(message: String) -> Self {
        Self {
            success: false,
            iterations: 0,
            residual_error: 0.0,
            converged: false,
            message,
        }
    }
}

struct ConstraintSolver<'a> {
    system: &'a mut ConstraintSystem,
    jacobian: Vec<Vec<f64>>,
    residuals: Vec<f64>,
    delta: Vec<f64>,
}

impl<'a> ConstraintSolver<'a> {
    fn new(system: &'a mut ConstraintSystem) -> Self {
        Self {
            system,
            jacobian: Vec::new(),
            residuals: Vec::new(),
            delta: Vec::new(),
        }
    }

    fn solve(&mut self) -> SolverResult {
        let total_dof = self.system.entities.iter()
            .map(|e| e.degrees_of_freedom() as usize)
            .sum();

        let num_constraints = self.system.geometric_constraints.len() + 
                            self.system.dimensional_constraints.len();

        if total_dof == 0 {
            return SolverResult::success(0, 0.0);
        }

        let mut iterations = 0;
        let mut max_error = f64::MAX;

        while iterations < self.system.solver_settings.max_iterations && 
              max_error > self.system.solver_settings.tolerance {

            self.build_system();

            if self.residuals.is_empty() {
                return SolverResult::success(iterations, 0.0);
            }

            max_error = self.residuals.iter()
                .map(|r| r.abs())
                .fold(0.0, f64::max);

            if max_error < self.system.solver_settings.tolerance {
                break;
            }

            self.solve_system();

            self.apply_deltas();

            iterations += 1;
        }

        if max_error < self.system.solver_settings.tolerance {
            SolverResult::success(iterations, max_error)
        } else {
            SolverResult::failure(format!("在{}次迭代后未收敛，残差：{}", iterations, max_error))
        }
    }

    fn build_system(&mut self) {
        let total_dof = self.system.entities.iter()
            .filter(|e| !e.is_fixed)
            .map(|e| e.degrees_of_freedom() as usize)
            .sum();

        let num_constraints = self.system.geometric_constraints.len() + 
                            self.system.dimensional_constraints.len();

        self.jacobian.clear();
        self.residuals.clear();
        self.jacobian.resize(num_constraints, vec![0.0; total_dof]);
        self.residuals.resize(num_constraints, 0.0);

        let mut row = 0;
        for constraint in &self.system.geometric_constraints {
            self.evaluate_geometric_constraint(row, constraint);
            row += 1;
        }

        for constraint in &self.system.dimensional_constraints {
            self.evaluate_dimensional_constraint(row, constraint);
            row += 1;
        }
    }

#[cfg(feature = "constraint")]
impl ConstraintSolver<'_> {
    fn evaluate_geometric_constraint(&mut self, row: usize, constraint: &GeometricConstraint) {
        match constraint {
            GeometricConstraint::Coincident(p1, p2) => {
                let pos1 = self.get_point_position(p1);
                let pos2 = self.get_point_position(p2);
                self.residuals[row] = (pos1 - pos2).norm();
                self.set_jacobian_coincident(row, p1, p2);
            }
            GeometricConstraint::Horizontal(p1, p2) => {
                let pos1 = self.get_point_position(p1);
                let pos2 = self.get_point_position(p2);
                self.residuals[row] = pos2.y - pos1.y;
                self.set_jacobian_horizontal(row, p1, p2);
            }
            GeometricConstraint::Vertical(p1, p2) => {
                let pos1 = self.get_point_position(p1);
                let pos2 = self.get_point_position(p2);
                self.residuals[row] = pos2.x - pos1.x;
                self.set_jacobian_vertical(row, p1, p2);
            }
            GeometricConstraint::Parallel(l1, l2) => {
                let dir1 = self.get_line_direction(l1);
                let dir2 = self.get_line_direction(l2);
                self.residuals[row] = dir1.cross(dir2);
                self.set_jacobian_parallel(row, l1, l2);
            }
            GeometricConstraint::Perpendicular(l1, l2) => {
                let dir1 = self.get_line_direction(l1);
                let dir2 = self.get_line_direction(l2);
                self.residuals[row] = dir1.dot(dir2);
                self.set_jacobian_perpendicular(row, l1, l2);
            }
            GeometricConstraint::Tangent(c1, c2) => {
                self.evaluate_tangent_constraint(row, c1, c2);
            }
            GeometricConstraint::Concentric(c1, c2) => {
                self.evaluate_concentric_constraint(row, c1, c2);
            }
            GeometricConstraint::EqualLength(l1, l2) => {
                let len1 = self.get_line_length(l1);
                let len2 = self.get_line_length(l2);
                self.residuals[row] = len1 - len2;
                self.set_jacobian_equal_length(row, l1, l2);
            }
            GeometricConstraint::EqualRadius(c1, c2) => {
                let r1 = self.get_circle_radius(c1);
                let r2 = self.get_circle_radius(c2);
                self.residuals[row] = r1 - r2;
                self.set_jacobian_equal_radius(row, c1, c2);
            }
            GeometricConstraint::Midpoint(p, l) => {
                self.evaluate_midpoint_constraint(row, p, l);
            }
            GeometricConstraint::PointOnLine(p, l) => {
                self.evaluate_point_on_line_constraint(row, p, l);
            }
            GeometricConstraint::PointOnCircle(p, c) => {
                self.evaluate_point_on_circle_constraint(row, p, c);
            }
            GeometricConstraint::PointOnArc(p, a) => {
                self.evaluate_point_on_arc_constraint(row, p, a);
            }
            GeometricConstraint::Symmetry(p1, p2, l) => {
                self.evaluate_symmetry_constraint(row, p1, p2, l);
            }
            GeometricConstraint::Angle(l1, l2, target) => {
                let dir1 = self.get_line_direction(l1);
                let dir2 = self.get_line_direction(l2);
                let angle = dir1.angle_between(dir2);
                self.residuals[row] = angle - target;
                self.set_jacobian_angle(row, l1, l2);
            }
            GeometricConstraint::Collinear(l1, l2) => {
                let dir1 = self.get_line_direction(l1);
                let dir2 = self.get_line_direction(l2);
                self.residuals[row] = dir1.cross(dir2);
                self.set_jacobian_collinear(row, l1, l2);
            }
            GeometricConstraint::ParallelX(l) => {
                let dir = self.get_line_direction(l);
                self.residuals[row] = dir.y;
                self.set_jacobian_parallel_x(row, l);
            }
            GeometricConstraint::ParallelY(l) => {
                let dir = self.get_line_direction(l);
                self.residuals[row] = dir.x;
                self.set_jacobian_parallel_y(row, l);
            }
        }
    }

    fn evaluate_tangent_constraint(&mut self, row: usize, c1: &CurveConstraint, c2: &CurveConstraint) {
        match (c1, c2) {
            (CurveConstraint::Line(l1), CurveConstraint::Circle(c2)) => {
                let line_p1 = self.get_line_endpoint(l1, 0);
                let line_p2 = self.get_line_endpoint(l1, 1);
                let center = self.get_circle_center(c2);
                let radius = self.get_circle_radius(c2);

                let to_center = center - line_p1;
                let line_dir = (line_p2 - line_p1).normalized();
                let proj = to_center.dot(line_dir);
                let closest = line_p1 + line_dir * proj;
                let to_closest = closest - center;

                self.residuals[row] = to_closest.norm() - radius;
            }
            (CurveConstraint::Circle(c1), CurveConstraint::Line(l2)) => {
                self.evaluate_tangent_constraint(row, c2, c1);
            }
            (CurveConstraint::Circle(c1), CurveConstraint::Circle(c2)) => {
                let center1 = self.get_circle_center(c1);
                let center2 = self.get_circle_center(c2);
                let r1 = self.get_circle_radius(c1);
                let r2 = self.get_circle_radius(c2);

                let dist = (center1 - center2).norm();
                self.residuals[row] = dist - (r1 + r2);
            }
            _ => {
                self.residuals[row] = 0.0;
            }
        }
    }

    fn evaluate_concentric_constraint(&mut self, row: usize, c1: &CircleConstraint, c2: &CircleConstraint) {
        let center1 = self.get_circle_center(c1);
        let center2 = self.get_circle_center(c2);
        self.residuals[row] = (center1 - center2).norm();
        self.set_jacobian_concentric(row, c1, c2);
    }

    fn evaluate_midpoint_constraint(&mut self, row: usize, p: &PointConstraint, l: &LineConstraint) {
        let point_pos = self.get_point_position(p);
        let line_p1 = self.get_line_endpoint(l, 0);
        let line_p2 = self.get_line_endpoint(l, 1);
        let midpoint = (line_p1 + line_p2) * 0.5;

        self.residuals[row] = (point_pos - midpoint).norm();
        self.set_jacobian_midpoint(row, p, l);
    }

    fn evaluate_point_on_line_constraint(&mut self, row: usize, p: &PointConstraint, l: &LineConstraint) {
        let point_pos = self.get_point_position(p);
        let line_p1 = self.get_line_endpoint(l, 0);
        let line_p2 = self.get_line_endpoint(l, 1);
        let line_dir = (line_p2 - line_p1).normalized();
        let to_point = point_pos - line_p1;
        let proj = to_point.dot(line_dir);
        let closest = line_p1 + line_dir * proj;

        self.residuals[row] = (point_pos - closest).norm();
        self.set_jacobian_point_on_line(row, p, l);
    }

    fn evaluate_point_on_circle_constraint(&mut self, row: usize, p: &PointConstraint, c: &CircleConstraint) {
        let point_pos = self.get_point_position(p);
        let center = self.get_circle_center(c);
        let radius = self.get_circle_radius(c);

        let dist = (point_pos - center).norm();
        self.residuals[row] = dist - radius;
        self.set_jacobian_point_on_circle(row, p, c);
    }

    fn evaluate_point_on_arc_constraint(&mut self, row: usize, p: &PointConstraint, a: &ArcConstraint) {
        let point_pos = self.get_point_position(p);
        let arc_center = self.get_arc_center(a);
        let arc_radius = self.get_arc_radius(a);
        let arc_start = self.get_arc_start_angle(a);
        let arc_end = self.get_arc_end_angle(a);

        let to_point = point_pos - arc_center;
        let point_angle = to_point.y.atan2(to_point.x);

        let normalized_point = if arc_end > arc_start {
            point_angle >= arc_start && point_angle <= arc_end
        } else {
            point_angle >= arc_start || point_angle <= arc_end
        };

        let radial_dist = (to_point.norm() - arc_radius).abs();

        if normalized_point {
            self.residuals[row] = radial_dist;
        } else {
            let start_dist = (point_pos - self.get_arc_point_at_angle(a, arc_start)).norm();
            let end_dist = (point_pos - self.get_arc_point_at_angle(a, arc_end)).norm();
            self.residuals[row] = radial_dist.min(start_dist).min(end_dist);
        }
    }

    fn evaluate_symmetry_constraint(&mut self, row: usize, p1: &PointConstraint, p2: &PointConstraint, l: &LineConstraint) {
        let pos1 = self.get_point_position(p1);
        let pos2 = self.get_point_position(p2);
        let line_p1 = self.get_line_endpoint(l, 0);
        let line_p2 = self.get_line_endpoint(l, 1);
        let line_dir = (line_p2 - line_p1).normalized();
        let normal = Vector2D::new(-line_dir.y, line_dir.x);

        let mid = (pos1 + pos2) * 0.5;
        let to_mid = mid - line_p1;
        let proj = to_mid.dot(line_dir);
        let closest = line_p1 + line_dir * proj;

        self.residuals[row] = (mid - closest).norm();
    }

    fn get_circle_center(&self, constraint: &CircleConstraint) -> Vector2D {
        match constraint {
            CircleConstraint::EntityCircle(entity_id) => {
                if let Some(entity) = self.system.entities.get(*entity_id) {
                    Vector2D::new(entity.parameters[0], entity.parameters[1])
                } else {
                    Vector2D::new(0.0, 0.0)
                }
            }
            CircleConstraint::CenterRadius(p, _) => self.get_point_position(p),
        }
    }

    fn get_arc_center(&self, constraint: &ArcConstraint) -> Vector2D {
        match constraint {
            ArcConstraint::EntityArc(entity_id) => {
                if let Some(entity) = self.system.entities.get(*entity_id) {
                    Vector2D::new(entity.parameters[0], entity.parameters[1])
                } else {
                    Vector2D::new(0.0, 0.0)
                }
            }
            ArcConstraint::CenterRadiusAngles(p, _, _, _) => self.get_point_position(p),
        }
    }

    fn get_arc_radius(&self, constraint: &ArcConstraint) -> f64 {
        match constraint {
            ArcConstraint::EntityArc(entity_id) => {
                if let Some(entity) = self.system.entities.get(*entity_id) {
                    entity.parameters[2]
                } else {
                    1.0
                }
            }
            ArcConstraint::CenterRadiusAngles(_, r, _, _) => *r,
        }
    }

    fn get_arc_start_angle(&self, constraint: &ArcConstraint) -> f64 {
        match constraint {
            ArcConstraint::EntityArc(entity_id) => {
                if let Some(entity) = self.system.entities.get(*entity_id) {
                    entity.parameters[3]
                } else {
                    0.0
                }
            }
            ArcConstraint::CenterRadiusAngles(_, _, start, _) => *start,
        }
    }

    fn get_arc_end_angle(&self, constraint: &ArcConstraint) -> f64 {
        match constraint {
            ArcConstraint::EntityArc(entity_id) => {
                if let Some(entity) = self.system.entities.get(*entity_id) {
                    entity.parameters[4]
                } else {
                    std::f64::consts::PI
                }
            }
            ArcConstraint::CenterRadiusAngles(_, _, _, end) => *end,
        }
    }

    fn get_arc_point_at_angle(&self, constraint: &ArcConstraint, angle: f64) -> Vector2D {
        let center = self.get_arc_center(constraint);
        let radius = self.get_arc_radius(constraint);
        center + Vector2D::new(angle.cos(), angle.sin()) * radius
    }

    fn get_line_endpoint(&self, constraint: &LineConstraint, index: usize) -> Vector2D {
        match constraint {
            LineConstraint::EntityLine(entity_id) => {
                if let Some(entity) = self.system.entities.get(*entity_id) {
                    Vector2D::new(entity.parameters[index * 2], entity.parameters[index * 2 + 1])
                } else {
                    Vector2D::new(0.0, 0.0)
                }
            }
            LineConstraint::ThroughPoints(p1, p2) => {
                if index == 0 {
                    self.get_point_position(p1)
                } else {
                    self.get_point_position(p2)
                }
            }
        }
    }

    fn set_jacobian_coincident(&mut self, _row: usize, _p1: &PointConstraint, _p2: &PointConstraint) {}
    fn set_jacobian_horizontal(&mut self, _row: usize, _p1: &PointConstraint, _p2: &PointConstraint) {}
    fn set_jacobian_vertical(&mut self, _row: usize, _p1: &PointConstraint, _p2: &PointConstraint) {}
    fn set_jacobian_parallel(&mut self, _row: usize, _l1: &LineConstraint, _l2: &LineConstraint) {}
    fn set_jacobian_perpendicular(&mut self, _row: usize, _l1: &LineConstraint, _l2: &LineConstraint) {}
    fn set_jacobian_equal_length(&mut self, _row: usize, _l1: &LineConstraint, _l2: &LineConstraint) {}
    fn set_jacobian_equal_radius(&mut self, _row: usize, _c1: &CircleConstraint, _c2: &CircleConstraint) {}
    fn set_jacobian_concentric(&mut self, _row: usize, _c1: &CircleConstraint, _c2: &CircleConstraint) {}
    fn set_jacobian_midpoint(&mut self, _row: usize, _p: &PointConstraint, _l: &LineConstraint) {}
    fn set_jacobian_point_on_line(&mut self, _row: usize, _p: &PointConstraint, _l: &LineConstraint) {}
    fn set_jacobian_point_on_circle(&mut self, _row: usize, _p: &PointConstraint, _c: &CircleConstraint) {}
    fn set_jacobian_angle(&mut self, _row: usize, _l1: &LineConstraint, _l2: &LineConstraint) {}
    fn set_jacobian_collinear(&mut self, _row: usize, _l1: &LineConstraint, _l2: &LineConstraint) {}
    fn set_jacobian_parallel_x(&mut self, _row: usize, _l: &LineConstraint) {}
    fn set_jacobian_parallel_y(&mut self, _row: usize, _l: &LineConstraint) {}
}

#[cfg(not(feature = "constraint"))]
impl ConstraintSolver<'_> {
    fn evaluate_geometric_constraint(&self, row: usize, constraint: &GeometricConstraint) {
        match constraint {
            GeometricConstraint::Coincident(p1, p2) => {
                let pos1 = self.get_point_position(p1);
                let pos2 = self.get_point_position(p2);
                self.residuals[row] = (pos1 - pos2).norm();
            }
            GeometricConstraint::Horizontal(p1, p2) => {
                let pos1 = self.get_point_position(p1);
                let pos2 = self.get_point_position(p2);
                self.residuals[row] = pos2.y - pos1.y;
            }
            GeometricConstraint::Vertical(p1, p2) => {
                let pos1 = self.get_point_position(p1);
                let pos2 = self.get_point_position(p2);
                self.residuals[row] = pos2.x - pos1.x;
            }
            _ => {
                self.residuals[row] = 0.0;
            }
        }
    }
}

    fn evaluate_dimensional_constraint(&self, row: usize, constraint: &DimensionalConstraint) {
        match constraint {
            DimensionalConstraint::Distance(p1, p2, target) => {
                let pos1 = self.get_point_position(p1);
                let pos2 = self.get_point_position(p2);
                let dist = (pos1 - pos2).norm();
                self.residuals[row] = dist - target;
            }
            DimensionalConstraint::Angle(l1, l2, target) => {
                let dir1 = self.get_line_direction(l1);
                let dir2 = self.get_line_direction(l2);
                let angle = dir1.angle_between(dir2);
                self.residuals[row] = angle - target;
            }
            DimensionalConstraint::Radius(c, target) => {
                let radius = self.get_circle_radius(c);
                self.residuals[row] = radius - target;
            }
            DimensionalConstraint::Diameter(c, target) => {
                let radius = self.get_circle_radius(c);
                self.residuals[row] = 2.0 * radius - target;
            }
            DimensionalConstraint::Length(l, target) => {
                let length = self.get_line_length(l);
                self.residuals[row] = length - target;
            }
        }
    }

    fn get_point_position(&self, constraint: &PointConstraint) -> Vector2D {
        match constraint {
            PointConstraint::EntityPoint(entity_id, point_index) => {
                if let Some(entity) = self.system.entities.get(*entity_id) {
                    match entity.entity_type {
                        EntityType::Point => Vector2D::new(entity.parameters[0], entity.parameters[1]),
                        EntityType::Line => {
                            if *point_index == 0 {
                                Vector2D::new(entity.parameters[0], entity.parameters[1])
                            } else {
                                Vector2D::new(entity.parameters[2], entity.parameters[3])
                            }
                        }
                        _ => Vector2D::new(0.0, 0.0),
                    }
                } else {
                    Vector2D::new(0.0, 0.0)
                }
            }
            PointConstraint::FreePoint(id) => Vector2D::new(0.0, 0.0),
        }
    }

    fn get_line_direction(&self, constraint: &LineConstraint) -> Vector2D {
        match constraint {
            LineConstraint::EntityLine(entity_id) => {
                if let Some(entity) = self.system.entities.get(*entity_id) {
                    let dir = Vector2D::new(
                        entity.parameters[2] - entity.parameters[0],
                        entity.parameters[3] - entity.parameters[1],
                    );
                    dir.normalized()
                } else {
                    Vector2D::new(1.0, 0.0)
                }
            }
            LineConstraint::ThroughPoints(p1, p2) => {
                let pos1 = self.get_point_position(p1);
                let pos2 = self.get_point_position(p2);
                let dir = pos2 - pos1;
                dir.normalized()
            }
        }
    }

    fn get_circle_radius(&self, constraint: &CircleConstraint) -> f64 {
        match constraint {
            CircleConstraint::EntityCircle(entity_id) => {
                if let Some(entity) = self.system.entities.get(*entity_id) {
                    entity.parameters[2]
                } else {
                    1.0
                }
            }
            CircleConstraint::CenterRadius(_, radius) => *radius,
        }
    }

    fn get_line_length(&self, constraint: &LineConstraint) -> f64 {
        match constraint {
            LineConstraint::EntityLine(entity_id) => {
                if let Some(entity) = self.system.entities.get(*entity_id) {
                    let dx = entity.parameters[2] - entity.parameters[0];
                    let dy = entity.parameters[3] - entity.parameters[1];
                    (dx * dx + dy * dy).sqrt()
                } else {
                    1.0
                }
            }
            LineConstraint::ThroughPoints(p1, p2) => {
                let pos1 = self.get_point_position(p1);
                let pos2 = self.get_point_position(p2);
                (pos1 - pos2).norm()
            }
        }
    }

    fn solve_system(&mut self) {
        let n = self.jacobian.len();
        let m = self.jacobian[0].len();

        if n == 0 || m == 0 {
            self.delta.clear();
            return;
        }

        let jtj: Vec<Vec<f64>> = (0..m)
            .map(|i| (0..m).map(|j| {
                (0..n).map(|k| self.jacobian[k][i] * self.jacobian[k][j]).sum()
            }).collect();

        let jtr: Vec<f64> = (0..m).map(|i| {
            (0..n).map(|k| self.jacobian[k][i] * self.residuals[k]).sum()
        }).collect();

        self.delta = self.solve_linear_system(&jtj, &jtr);
    }

    fn solve_linear_system(&self, a: &[Vec<f64>], b: &[f64]) -> Vec<f64> {
        let n = a.len();
        if n == 0 {
            return Vec::new();
        }

        let mut aug: Vec<Vec<f64>> = a.iter()
            .enumerate()
            .map(|(i, row)| {
                let mut new_row = row.clone();
                new_row.push(b[i]);
                new_row
            })
            .collect();

        for k in 0..n {
            let pivot = aug[k][k].abs();
            let mut pivot_row = k;
            for i in (k + 1)..n {
                if aug[i][k].abs() > pivot {
                    pivot = aug[i][k].abs();
                    pivot_row = i;
                }
            }

            if pivot < 1e-12 {
                continue;
            }

            if pivot_row != k {
                aug.swap(k, pivot_row);
            }

            for i in (k + 1)..n {
                let factor = aug[i][k] / aug[k][k];
                for j in k..=n {
                    aug[i][j] -= factor * aug[k][j];
                }
            }
        }

        let mut x = vec![0.0; n];
        for i in (0..n).rev() {
            let mut sum = 0.0;
            for j in (i + 1)..n {
                sum += aug[i][j] * x[j];
            }
            if aug[i][i].abs() < 1e-12 {
                x[i] = 0.0;
            } else {
                x[i] = (aug[i][n] - sum) / aug[i][i];
            }
        }

        x
    }

    fn apply_deltas(&mut self) {
        let mut param_index = 0;

        for entity in &mut self.system.entities {
            if entity.is_fixed {
                continue;
            }

            let dof = entity.degrees_of_freedom() as usize;
            if dof > 0 && param_index < self.delta.len() {
                for i in 0..dof {
                    if param_index + i < self.delta.len() {
                        entity.parameters[i] += self.delta[param_index + i];
                    }
                }
                param_index += dof;
            }
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Vector2D {
    pub x: f64,
    pub y: f64,
}

impl Vector2D {
    pub fn new(x: f64, y: f64) -> Self {
        Self { x, y }
    }

    pub fn norm(&self) -> f64 {
        (self.x * self.x + self.y * self.y).sqrt()
    }

    pub fn normalized(&self) -> Self {
        let norm = self.norm();
        if norm < 1e-12 {
            Self { x: 1.0, y: 0.0 }
        } else {
            Self {
                x: self.x / norm,
                y: self.y / norm,
            }
        }
    }

    pub fn dot(&self, other: Vector2D) -> f64 {
        self.x * other.x + self.y * other.y
    }

    pub fn cross(&self, other: Vector2D) -> f64 {
        self.x * other.y - self.y * other.x
    }

    pub fn angle_between(&self, other: Vector2D) -> f64 {
        let dot = self.dot(other);
        let det = self.cross(other);
        det.atan2(dot)
    }
}

impl std::ops::Sub for Vector2D {
    type Output = Self;

    fn sub(self, other: Self) -> Self::Output {
        Self {
            x: self.x - other.x,
            y: self.y - other.y,
        }
    }
}

impl std::ops::Add for Vector2D {
    type Output = Self;

    fn add(self, other: Self) -> Self::Output {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

impl std::ops::AddAssign for Vector2D {
    fn add_assign(&mut self, other: Self) {
        self.x += other.x;
        self.y += other.y;
    }
}

impl std::ops::SubAssign for Vector2D {
    fn sub_assign(&mut self, other: Self) {
        self.x -= other.x;
        self.y -= other.y;
    }
}

impl std::ops::Mul<f64> for Vector2D {
    type Output = Self;

    fn mul(self, scalar: f64) -> Self::Output {
        Self {
            x: self.x * scalar,
            y: self.y * scalar,
        }
    }
}

impl std::ops::Div<f64> for Vector2D {
    type Output = Self;

    fn div(self, scalar: f64) -> Self::Output {
        Self {
            x: self.x / scalar,
            y: self.y / scalar,
        }
    }
}

struct ConstraintGraph {
    nodes: HashSet<usize>,
    edges: HashSet<(usize, usize)>,
}

impl ConstraintGraph {
    pub fn new() -> Self {
        Self {
            nodes: HashSet::new(),
            edges: HashSet::new(),
        }
    }

    pub fn add_node(&mut self, node: usize) {
        self.nodes.insert(node);
    }

    pub fn add_edge(&mut self, node1: usize, node2: usize) {
        if node1 != node2 {
            self.edges.insert((node1.min(node2), node1.max(node2)));
        }
    }

    pub fn get_connected_components(&self) -> Vec<Vec<usize>> {
        let mut visited = HashSet::new();
        let mut components = Vec::new();

        for &node in &self.nodes {
            if !visited.contains(&node) {
                let component = self.bfs(node, &mut visited);
                components.push(component);
            }
        }

        components
    }

    fn bfs(&self, start: usize, visited: &mut HashSet<usize>) -> Vec<usize> {
        let mut component = Vec::new();
        let mut queue = VecDeque::new();

        queue.push_back(start);
        visited.insert(start);

        while let Some(node) = queue.pop_front() {
            component.push(node);

            for &neighbor in self.get_neighbors(node) {
                if !visited.contains(&neighbor) {
                    visited.insert(neighbor);
                    queue.push_back(neighbor);
                }
            }
        }

        component
    }

    fn get_neighbors(&self, node: usize) -> Vec<usize> {
        let mut neighbors = Vec::new();

        for &(a, b) in &self.edges {
            if a == node {
                neighbors.push(b);
            } else if b == node {
                neighbors.push(a);
            }
        }

        neighbors
    }

    pub fn has_cycles(&self) -> bool {
        let components = self.get_connected_components();

        for component in components {
            if self.detect_cycle_in_component(&component) {
                return true;
            }
        }

        false
    }

    fn detect_cycle_in_component(&self, component: &[usize]) -> bool {
        let mut in_degree = HashMap::new();
        let mut adjacency = HashMap::new();

        for &node in component {
            in_degree.insert(node, 0);
            adjacency.insert(node, Vec::new());
        }

        for &(a, b) in &self.edges {
            if component.contains(&a) && component.contains(&b) {
                adjacency.get_mut(&a).unwrap().push(b);
                *in_degree.get_mut(&b).unwrap() += 1;
            }
        }

        let mut queue: VecDeque<usize> = in_degree.iter()
            .filter(|(_, &d)| d == 0)
            .map(|(&node, _)| node)
            .collect();

        let mut visited_count = 0;

        while let Some(node) = queue.pop_front() {
            visited_count += 1;

            for &neighbor in adjacency.get(&node).unwrap() {
                if let Some(&mut deg) = in_degree.get_mut(&neighbor) {
                    deg -= 1;
                    if deg == 0 {
                        queue.push_back(neighbor);
                    }
                }
            }
        }

        visited_count < component.len()
    }
}

#[derive(Debug, Clone)]
pub enum ValidationError {
    InvalidGeometricConstraint(usize),
    InvalidDimensionalConstraint(usize),
    OverConstrained { excess_constraints: usize },
    UnderConstrained { missing_constraints: usize },
    ConflictingConstraints { constraints: Vec<usize> },
}

impl GeometricConstraint {
    pub fn get_constrained_entities(&self) -> Vec<usize> {
        match self {
            GeometricConstraint::Coincident(p1, p2) => {
                let mut entities = Vec::new();
                if let PointConstraint::EntityPoint(id, _) = p1 {
                    entities.push(*id);
                }
                if let PointConstraint::EntityPoint(id, _) = p2 {
                    if !entities.contains(id) {
                        entities.push(*id);
                    }
                }
                entities
            }
            GeometricConstraint::Horizontal(p1, p2) => {
                let mut entities = Vec::new();
                if let PointConstraint::EntityPoint(id, _) = p1 {
                    entities.push(*id);
                }
                if let PointConstraint::EntityPoint(id, _) = p2 {
                    if !entities.contains(id) {
                        entities.push(*id);
                    }
                }
                entities
            }
            GeometricConstraint::Vertical(p1, p2) => {
                let mut entities = Vec::new();
                if let PointConstraint::EntityPoint(id, _) = p1 {
                    entities.push(*id);
                }
                if let PointConstraint::EntityPoint(id, _) = p2 {
                    if !entities.contains(id) {
                        entities.push(*id);
                    }
                }
                entities
            }
            GeometricConstraint::Parallel(l1, l2) => {
                let mut entities = Vec::new();
                if let LineConstraint::EntityLine(id) = l1 {
                    entities.push(*id);
                }
                if let LineConstraint::EntityLine(id) = l2 {
                    if !entities.contains(id) {
                        entities.push(*id);
                    }
                }
                entities
            }
            GeometricConstraint::Perpendicular(l1, l2) => {
                let mut entities = Vec::new();
                if let LineConstraint::EntityLine(id) = l1 {
                    entities.push(*id);
                }
                if let LineConstraint::EntityLine(id) = l2 {
                    if !entities.contains(id) {
                        entities.push(*id);
                    }
                }
                entities
            }
            GeometricConstraint::Tangent(c1, c2) => {
                let mut entities = Vec::new();
                match c1 {
                    CurveConstraint::Line(l) => {
                        if let LineConstraint::EntityLine(id) = l {
                            entities.push(*id);
                        }
                    }
                    CurveConstraint::Circle(c) => {
                        if let CircleConstraint::EntityCircle(id) = c {
                            entities.push(*id);
                        }
                    }
                    CurveConstraint::Arc(a) => {
                        if let ArcConstraint::EntityArc(id) = a {
                            entities.push(*id);
                        }
                    }
                }
                match c2 {
                    CurveConstraint::Line(l) => {
                        if let LineConstraint::EntityLine(id) = l {
                            if !entities.contains(id) {
                                entities.push(*id);
                            }
                        }
                    }
                    CurveConstraint::Circle(c) => {
                        if let CircleConstraint::EntityCircle(id) = c {
                            if !entities.contains(id) {
                                entities.push(*id);
                            }
                        }
                    }
                    CurveConstraint::Arc(a) => {
                        if let ArcConstraint::EntityArc(id) = a {
                            if !entities.contains(id) {
                                entities.push(*id);
                            }
                        }
                    }
                }
                entities
            }
            GeometricConstraint::Concentric(c1, c2) => {
                let mut entities = Vec::new();
                if let CircleConstraint::EntityCircle(id) = c1 {
                    entities.push(*id);
                }
                if let CircleConstraint::EntityCircle(id) = c2 {
                    if !entities.contains(id) {
                        entities.push(*id);
                    }
                }
                entities
            }
            _ => Vec::new(),
        }
    }

    pub fn is_valid(&self) -> bool {
        match self {
            GeometricConstraint::Coincident(p1, p2) => {
                matches!(p1, PointConstraint::EntityPoint(_, _)) &&
                matches!(p2, PointConstraint::EntityPoint(_, _))
            }
            GeometricConstraint::Horizontal(p1, p2) => {
                matches!(p1, PointConstraint::EntityPoint(_, _)) &&
                matches!(p2, PointConstraint::EntityPoint(_, _))
            }
            GeometricConstraint::Vertical(p1, p2) => {
                matches!(p1, PointConstraint::EntityPoint(_, _)) &&
                matches!(p2, PointConstraint::EntityPoint(_, _))
            }
            GeometricConstraint::Parallel(l1, l2) => {
                matches!(l1, LineConstraint::EntityLine(_)) &&
                matches!(l2, LineConstraint::EntityLine(_))
            }
            GeometricConstraint::Perpendicular(l1, l2) => {
                matches!(l1, LineConstraint::EntityLine(_)) &&
                matches!(l2, LineConstraint::EntityLine(_))
            }
            GeometricConstraint::Tangent(c1, c2) => {
                matches!(c1, CurveConstraint::Line(_) | CurveConstraint::Circle(_) | CurveConstraint::Arc(_)) &&
                matches!(c2, CurveConstraint::Line(_) | CurveConstraint::Circle(_) | CurveConstraint::Arc(_))
            }
            GeometricConstraint::Concentric(c1, c2) => {
                matches!(c1, CircleConstraint::EntityCircle(_)) &&
                matches!(c2, CircleConstraint::EntityCircle(_))
            }
            GeometricConstraint::EqualLength(l1, l2) => {
                matches!(l1, LineConstraint::EntityLine(_)) &&
                matches!(l2, LineConstraint::EntityLine(_))
            }
            GeometricConstraint::EqualRadius(c1, c2) => {
                matches!(c1, CircleConstraint::EntityCircle(_)) &&
                matches!(c2, CircleConstraint::EntityCircle(_))
            }
            GeometricConstraint::Midpoint(p, l) => {
                matches!(p, PointConstraint::EntityPoint(_, _)) &&
                matches!(l, LineConstraint::EntityLine(_))
            }
            GeometricConstraint::PointOnLine(p, l) => {
                matches!(p, PointConstraint::EntityPoint(_, _)) &&
                matches!(l, LineConstraint::EntityLine(_))
            }
            GeometricConstraint::PointOnCircle(p, c) => {
                matches!(p, PointConstraint::EntityPoint(_, _)) &&
                matches!(c, CircleConstraint::EntityCircle(_))
            }
            GeometricConstraint::PointOnArc(p, a) => {
                matches!(p, PointConstraint::EntityPoint(_, _)) &&
                matches!(a, ArcConstraint::EntityArc(_))
            }
            GeometricConstraint::Symmetry(p1, p2, l) => {
                matches!(p1, PointConstraint::EntityPoint(_, _)) &&
                matches!(p2, PointConstraint::EntityPoint(_, _)) &&
                matches!(l, LineConstraint::EntityLine(_))
            }
            GeometricConstraint::Angle(l1, l2, angle) => {
                matches!(l1, LineConstraint::EntityLine(_)) &&
                matches!(l2, LineConstraint::EntityLine(_)) &&
                angle.is_finite()
            }
            GeometricConstraint::Collinear(l1, l2) => {
                matches!(l1, LineConstraint::EntityLine(_)) &&
                matches!(l2, LineConstraint::EntityLine(_))
            }
            GeometricConstraint::ParallelX(l) => {
                matches!(l, LineConstraint::EntityLine(_))
            }
            GeometricConstraint::ParallelY(l) => {
                matches!(l, LineConstraint::EntityLine(_))
            }
        }
    }
}

impl DimensionalConstraint {
    pub fn get_constrained_entities(&self) -> Vec<usize> {
        match self {
            DimensionalConstraint::Distance(p1, p2, _) => {
                let mut entities = Vec::new();
                if let PointConstraint::EntityPoint(id, _) = p1 {
                    entities.push(*id);
                }
                if let PointConstraint::EntityPoint(id, _) = p2 {
                    if !entities.contains(id) {
                        entities.push(*id);
                    }
                }
                entities
            }
            DimensionalConstraint::Angle(l1, l2, _) => {
                let mut entities = Vec::new();
                if let LineConstraint::EntityLine(id) = l1 {
                    entities.push(*id);
                }
                if let LineConstraint::EntityLine(id) = l2 {
                    if !entities.contains(id) {
                        entities.push(*id);
                    }
                }
                entities
            }
            DimensionalConstraint::Radius(c, _) => {
                if let CircleConstraint::EntityCircle(id) = c {
                    vec![*id]
                } else {
                    Vec::new()
                }
            }
            DimensionalConstraint::Diameter(c, _) => {
                if let CircleConstraint::EntityCircle(id) = c {
                    vec![*id]
                } else {
                    Vec::new()
                }
            }
            DimensionalConstraint::Length(l, _) => {
                if let LineConstraint::EntityLine(id) = l {
                    vec![*id]
                } else {
                    Vec::new()
                }
            }
        }
    }

    pub fn is_valid(&self) -> bool {
        match self {
            DimensionalConstraint::Distance(p1, p2, dist) => {
                matches!(p1, PointConstraint::EntityPoint(_, _)) &&
                matches!(p2, PointConstraint::EntityPoint(_, _)) &&
                dist.is_finite() && *dist >= 0.0
            }
            DimensionalConstraint::Angle(l1, l2, angle) => {
                matches!(l1, LineConstraint::EntityLine(_)) &&
                matches!(l2, LineConstraint::EntityLine(_)) &&
                angle.is_finite()
            }
            DimensionalConstraint::Radius(_, radius) => {
                radius.is_finite() && *radius >= 0.0
            }
            DimensionalConstraint::Diameter(_, diameter) => {
                diameter.is_finite() && *diameter >= 0.0
            }
            DimensionalConstraint::Length(_, length) => {
                length.is_finite() && *length >= 0.0
            }
        }
    }
}

pub struct ConstraintBuilder;

impl ConstraintBuilder {
    pub fn coincident(
        system: &mut ConstraintSystem,
        entity1_id: usize,
        point1_index: usize,
        entity2_id: usize,
        point2_index: usize,
    ) {
        let constraint = GeometricConstraint::Coincident(
            PointConstraint::EntityPoint(entity1_id, point1_index),
            PointConstraint::EntityPoint(entity2_id, point2_index),
        );
        system.add_geometric_constraint(constraint);
    }

    pub fn horizontal(
        system: &mut ConstraintSystem,
        entity1_id: usize,
        point1_index: usize,
        entity2_id: usize,
        point2_index: usize,
    ) {
        let constraint = GeometricConstraint::Horizontal(
            PointConstraint::EntityPoint(entity1_id, point1_index),
            PointConstraint::EntityPoint(entity2_id, point2_index),
        );
        system.add_geometric_constraint(constraint);
    }

    pub fn vertical(
        system: &mut ConstraintSystem,
        entity1_id: usize,
        point1_index: usize,
        entity2_id: usize,
        point2_index: usize,
    ) {
        let constraint = GeometricConstraint::Vertical(
            PointConstraint::EntityPoint(entity1_id, point1_index),
            PointConstraint::EntityPoint(entity2_id, point2_index),
        );
        system.add_geometric_constraint(constraint);
    }

    pub fn parallel(
        system: &mut ConstraintSystem,
        line1_id: usize,
        line2_id: usize,
    ) {
        let constraint = GeometricConstraint::Parallel(
            LineConstraint::EntityLine(line1_id),
            LineConstraint::EntityLine(line2_id),
        );
        system.add_geometric_constraint(constraint);
    }

    pub fn perpendicular(
        system: &mut ConstraintSystem,
        line1_id: usize,
        line2_id: usize,
    ) {
        let constraint = GeometricConstraint::Perpendicular(
            LineConstraint::EntityLine(line1_id),
            LineConstraint::EntityLine(line2_id),
        );
        system.add_geometric_constraint(constraint);
    }

    pub fn tangent(
        system: &mut ConstraintSystem,
        curve1_id: usize,
        curve1_type: CurveType,
        curve2_id: usize,
        curve2_type: CurveType,
    ) {
        let c1 = match curve1_type {
            CurveType::Line => CurveConstraint::Line(LineConstraint::EntityLine(curve1_id)),
            CurveType::Circle => CurveConstraint::Circle(CircleConstraint::EntityCircle(curve1_id)),
            CurveType::Arc => CurveConstraint::Arc(ArcConstraint::EntityArc(curve1_id)),
        };
        
        let c2 = match curve2_type {
            CurveType::Line => CurveConstraint::Line(LineConstraint::EntityLine(curve2_id)),
            CurveType::Circle => CurveConstraint::Circle(CircleConstraint::EntityCircle(curve2_id)),
            CurveType::Arc => CurveConstraint::Arc(ArcConstraint::EntityArc(curve2_id)),
        };

        let constraint = GeometricConstraint::Tangent(c1, c2);
        system.add_geometric_constraint(constraint);
    }

    pub fn concentric(
        system: &mut ConstraintSystem,
        circle1_id: usize,
        circle2_id: usize,
    ) {
        let constraint = GeometricConstraint::Concentric(
            CircleConstraint::EntityCircle(circle1_id),
            CircleConstraint::EntityCircle(circle2_id),
        );
        system.add_geometric_constraint(constraint);
    }

    pub fn distance(
        system: &mut ConstraintSystem,
        entity1_id: usize,
        point1_index: usize,
        entity2_id: usize,
        point2_index: usize,
        target_distance: f64,
    ) {
        let constraint = DimensionalConstraint::Distance(
            PointConstraint::EntityPoint(entity1_id, point1_index),
            PointConstraint::EntityPoint(entity2_id, point2_index),
            target_distance,
        );
        system.add_dimensional_constraint(constraint);
    }

    pub fn angle(
        system: &mut ConstraintSystem,
        line1_id: usize,
        line2_id: usize,
        target_angle: f64,
    ) {
        let constraint = DimensionalConstraint::Angle(
            LineConstraint::EntityLine(line1_id),
            LineConstraint::EntityLine(line2_id),
            target_angle,
        );
        system.add_dimensional_constraint(constraint);
    }

    pub fn radius(
        system: &mut ConstraintSystem,
        circle_id: usize,
        target_radius: f64,
    ) {
        let constraint = DimensionalConstraint::Radius(
            CircleConstraint::EntityCircle(circle_id),
            target_radius,
        );
        system.add_dimensional_constraint(constraint);
    }

    pub fn diameter(
        system: &mut ConstraintSystem,
        circle_id: usize,
        target_diameter: f64,
    ) {
        let constraint = DimensionalConstraint::Diameter(
            CircleConstraint::EntityCircle(circle_id),
            target_diameter,
        );
        system.add_dimensional_constraint(constraint);
    }

    pub fn length(
        system: &mut ConstraintSystem,
        line_id: usize,
        target_length: f64,
    ) {
        let constraint = DimensionalConstraint::Length(
            LineConstraint::EntityLine(line_id),
            target_length,
        );
        system.add_dimensional_constraint(constraint);
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum CurveType {
    Line,
    Circle,
    Arc,
}
