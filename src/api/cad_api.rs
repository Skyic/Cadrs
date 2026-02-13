use crate::data_structure::{Document, Entity, ObjectId, Layer, Block, EntityType, EntityGeometry};
use crate::geometry::{Point, Vector2, Line, Circle, Arc, Ellipse, Polyline, BSpline, NURBS};
use crate::dimension::{DimensionStyle, LinearDimension, AngularDimension, RadialDimension, OrdinateDimension};
use crate::constraint::{ConstraintSolver, GeometricConstraint, PointConstraint, LineConstraint};
use crate::api::error::{CADError, CADResult};
use thiserror::Error;
use std::collections::HashMap;

#[derive(Debug, Error)]
pub enum APIError {
    #[error("文档未打开")]
    DocumentNotOpen,
    
    #[error("无效的选择: {description}")]
    InvalidSelection { description: String },
    
    #[error("操作失败: {message}")]
    OperationFailed { message: String },
    
    #[error("约束求解失败: {message}")]
    ConstraintSolverFailed { message: String },
}

pub struct CADAPI {
    document: Option<Document>,
    command_processor: CommandProcessor,
    undo_manager: UndoManager,
    selection_manager: SelectionManager,
    snap_manager: SnapManager,
    current_layer: Option<ObjectId>,
    current_style: DimensionStyle,
}

impl Default for CADAPI {
    fn default() -> Self {
        Self {
            document: None,
            command_processor: CommandProcessor::new(),
            undo_manager: UndoManager::new(),
            selection_manager: SelectionManager::new(),
            snap_manager: SnapManager::new(),
            current_layer: None,
            current_style: DimensionStyle::default(),
        }
    }
}

impl CADAPI {
    pub fn new() -> Self {
        Self::default()
    }
    
    pub fn create_document(&mut self, name: &str) {
        self.document = Some(Document::new(name));
        self.current_layer = Some(self.document.as_ref().unwrap().layers[0].id);
    }
    
    pub fn open_document(&mut self, _filename: &str) -> CADResult<()> {
        self.document = Some(Document::new("Untitled"));
        Ok(())
    }
    
    pub fn save_document(&self, filename: &str) -> CADResult<()> {
        if self.document.is_none() {
            return Err(Box::new(APIError::DocumentNotOpen));
        }
        Ok(())
    }
    
    pub fn close_document(&mut self) {
        self.document = None;
    }
    
    pub fn current_document(&self) -> Option<&Document> {
        self.document.as_ref()
    }
    
    pub fn current_document_mut(&mut self) -> Option<&mut Document> {
        self.document.as_mut()
    }
    
    pub fn add_line(&mut self, start: Point, end: Point) -> CADResult<ObjectId> {
        self.begin_transaction("Add Line");
        
        let line = Line::new(start, end);
        let entity = Entity::new(EntityType::Line, EntityGeometry::Line(line));
        
        let id = entity.id;
        
        if let Some(doc) = &mut self.document {
            doc.add_entity(entity);
        }
        
        self.commit_transaction();
        Ok(id)
    }
    
    pub fn add_circle(&mut self, center: Point, radius: f64) -> CADResult<ObjectId> {
        self.begin_transaction("Add Circle");
        
        let circle = Circle::new(center, radius);
        let entity = Entity::new(EntityType::Circle, EntityGeometry::Circle(circle));
        
        let id = entity.id;
        
        if let Some(doc) = &mut self.document {
            doc.add_entity(entity);
        }
        
        self.commit_transaction();
        Ok(id)
    }
    
    pub fn add_arc(&mut self, center: Point, radius: f64, start_angle: f64, end_angle: f64) -> CADResult<ObjectId> {
        self.begin_transaction("Add Arc");
        
        let arc = Arc::new(center, radius, start_angle, end_angle);
        let entity = Entity::new(EntityType::Arc, EntityGeometry::Arc(arc));
        
        let id = entity.id;
        
        if let Some(doc) = &mut self.document {
            doc.add_entity(entity);
        }
        
        self.commit_transaction();
        Ok(id)
    }
    
    pub fn add_arc_3_point(&mut self, start: Point, end: Point, through: Point) -> CADResult<ObjectId> {
        let center = self.calculate_arc_center(start, through, end)?;
        let radius = center.distance_to(&start);
        let start_angle = (start - center).to_vector2().angle();
        let end_angle = (end - center).to_vector2().angle();
        
        self.add_arc(center, radius, start_angle, end_angle)
    }
    
    fn calculate_arc_center(&self, p1: Point, p2: Point, p3: Point) -> CADResult<Point> {
        let mid1 = p1.midpoint(&p2);
        let mid2 = p2.midpoint(&p3);
        
        let dir1 = (p2 - p1).to_vector2().normalize();
        let dir2 = (p3 - p2).to_vector2().normalize();
        
        let normal1 = Vector2::new(-dir1.y, dir1.x);
        let normal2 = Vector2::new(-dir2.y, dir2.x);
        
        let denom = normal1.x * normal2.y - normal1.y * normal2.x;
        if denom.abs() < 1e-10 {
            return Err(Box::new(APIError::OperationFailed {
                message: "Points are collinear".to_string(),
            }));
        }
        
        let t = ((mid2.x - mid1.x) * normal1.y - (mid2.y - mid1.y) * normal1.x) / denom;
        
        let center = Point::new(
            mid1.x + normal1.x * t,
            mid1.y + normal1.y * t,
            0.0,
        );
        
        Ok(center)
    }
    
    pub fn add_ellipse(&mut self, center: Point, semi_major: f64, semi_minor: f64, rotation: f64) -> CADResult<ObjectId> {
        self.begin_transaction("Add Ellipse");
        
        let ellipse = Ellipse::new(center, semi_major, semi_minor, rotation);
        let entity = Entity::new(EntityType::Ellipse, EntityGeometry::Ellipse(ellipse));
        
        let id = entity.id;
        
        if let Some(doc) = &mut self.document {
            doc.add_entity(entity);
        }
        
        self.commit_transaction();
        Ok(id)
    }
    
    pub fn add_polyline(&mut self, vertices: Vec<Point>) -> CADResult<ObjectId> {
        self.begin_transaction("Add Polyline");
        
        let mut polyline = Polyline::new();
        for vertex in &vertices {
            polyline.push(*vertex);
        }
        
        let entity = Entity::new(EntityType::Polyline, EntityGeometry::Polyline(polyline));
        
        let id = entity.id;
        
        if let Some(doc) = &mut self.document {
            doc.add_entity(entity);
        }
        
        self.commit_transaction();
        Ok(id)
    }
    
    pub fn add_bspline(&mut self, control_points: Vec<Point>, degree: usize) -> CADResult<ObjectId> {
        self.begin_transaction("Add BSpline");
        
        let spline = BSpline::from_points(control_points, degree);
        let entity = Entity::new(EntityType::BSpline, EntityGeometry::BSpline(spline));
        
        let id = entity.id;
        
        if let Some(doc) = &mut self.document {
            doc.add_entity(entity);
        }
        
        self.commit_transaction();
        Ok(id)
    }
    
    pub fn add_linear_dimension(&mut self, p1: Point, p2: Point, location: Option<Point>) -> CADResult<ObjectId> {
        self.begin_transaction("Add Linear Dimension");
        
        let definition_line = Line::new(p1, p2);
        let dimension = LinearDimension::new(definition_line, self.current_style.clone(), location);
        let entity = Entity::new(EntityType::Dimension, EntityGeometry::Dimension(dimension));
        
        let id = entity.id;
        
        if let Some(doc) = &mut self.document {
            doc.add_entity(entity);
        }
        
        self.commit_transaction();
        Ok(id)
    }
    
    pub fn add_aligned_dimension(&mut self, p1: Point, p2: Point, location: Option<Point>) -> CADResult<ObjectId> {
        self.begin_transaction("Add Aligned Dimension");
        
        let dimension = AlignedDimension::new(p1, p2, self.current_style.clone(), location);
        let entity = Entity::new(EntityType::Dimension, EntityGeometry::Dimension(dimension));
        
        let id = entity.id;
        
        if let Some(doc) = &mut self.document {
            doc.add_entity(entity);
        }
        
        self.commit_transaction();
        Ok(id)
    }
    
    pub fn add_radial_dimension(&mut self, center: Point, radius_point: Point, is_diameter: bool) -> CADResult<ObjectId> {
        self.begin_transaction("Add Radial Dimension");
        
        let circle = Circle::new(center, center.distance_to(&radius_point));
        let dimension = RadialDimension::new_diameter(circle, radius_point, Point::new(center.x - radius_point.x, center.y - radius_point.y, 0.0), self.current_style.clone());
        let entity = Entity::new(EntityType::Dimension, EntityGeometry::Dimension(dimension));
        
        let id = entity.id;
        
        if let Some(doc) = &mut self.document {
            doc.add_entity(entity);
        }
        
        self.commit_transaction();
        Ok(id)
    }
    
    pub fn delete_entity(&mut self, id: ObjectId) -> CADResult<()> {
        self.begin_transaction("Delete Entity");
        
        if let Some(doc) = &mut self.document {
            doc.remove_entity(&id);
        }
        
        self.selection_manager.clear();
        
        self.commit_transaction();
        Ok(())
    }
    
    pub fn move_entities(&mut self, entities: &[ObjectId], delta: Vector2) -> CADResult<()> {
        self.begin_transaction("Move Entities");
        
        for id in entities {
            if let Some(doc) = &mut self.document {
                if let Some(entity) = doc.get_entity_mut(id) {
                    self.move_entity(entity, delta);
                }
            }
        }
        
        self.commit_transaction();
        Ok(())
    }
    
    fn move_entity(&self, entity: &mut Entity, delta: Vector2) {
        match &mut entity.geometry {
            EntityGeometry::Line(line) => {
                line.start = Point::new(line.start.x + delta.x, line.start.y + delta.y, 0.0);
                line.end = Point::new(line.end.x + delta.x, line.end.y + delta.y, 0.0);
            }
            EntityGeometry::Circle(circle) => {
                circle.center = Point::new(circle.center.x + delta.x, circle.center.y + delta.y, 0.0);
            }
            EntityGeometry::Arc(arc) => {
                arc.center = Point::new(arc.center.x + delta.x, arc.center.y + delta.y, 0.0);
            }
            EntityGeometry::Polyline(polyline) => {
                for vertex in &mut polyline.vertices {
                    vertex.x += delta.x;
                    vertex.y += delta.y;
                }
            }
            _ => {}
        }
    }
    
    pub fn rotate_entities(&mut self, entities: &[ObjectId], center: Point, angle: f64) -> CADResult<()> {
        self.begin_transaction("Rotate Entities");
        
        let cos_a = angle.cos();
        let sin_a = angle.sin();
        
        for id in entities {
            if let Some(doc) = &mut self.document {
                if let Some(entity) = doc.get_entity_mut(id) {
                    self.rotate_entity(entity, center, cos_a, sin_a);
                }
            }
        }
        
        self.commit_transaction();
        Ok(())
    }
    
    fn rotate_entity(&self, entity: &mut Entity, center: Point, cos_a: f64, sin_a: f64) {
        match &mut entity.geometry {
            EntityGeometry::Line(line) => {
                line.start = self.rotate_point(line.start, center, cos_a, sin_a);
                line.end = self.rotate_point(line.end, center, cos_a, sin_a);
            }
            EntityGeometry::Circle(circle) => {
                circle.center = self.rotate_point(circle.center, center, cos_a, sin_a);
            }
            EntityGeometry::Arc(arc) => {
                arc.center = self.rotate_point(arc.center, center, cos_a, sin_a);
            }
            EntityGeometry::Polyline(polyline) => {
                for vertex in &mut polyline.vertices {
                    *vertex = self.rotate_point(*vertex, center, cos_a, sin_a);
                }
            }
            _ => {}
        }
    }
    
    fn rotate_point(&self, point: Point, center: Point, cos_a: f64, sin_a: f64) -> Point {
        let dx = point.x - center.x;
        let dy = point.y - center.y;
        
        Point::new(
            center.x + dx * cos_a - dy * sin_a,
            center.y + dx * sin_a + dy * cos_a,
            0.0,
        )
    }
    
    pub fn scale_entities(&mut self, entities: &[ObjectId], center: Point, factor: f64) -> CADResult<()> {
        self.begin_transaction("Scale Entities");
        
        for id in entities {
            if let Some(doc) = &mut self.document {
                if let Some(entity) = doc.get_entity_mut(id) {
                    self.scale_entity(entity, center, factor);
                }
            }
        }
        
        self.commit_transaction();
        Ok(())
    }
    
    fn scale_entity(&self, entity: &mut Entity, center: Point, factor: f64) {
        match &mut entity.geometry {
            EntityGeometry::Line(line) => {
                line.start = self.scale_point(line.start, center, factor);
                line.end = self.scale_point(line.end, center, factor);
            }
            EntityGeometry::Circle(circle) => {
                circle.center = self.scale_point(circle.center, center, factor);
                circle.radius *= factor;
            }
            EntityGeometry::Arc(arc) => {
                arc.center = self.scale_point(arc.center, center, factor);
                arc.radius *= factor;
            }
            EntityGeometry::Ellipse(ellipse) => {
                ellipse.center = self.scale_point(ellipse.center, center, factor);
                ellipse.semi_major *= factor;
                ellipse.semi_minor *= factor;
            }
            EntityGeometry::Polyline(polyline) => {
                for vertex in &mut polyline.vertices {
                    *vertex = self.scale_point(*vertex, center, factor);
                }
            }
            _ => {}
        }
    }
    
    fn scale_point(&self, point: Point, center: Point, factor: f64) -> Point {
        Point::new(
            center.x + (point.x - center.x) * factor,
            center.y + (point.y - center.y) * factor,
            0.0,
        )
    }
    
    pub fn mirror_entities(&mut self, entities: &[ObjectId], axis_start: Point, axis_end: Point) -> CADResult<()> {
        self.begin_transaction("Mirror Entities");
        
        let axis_line = Line::new(axis_start, axis_end);
        let direction = axis_line.direction();
        let normal = Vector2::new(-direction.y, direction.x);
        
        for id in entities {
            if let Some(doc) = &mut self.document {
                if let Some(entity) = doc.get_entity_mut(id) {
                    self.mirror_entity(entity, axis_start, normal);
                }
            }
        }
        
        self.commit_transaction();
        Ok(())
    }
    
    fn mirror_entity(&self, entity: &mut Entity, axis_point: Point, normal: Vector2) {
        match &mut entity.geometry {
            EntityGeometry::Line(line) => {
                line.start = self.mirror_point(line.start, axis_point, normal);
                line.end = self.mirror_point(line.end, axis_point, normal);
            }
            EntityGeometry::Circle(circle) => {
                circle.center = self.mirror_point(circle.center, axis_point, normal);
            }
            EntityGeometry::Arc(arc) => {
                arc.center = self.mirror_point(arc.center, axis_point, normal);
            }
            EntityGeometry::Polyline(polyline) => {
                for vertex in &mut polyline.vertices {
                    *vertex = self.mirror_point(*vertex, axis_point, normal);
                }
            }
            _ => {}
        }
    }
    
    fn mirror_point(&self, point: Point, axis_point: Point, normal: Vector2) -> Point {
        let to_point = point - axis_point;
        let dot = to_point.x * normal.x + to_point.y * normal.y;
        
        Point::new(
            point.x - 2.0 * dot * normal.x,
            point.y - 2.0 * dot * normal.y,
            0.0,
        )
    }
    
    pub fn add_constraint(&mut self, constraint: GeometricConstraint) -> CADResult<()> {
        self.begin_transaction("Add Constraint");
        
        let entities = constraint.get_entities();
        for id in entities {
            if let Some(doc) = &mut self.document {
                if let Some(entity) = doc.get_entity_mut(&id) {
                    entity.add_constraint(constraint.clone());
                }
            }
        }
        
        self.commit_transaction();
        Ok(())
    }
    
    pub fn solve_constraints(&mut self) -> CADResult<()> {
        self.begin_transaction("Solve Constraints");
        
        let mut solver = ConstraintSolver::new();
        
        if let Some(doc) = &mut self.document {
            let entities = doc.get_all_entities();
            for entity in entities {
                for constraint in entity.constraints() {
                    solver.add_constraint(constraint.clone());
                }
            }
            
            if let Err(e) = solver.solve() {
                return Err(Box::new(APIError::ConstraintSolverFailed {
                    message: e.to_string(),
                }));
            }
        }
        
        self.commit_transaction();
        Ok(())
    }
    
    pub fn get_distance(&self, point1: Point, point2: Point) -> f64 {
        point1.distance_to(&point2)
    }
    
    pub fn get_angle(&self, vertex: Point, point1: Point, point2: Point) -> f64 {
        let v1 = (point1 - vertex).to_vector2();
        let v2 = (point2 - vertex).to_vector2();
        
        let dot = v1.x * v2.x + v1.y * v2.y;
        let cross = v1.x * v2.y - v1.y * v2.x;
        
        cross.atan2(dot).abs()
    }
    
    pub fn snap_to_grid(&self, point: Point, grid_size: f64) -> Point {
        let x = (point.x / grid_size).round() * grid_size;
        let y = (point.y / grid_size).round() * grid_size;
        Point::new(x, y, 0.0)
    }
    
    pub fn undo(&mut self) {
        self.undo_manager.undo();
    }
    
    pub fn redo(&mut self) {
        self.undo_manager.redo();
    }
    
    pub fn begin_transaction(&mut self, name: &str) {
        self.undo_manager.begin_transaction(name.to_string());
    }
    
    pub fn commit_transaction(&mut self) {
        self.undo_manager.commit_transaction();
    }
    
    pub fn abort_transaction(&mut self) {
        self.undo_manager.abort_transaction();
    }
    
    pub fn select(&mut self, point: Point) -> CADResult<Vec<ObjectId>> {
        let entities = self.selection_manager.select_point(point, &self.document);
        Ok(entities)
    }
    
    pub fn select_box(&mut self, corner1: Point, corner2: Point) -> CADResult<Vec<ObjectId>> {
        let entities = self.selection_manager.select_box(corner1, corner2, &self.document);
        Ok(entities)
    }
    
    pub fn clear_selection(&mut self) {
        self.selection_manager.clear();
    }
    
    pub fn selected_entities(&self) -> Vec<&Entity> {
        let ids = self.selection_manager.get_selection();
        if let Some(doc) = &self.document {
            ids.iter()
                .filter_map(|id| doc.get_entity(id))
                .collect()
        } else {
            Vec::new()
        }
    }
}

pub struct CommandProcessor {
    commands: HashMap<String, Box<dyn Command>>,
}

impl CommandProcessor {
    pub fn new() -> Self {
        Self {
            commands: HashMap::new(),
        }
    }
    
    pub fn register_command<C: Command + 'static>(&mut self, name: &str, command: C) {
        self.commands.insert(name.to_string(), Box::new(command));
    }
    
    pub fn execute(&self, name: &str, args: &[Box<dyn std::any::Any>]) -> Result<Box<dyn std::any::Any>, String> {
        if let Some(command) = self.commands.get(name) {
            command.execute(args)
        } else {
            Err(format!("Command '{}' not found", name))
        }
    }
}

pub trait Command {
    fn name(&self) -> &str;
    fn execute(&self, args: &[Box<dyn std::any::Any>]) -> Result<Box<dyn std::any::Any>, String>;
}

pub struct UndoManager {
    transactions: Vec<Transaction>,
    current_index: usize,
    max_history: usize,
}

impl UndoManager {
    pub fn new() -> Self {
        Self {
            transactions: Vec::new(),
            current_index: 0,
            max_history: 100,
        }
    }
    
    pub fn begin_transaction(&mut self, name: String) {
        if self.current_index < self.transactions.len() {
            self.transactions.truncate(self.current_index);
        }
        
        self.transactions.push(Transaction::new(name));
        self.current_index = self.transactions.len();
    }
    
    pub fn commit_transaction(&mut self) {
        if let Some(transaction) = self.transactions.last_mut() {
            transaction.commit();
        }
    }
    
    pub fn abort_transaction(&mut self) {
        self.transactions.pop();
        self.current_index = self.transactions.len();
    }
    
    pub fn undo(&mut self) {
        if self.current_index > 0 {
            self.current_index -= 1;
            if let Some(transaction) = self.transactions.get(self.current_index) {
                transaction.undo();
            }
        }
    }
    
    pub fn redo(&mut self) {
        if self.current_index < self.transactions.len() {
            if let Some(transaction) = self.transactions.get(self.current_index) {
                transaction.redo();
            }
            self.current_index += 1;
        }
    }
}

pub struct Transaction {
    name: String,
    operations: Vec<Operation>,
    committed: bool,
}

impl Transaction {
    pub fn new(name: String) -> Self {
        Self {
            name,
            operations: Vec::new(),
            committed: false,
        }
    }
    
    pub fn add_operation(&mut self, operation: Operation) {
        self.operations.push(operation);
    }
    
    pub fn commit(&mut self) {
        self.committed = true;
    }
    
    pub fn undo(&self) {
        for operation in self.operations.iter().rev() {
            operation.undo();
        }
    }
    
    pub fn redo(&self) {
        for operation in self.operations.iter() {
            operation.redo();
        }
    }
}

pub struct Operation {
    forward: Box<dyn FnMut()>,
    backward: Box<dyn FnMut()>,
}

impl Operation {
    pub fn new<F, B>(forward: F, backward: B) -> Self
    where
        F: FnMut() + 'static,
        B: FnMut() + 'static,
    {
        Self {
            forward: Box::new(forward),
            backward: Box::new(backward),
        }
    }
    
    pub fn undo(&mut self) {
        (self.backward)();
    }
    
    pub fn redo(&mut self) {
        (self.forward)();
    }
}

pub struct SelectionManager {
    selection: Vec<ObjectId>,
    selection_mode: SelectionMode,
}

impl SelectionManager {
    pub fn new() -> Self {
        Self {
            selection: Vec::new(),
            selection_mode: SelectionMode::Point,
        }
    }
    
    pub fn select_point(&mut self, point: Point, _document: &Option<Document>) -> Vec<ObjectId> {
        self.selection.clear();
        self.selection
    }
    
    pub fn select_box(&mut self, corner1: Point, corner2: Point, _document: &Option<Document>) -> Vec<ObjectId> {
        self.selection.clear();
        self.selection
    }
    
    pub fn add_to_selection(&mut self, id: ObjectId) {
        if !self.selection.contains(&id) {
            self.selection.push(id);
        }
    }
    
    pub fn remove_from_selection(&mut self, id: &ObjectId) {
        self.selection.retain(|x| x != id);
    }
    
    pub fn clear(&mut self) {
        self.selection.clear();
    }
    
    pub fn get_selection(&self) -> Vec<ObjectId> {
        self.selection.clone()
    }
    
    pub fn set_selection_mode(&mut self, mode: SelectionMode) {
        self.selection_mode = mode;
    }
}

#[derive(Clone, Copy, PartialEq)]
pub enum SelectionMode {
    Point,
    Window,
    Crossing,
    All,
    Fence,
}

pub struct SnapManager {
    snap_modes: Vec<SnapMode>,
    snap_point: Option<Point>,
    aperture_size: usize,
    grid_snap: bool,
    grid_spacing: f64,
}

impl SnapManager {
    pub fn new() -> Self {
        Self {
            snap_modes: Vec::new(),
            snap_point: None,
            aperture_size: 10,
            grid_snap: false,
            grid_spacing: 1.0,
        }
    }
    
    pub fn enable_snap_mode(&mut self, mode: SnapMode) {
        if !self.snap_modes.contains(&mode) {
            self.snap_modes.push(mode);
        }
    }
    
    pub fn disable_snap_mode(&mut self, mode: &SnapMode) {
        self.snap_modes.retain(|m| m != mode);
    }
    
    pub fn set_grid_snap(&mut self, enabled: bool, spacing: f64) {
        self.grid_snap = enabled;
        self.grid_spacing = spacing;
    }
    
    pub fn snap(&mut self, point: Point, entities: &[Entity]) -> Point {
        self.snap_point = None;
        
        if self.grid_snap {
            let snapped = Point::new(
                (point.x / self.grid_spacing).round() * self.grid_spacing,
                (point.y / self.grid_spacing).round() * self.grid_spacing,
                0.0,
            );
            return snapped;
        }
        
        point
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum SnapMode {
    Endpoint,
    Midpoint,
    Center,
    Intersection,
    Perpendicular,
    Tangent,
    Nearest,
    Grid,
    Node,
    Extension,
}
