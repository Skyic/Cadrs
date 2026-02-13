# Cadrs CAD SDK API Documentation

This document provides detailed API documentation for the Cadrs CAD SDK.

## Core Modules

### Data Structure Module

The data_structure module provides the foundational data types and structures for CAD document management.

#### Document

```rust
pub struct Document {
    layers: LayerManager,
    blocks: BlockManager,
    entities: EntityContainer,
    current_layer: Option<LayerId>,
}
```

**Methods:**

- `new() -> Document` - Creates a new empty document
- `add_entity<E: Into<Entity>>(&mut self, entity: E) -> Result<EntityId, CadrsError>` - Adds an entity to the document
- `remove_entity(&mut self, id: EntityId) -> Result<(), CadrsError>` - Removes an entity from the document
- `get_entity(&self, id: EntityId) -> Option<&Entity>` - Retrieves an entity by ID
- `get_entity_mut(&mut self, id: EntityId) -> Option<&mut Entity>` - Retrieves a mutable reference to an entity
- `set_current_layer(&mut self, layer: &str) -> Result<(), CadrsError>` - Sets the active layer
- `get_current_layer(&self) -> Option<&Layer>` - Returns the current active layer
- `add_layer(&mut self, layer: Layer) -> Result<LayerId, CadrsError>` - Adds a new layer
- `add_block(&mut self, block: Block) -> Result<BlockId, CadrsError>` - Adds a new block definition

#### Entity

```rust
pub struct Entity {
    id: EntityId,
    layer: LayerId,
    visibility: Visibility,
    line_type: Option<LineTypeId>,
    properties: EntityProperties,
}
```

**Methods:**

- `id(&self) -> EntityId` - Returns the unique entity identifier
- `layer(&self) -> LayerId` - Returns the layer ID
- `set_layer(&mut self, layer: LayerId)` - Sets the entity layer
- `visibility(&self) -> Visibility` - Returns the visibility status
- `set_visibility(&mut self, visibility: Visibility)` - Sets the visibility
- `transform(&mut self, transform: &Transformation2D)` - Applies a 2D transformation
- `bounds(&self) -> Option<BoundingBox>` - Returns the bounding box

#### EntityId

```rust
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct EntityId(u64);
```

**Methods:**

- `new() -> Self` - Creates a new unique ID
- `as_u64(&self) -> u64` - Returns the raw u64 value

#### Layer

```rust
pub struct Layer {
    name: String,
    color: Color,
    visibility: Visibility,
    line_type: Option<LineTypeId>,
    plot_style: Option<PlotStyleId>,
    is_locked: bool,
    is_frozen: bool,
}
```

**Methods:**

- `new(name: &str) -> Self` - Creates a new layer with the given name
- `name(&self) -> &str` - Returns the layer name
- `color(&self) -> Color` - Returns the layer color
- `set_color(&mut self, color: Color)` - Sets the layer color
- `is_locked(&self) -> bool` - Checks if the layer is locked
- `set_locked(&mut self, locked: bool)` - Sets the locked state

#### Block

```rust
pub struct Block {
    name: String,
    entities: Vec<Entity>,
    origin: Point,
    attributes: Vec<BlockAttribute>,
}
```

**Methods:**

- `new(name: &str) -> Self` - Creates a new block
- `name(&self) -> &str` - Returns the block name
- `add_entity<E: Into<Entity>>(&mut self, entity: E)` - Adds an entity to the block
- `origin(&self) -> Point` - Returns the block origin
- `set_origin(&mut self, origin: Point)` - Sets the block origin
- `count(&self) -> usize` - Returns the number of entities

#### Selection

```rust
pub struct Selection {
    selected: BTreeSet<EntityId>,
    highlighted: BTreeSet<EntityId>,
}
```

**Methods:**

- `new() -> Self` - Creates an empty selection
- `select(&mut self, id: EntityId)` - Adds an entity to selection
- `deselect(&mut self, id: EntityId)` - Removes an entity from selection
- `clear(&mut self)` - Clears all selections
- `is_selected(&self, id: EntityId) -> bool` - Checks if an entity is selected
- `selected_ids(&self) -> &BTreeSet<EntityId>` - Returns all selected IDs
- `toggle(&mut self, id: EntityId)` - Toggles selection state

#### EventSystem

```rust
pub struct EventSystem {
    listeners: HashMap<EventType, Vec<Box<dyn EventListener>>>,
}
```

**Methods:**

- `new() -> Self` - Creates a new event system
- `subscribe<E: Event>(&mut self, event_type: E::EventType, listener: impl EventListener)` - Registers an event listener
- `publish<E: Event>(&self, event: E)` - Publishes an event to all listeners
- `unsubscribe(&mut self, listener_id: ListenerId)` - Removes a listener

### Geometry Module

The geometry module provides fundamental geometric primitives and operations.

#### Point

```rust
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Point {
    x: f64,
    y: f64,
}
```

**Methods:**

- `new(x: f64, y: f64) -> Self` - Creates a new point
- `origin() -> Self` - Returns the origin point (0, 0)
- `x(&self) -> f64` - Returns the x coordinate
- `y(&self) -> f64` - Returns the y coordinate
- `distance_to(&self, other: Point) -> f64` - Calculates distance to another point
- `midpoint(&self, other: Point) -> Point` - Returns the midpoint
- `lerp(&self, other: Point, t: f64) -> Point` - Linear interpolation
- `rotate(&self, angle: f64, center: Point) -> Point` - Rotates around a center point
- `transform(&self, transform: &Transformation2D) -> Point` - Applies a transformation
- `polar(&self, radius: f64, angle: f64) -> Point` - Creates a point from polar coordinates

#### Vector2D

```rust
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Vector2D {
    x: f64,
    y: f64,
}
```

**Methods:**

- `new(x: f64, y: f64) -> Self` - Creates a new vector
- `zero() -> Self` - Returns the zero vector
- `from_points(from: Point, to: Point) -> Self` - Creates a vector from two points
- `length(&self) -> f64` - Returns the vector magnitude
- `normalize(&self) -> Option<Self>` - Returns a unit vector
- `dot(&self, other: Vector2D) -> f64` - Dot product
- `cross(&self, other: Vector2D) -> f64` - Cross product (2D, returns scalar)
- `angle(&self) -> f64` - Returns the angle in radians
- `perpendicular(&self) -> Self` - Returns a perpendicular vector
- `rotate(&self, angle: f64) -> Self` - Rotates the vector

#### Line

```rust
pub struct Line {
    start: Point,
    end: Point,
}
```

**Methods:**

- `new(start: Point, end: Point) -> Self` - Creates a new line segment
- `start(&self) -> Point` - Returns the start point
- `end(&self) -> Point` - Returns the end point
- `length(&self) -> f64` - Returns the line length
- `direction(&self) -> Vector2D` - Returns the direction vector
- `midpoint(&self) -> Point` - Returns the midpoint
- `is_point_on(&self, point: Point) -> bool` - Checks if a point lies on the line
- `closest_point(&self, point: Point) -> Point` - Finds the closest point on the line
- `distance_to(&self, point: Point) -> f64` - Returns the distance to a point
- `intersection(&self, other: &Line) -> Option<Point>` - Finds intersection with another line

#### Arc

```rust
pub struct Arc {
    center: Point,
    radius: f64,
    start_angle: f64,
    end_angle: f64,
    is_counter_clockwise: bool,
}
```

**Methods:**

- `new(center: Point, radius: f64, start_angle: f64, end_angle: f64) -> Self` - Creates an arc
- `center(&self) -> Point` - Returns the center point
- `radius(&self) -> f64` - Returns the radius
- `start_angle(&self) -> f64` - Returns the start angle in radians
- `end_angle(&self) -> f64` - Returns the end angle in radians
- `length(&self) -> f64` - Calculates the arc length
- `sweep_angle(&self) -> f64` - Returns the sweep angle in radians
- `start_point(&self) -> Point` - Returns the start point
- `end_point(&self) -> Point` - Returns the end point
- `point_at_angle(&self, angle: f64) -> Point` - Returns the point at a given angle
- `is_counter_clockwise(&self) -> bool` - Checks if arc is drawn CCW
- `set_counter_clockwise(&mut self, ccw: bool)` - Sets the drawing direction

#### Circle

```rust
pub struct Circle {
    center: Point,
    radius: f64,
}
```

**Methods:**

- `new(center: Point, radius: f64) -> Self` - Creates a new circle
- `center(&self) -> Point` - Returns the center point
- `radius(&self) -> f64` - Returns the radius
- `diameter(&self) -> f64` - Returns the diameter
- `area(&self) -> f64` - Returns the area
- `circumference(&self) -> f64` - Returns the circumference
- `point_at_angle(&self, angle: f64) -> Point` - Returns a point on the circumference
- `is_point_inside(&self, point: Point) -> bool` - Checks if a point is inside

#### Ellipse

```rust
pub struct Ellipse {
    center: Point,
    major_axis: Vector2D,
    minor_radius: f64,
    start_parameter: f64,
    end_parameter: f64,
}
```

**Methods:**

- `new(center: Point, major_axis: Vector2D, minor_radius: f64) -> Self` - Creates an ellipse
- `center(&self) -> Point` - Returns the center
- `major_axis(&self) -> &Vector2D` - Returns the major axis vector
- `minor_radius(&self) -> f64` - Returns the minor radius
- `major_radius(&self) -> f64` - Returns the major radius
- `eccentricity(&self) -> f64` - Calculates the eccentricity
- `point_at_parameter(&self, param: f64) -> Point` - Returns a point at parameter value

#### Polyline

```rust
pub struct Polyline {
    vertices: Vec<PolylineVertex>,
    is_closed: bool,
}
```

**Methods:**

- `new() -> Self` - Creates an empty open polyline
- `new_closed() -> Self` - Creates a closed polyline
- `add_vertex(&mut self, vertex: PolylineVertex)` - Adds a vertex
- `add_point(&mut self, point: Point)` - Adds a point vertex
- `add_arc(&mut self, point: Point, bulge: f64)` - Adds an arc vertex with bulge factor
- `is_closed(&self) -> bool` - Checks if polyline is closed
- `set_closed(&mut self, closed: bool)` - Sets the closed state
- `vertices(&self) -> &[PolylineVertex]` - Returns all vertices
- `total_length(&self) -> f64` - Calculates the total length
- `explode(&self) -> Vec<Line>` - Converts to line segments

#### BSpline

```rust
pub struct BSpline {
    control_points: Vec<Point>,
    degree: usize,
    knot_vector: Vec<f64>,
    is_rational: bool,
    weights: Option<Vec<f64>>,
}
```

**Methods:**

- `new(control_points: Vec<Point>, degree: usize) -> Result<Self, BSplineError>` - Creates a B-spline
- `control_points(&self) -> &[Point]` - Returns control points
- `degree(&self) -> usize` - Returns the degree
- `knot_vector(&self) -> &[f64]` - Returns the knot vector
- `evaluate(&self, t: f64) -> Point` - Evaluates the curve at parameter t
- `derivative(&self, t: f64) -> Vector2D` - Returns the first derivative
- `greville_abscissae(&self) -> Vec<f64>` - Returns Greville abscissae for interpolation
- `insert_knot(&mut self, t: f64, multiplicity: usize) -> Result<(), BSplineError>` - Inserts a knot
- `elevate_degree(&mut self, amount: usize)` - Elevates the degree

#### Curve

```rust
pub trait Curve {
    fn start_point(&self) -> Point;
    fn end_point(&self) -> Point;
    fn point_at(&self, t: f64) -> Point;
    fn tangent_at(&self, t: f64) -> Vector2D;
    fn length(&self, tolerance: f64) -> f64;
    fn parameter_from_point(&self, point: Point, tolerance: f64) -> Option<f64>;
}
```

Implementations: `Line`, `Arc`, `Circle`, `Ellipse`, `Polyline`, `BSpline`, `NURBS`

#### Intersection

```rust
pub struct Intersection {
    pub points: Vec<Point>,
    pub parameters: Vec<f64>,
    pub is_tangent: bool,
}
```

**Functions:**

- `line_line(line1: &Line, line2: &Line) -> Option<Intersection>` - Intersects two lines
- `line_circle(line: &Line, circle: &Circle) -> Intersection` - Intersects line and circle
- `circle_circle(circle1: &Circle, circle2: &Circle) -> Intersection` - Intersects two circles
- `line_arc(line: &Line, arc: &Arc) -> Intersection` - Intersects line and arc
- `arc_arc(arc1: &Arc, arc2: &Arc) -> Intersection` - Intersects two arcs

### Math Module

The math module provides mathematical utilities for CAD operations.

#### Matrix3x3

```rust
pub struct Matrix3x3 {
    data: [[f64; 3]; 3],
}
```

**Methods:**

- `identity() -> Self` - Creates the identity matrix
- `zero() -> Self` - Creates a zero matrix
- `new(data: [[f64; 3]; 3]) -> Self` - Creates from array
- `rotation(angle: f64) -> Self` - Creates a rotation matrix
- `translation(dx: f64, dy: f64) -> Self` - Creates a translation matrix
- `scale(sx: f64, sy: f64) -> Self` - Creates a scaling matrix
- `determinant(&self) -> f64` - Calculates the determinant
- `inverse(&self) -> Option<Self>` - Calculates the inverse matrix
- `multiply(&self, other: &Self) -> Self` - Matrix multiplication
- `transform_point(&self, point: Point) -> Point` - Transforms a point

#### Transformation2D

```rust
pub struct Transformation2D {
    matrix: Matrix3x3,
}
```

**Methods:**

- `identity() -> Self` - Creates an identity transformation
- `translation(dx: f64, dy: f64) -> Self` - Creates a translation
- `rotation(angle: f64) -> Self` - Creates a rotation about origin
- `rotation_about(angle: f64, center: Point) -> Self` - Rotation about a point
- `scale(sx: f64, sy: f64) -> Self` - Creates a scaling transformation
- `scale_uniform(s: f64) -> Self` - Uniform scaling
- `compose(&self, other: &Self) -> Self` - Composes two transformations
- `inverse(&self) -> Option<Self>` - Calculates the inverse
- `apply(&self, point: Point) -> Point` - Applies transformation to a point

### Command Module

The command module provides a command pattern implementation for CAD operations.

#### Command

```rust
pub trait Command {
    fn execute(&mut self, context: &mut CommandContext) -> Result<CommandResult, CadrsError>;
    fn undo(&mut self, context: &mut CommandContext) -> Result<CommandResult, CadrsError>;
    fn redo(&mut self, context: &mut CommandContext) -> Result<CommandResult, CadrsError>;
    fn name(&self) -> &str;
    fn is_reversible(&self) -> bool;
}
```

#### CommandManager

```rust
pub struct CommandManager {
    history: CommandHistory,
    current_command: Option<Box<dyn Command>>,
    context: CommandContext,
}
```

**Methods:**

- `new(document: Weak<Document>) -> Self` - Creates a new command manager
- `execute(&mut self, command: impl Command) -> Result<(), CadrsError>` - Executes a command
- `undo(&mut self) -> Result<(), CadrsError>` - Undoes the last command
- `redo(&mut self) -> Result<(), CadrsError>` - Redoes the last undone command
- `is_undoable(&self) -> bool` - Checks if undo is available
- `is_redoable(&self) -> bool` - Checks if redo is available
- `clear_history(&mut self)` - Clears command history
- `current_command_name(&self) -> Option<&str>` - Returns the current command name

#### CommandHistory

```rust
pub struct CommandHistory {
    undo_stack: Vec<Box<dyn Command>>,
    redo_stack: Vec<Box<dyn Command>>,
    max_history_size: usize,
}
```

**Methods:**

- `new(max_size: usize) -> Self` - Creates history with maximum size
- `push(&mut self, command: Box<dyn Command>)` - Adds a command to history
- `undo(&mut self) -> Option<Box<dyn Command>>` - Pops the last command for undo
- `redo(&mut self) -> Option<Box<dyn Command>>` - Pops the last undone command
- `can_undo(&self) -> bool` - Checks if undo is available
- `can_redo(&self) -> bool` - Checks if redo is available
- `clear(&mut self)` - Clears all history

### Render Module

The render module provides rendering capabilities for CAD entities.

#### Renderer

```rust
pub trait Renderer {
    fn initialize(&mut self) -> Result<(), RenderError>;
    fn begin_frame(&mut self) -> Result<(), RenderError>;
    fn end_frame(&mut self) -> Result<(), RenderError>;
    fn draw_entity(&mut self, entity: &Entity) -> Result<(), RenderError>;
    fn set_viewport(&mut self, viewport: &Viewport) -> Result<(), RenderError>;
    fn clear(&mut self, color: Color) -> Result<(), RenderError>;
}
```

Implementations: `SoftwareRenderer`, `GPURenderer`, `SVGRenderer`

#### Viewport

```rust
pub struct Viewport {
    origin: Point,
    size: Size,
    zoom: f64,
    rotation: f64,
    view_matrix: Matrix3x3,
}
```

**Methods:**

- `new(origin: Point, size: Size) -> Self` - Creates a new viewport
- `set_origin(&mut self, origin: Point)` - Sets the viewport origin
- `set_size(&mut self, size: Size)` - Sets the viewport size
- `zoom(&self) -> f64` - Returns the zoom level
- `set_zoom(&mut self, zoom: f64)` - Sets the zoom level
- `zoom_extents(&mut self, bounding_box: BoundingBox)` - Zooms to fit a bounding box
- `world_to_screen(&self, point: Point) -> Point` - Converts world coordinates to screen
- `screen_to_world(&self, point: Point) -> Point` - Converts screen coordinates to world

#### SVGRenderer

```rust
pub struct SVGRenderer {
    svg: SVGDocument,
}
```

**Methods:**

- `new() -> Self` - Creates a new SVG renderer
- `set_size(&mut self, width: f64, height: f64)` - Sets the SVG dimensions
- `add_layer(&mut self, name: &str, color: Color)` - Adds a layer
- `add_entity(&mut self, entity: &Entity) -> Result<(), RenderError>` - Adds an entity
- `add_text(&mut self, text: &Text) -> Result<(), RenderError>` - Adds text
- `add_dimension(&mut self, dimension: &Dimension) -> Result<(), RenderError>` - Adds a dimension
- `save(&self, path: &Path) -> Result<(), std::io::Error>` - Saves to file
- `to_string(&self) -> String` - Returns the SVG as a string

### I/O Module

The I/O module provides file import and export capabilities.

#### Exporter

```rust
pub trait Exporter {
    fn export(&self, document: &Document, stream: &mut dyn Write) -> Result<(), IoError>;
    fn extension(&self) -> &str;
    fn format_name(&self) -> &str;
}
```

Implementations: `DXFExporter`, `IGESExporter`, `STEPExporter`, `SVGExporter`

#### Importer

```rust
pub trait Importer {
    fn import_document(&self, stream: &mut dyn Read) -> Result<Document, IoError>;
    fn extension(&self) -> &str;
    fn format_name(&self) -> &str;
    fn can_import(&self) -> bool;
}
```

Implementations: `DXFImporter`, `IGESImporter`, `STEPImporter`, `SVGImporter`

#### DXFExporter

```rust
pub struct DXFExporter;
```

**Methods:**

- `new() -> Self` - Creates a new DXF exporter
- `export(&self, document: &Document, stream: &mut dyn Write) -> Result<(), IoError>` - Exports to DXF format

#### DXFImporter

```rust
pub struct DXFImporter;
```

**Methods:**

- `new() -> Self` - Creates a new DXF importer
- `import_document(&self, stream: &mut dyn Read) -> Result<Document, IoError>` - Imports from DXF format

### Constraint Module

The constraint module provides geometric constraint solving capabilities.

#### ConstraintSolver

```rust
pub struct ConstraintSolver {
    variables: Vec<Variable>,
    constraints: Vec<Box<dyn Constraint>>,
    solver: Box<dyn SolverAlgorithm>,
}
```

**Methods:**

- `new() -> Self` - Creates a new constraint solver
- `add_variable(&mut self, name: &str, initial_value: f64) -> VariableId` - Adds a variable
- `add_constraint(&mut self, constraint: impl Constraint) -> Result<ConstraintId, ConstraintError>` - Adds a constraint
- `remove_constraint(&mut self, id: ConstraintId) -> Result<(), ConstraintError>` - Removes a constraint
- `solve(&mut self) -> Result<SolveResult, SolverError>` - Solves the constraint system
- `set_variable_value(&mut self, id: VariableId, value: f64)` - Sets a variable value
- `get_variable_value(&self, id: VariableId) -> f64` - Gets a variable value
- `add_equality(&mut self, expr1: &Expression, expr2: &Expression) -> Result<ConstraintId, ConstraintError>` - Adds equality constraint

### Dimension Module

The dimension module provides dimension annotation functionality.

#### Dimension

```rust
pub trait Dimension {
    fn dimension_type(&self) -> DimensionType;
    fn measurement(&self) -> f64;
    fn text(&self) -> &str;
    fn set_text(&mut self, text: &str);
    fn update_measurement(&mut self);
}
```

Implementations: `LinearDimension`, `AngularDimension`, `RadialDimension`, `OrdinateDimension`

#### LinearDimension

```rust
pub struct LinearDimension {
    attachment_point: AttachmentPoint,
    dimension_line_location: Point,
    text_position: Point,
    measurement: f64,
    text: String,
    tolerance: Option<Tolerance>,
}
```

**Methods:**

- `new(attachment: AttachmentPoint, location: Point) -> Self` - Creates a linear dimension
- `attachment_point(&self) -> AttachmentPoint` - Returns the attachment point
- `dimension_line_location(&self) -> Point` - Returns the dimension line location
- `set_text_position(&mut self, pos: Point)` - Sets the text position
- `measurement(&self) -> f64` - Returns the measurement value
- `tolerance(&self) -> Option<&Tolerance>` - Returns the tolerance

#### AngularDimension

```rust
pub struct AngularDimension {
    center: Point,
    start_line: Line,
    end_line: Line,
    arc_radius: f64,
    text_position: Point,
    measurement: f64,
    text: String,
}
```

**Methods:**

- `new(center: Point, start_line: Line, end_line: Line) -> Self` - Creates an angular dimension
- `center(&self) -> Point` - Returns the center point
- `arc_radius(&self) -> f64` - Returns the arc radius
- `measurement(&self) -> f64` - Returns the angle in degrees

#### RadialDimension

```rust
pub struct RadialDimension {
    center: Point,
    arc: Arc,
    text_position: Point,
    leader_length: f64,
}
```

**Methods:**

- `new(center: Point, arc: Arc) -> Self` - Creates a radial dimension
- `center(&self) -> Point` - Returns the center point
- `radius(&self) -> f64` - Returns the radius
- `diameter(&self) -> f64` - Returns the diameter

### Text Module

The text module provides text annotation capabilities.

#### Text

```rust
pub struct Text {
    position: Point,
    content: String,
    height: f64,
    rotation: f64,
    width_factor: f64,
    style: TextStyle,
    horizontal_alignment: HorizontalAlignment,
    vertical_alignment: VerticalAlignment,
}
```

**Methods:**

- `new(position: Point, content: &str) -> Self` - Creates a new text entity
- `position(&self) -> Point` - Returns the insertion point
- `content(&self) -> &str` - Returns the text content
- `set_content(&mut self, content: &str)` - Sets the text content
- `height(&self) -> f64` - Returns the text height
- `set_height(&mut self, height: f64)` - Sets the text height
- `style(&self) -> &TextStyle` - Returns the text style
- `set_style(&mut self, style: TextStyle)` - Sets the text style
- `rotation(&self) -> f64` - Returns the rotation angle in degrees
- `bounds(&self) -> BoundingBox` - Returns the bounding box

#### TextStyle

```rust
pub struct TextStyle {
    name: String,
    font: String,
    height: f64,
    width_factor: f64,
    oblique_angle: f64,
    is_upside_down: bool,
    is_backwards: bool,
}
```

**Methods:**

- `standard() -> Self` - Returns the standard text style
- `new(name: &str) -> Self` - Creates a new text style
- `name(&self) -> &str` - Returns the style name
- `font(&self) -> &str` - Returns the font name
- `set_font(&mut self, font: &str)` - Sets the font

#### MText

```rust
pub struct MText {
    position: Point,
    content: String,
    height: f64,
    width: f64,
    style: TextStyle,
    attachment: MTextAttachment,
    line_spacing: f64,
}
```

**Methods:**

- `new(position: Point, content: &str) -> Self` - Creates a new mtext entity
- `content(&self) -> &str` - Returns the full text content
- `append(&mut self, text: &str)` - Appends text to mtext
- `add_symbol(&mut self, symbol: MTextSymbol)` - Adds a special symbol
- `add_field(&mut self, field: Field)` - Adds a field

### Snap Module

The snap module provides object snapping functionality.

#### SnapPoint

```rust
pub struct SnapPoint {
    point: Point,
    snap_type: SnapType,
    source_entity: EntityId,
    display_marker: Option<Marker>,
}
```

**Methods:**

- `new(point: Point, snap_type: SnapType, source: EntityId) -> Self` - Creates a snap point
- `point(&self) -> Point` - Returns the snap point location
- `snap_type(&self) -> SnapType` - Returns the snap type
- `source_entity(&self) -> EntityId` - Returns the source entity ID

#### OSnap

```rust
pub struct OSnap {
    enabled: bool,
    snap_points: Vec<SnapPoint>,
    aperture_size: f64,
}
```

**Methods:**

- `new() -> Self` - Creates a new OSnap manager
- `enable(&mut self)` - Enables object snapping
- `disable(&mut self)` - Disables object snapping
- `is_enabled(&self) -> bool` - Checks if enabled
- `add_snap_point(&mut self, snap: SnapPoint)` - Adds a snap point
- `find_snap_points(&self, cursor: Point, tolerance: f64) -> Vec<SnapPoint>` - Finds snap points near cursor
- `set_aperture(&mut self, size: f64)` - Sets the aperture size

### Edit Module

The edit module provides entity editing capabilities.

#### Transformation

```rust
pub struct Transformation {
    translation: Option<Vector2D>,
    rotation: Option<f64>,
    scale: Option<Vector2D>,
    mirror_line: Option<Line>,
}
```

**Methods:**

- `new() -> Self` - Creates an identity transformation
- `with_translation(&mut self, translation: Vector2D) -> &mut Self` - Adds translation
- `with_rotation(&mut self, angle: f64, center: Point) -> &mut Self` - Adds rotation
- `with_scale(&mut self, scale: Vector2D, center: Point) -> &mut Self` - Adds scaling
- `with_mirror(&mut self, line: Line) -> &mut Self` - Adds mirroring
- `apply(&self, entity: &mut Entity)` - Applies transformation to an entity

#### GripEditor

```rust
pub struct GripEditor {
    grips: HashMap<EntityId, Vec<Grip>>,
    active_grip: Option<ActiveGrip>,
    grip_size: f64,
}
```

**Methods:**

- `new() -> Self` - Creates a new grip editor
- `add_entity(&mut self, entity: &Entity)` - Adds grips for an entity
- `remove_entity(&mut self, id: EntityId)` - Removes grips for an entity
- `find_grip_at(&self, point: Point, tolerance: f64) -> Option<ActiveGrip>` - Finds grip at point
- `move_grip(&mut self, grip: ActiveGrip, delta: Vector2D)` - Moves a grip
- `set_grip_size(&mut self, size: f64)` - Sets the visual grip size
- `clear(&mut self)` - Clears all grips

#### Grip

```rust
pub struct Grip {
    position: Point,
    grip_type: GripType,
    is_mandatory: bool,
}
```

**Methods:**

- `new(position: Point, grip_type: GripType) -> Self` - Creates a new grip
- `position(&self) -> Point` - Returns the grip position
- `grip_type(&self) -> GripType` - Returns the grip type

### Grid Module

The grid module provides grid functionality for drafting.

#### Grid

```rust
pub struct Grid {
    is_visible: bool,
    is_snap_enabled: bool,
    spacing: Vector2D,
    major_line_every: usize,
    origin: Point,
    limits: BoundingBox,
    sub_divisions: usize,
}
```

**Methods:**

- `new() -> Self` - Creates a default grid
- `set_visible(&mut self, visible: bool)` - Sets visibility
- `is_visible(&self) -> bool` - Checks if visible
- `enable_snap(&mut self)` - Enables snap to grid
- `disable_snap(&mut self)` - Disables snap to grid
- `is_snap_enabled(&self) -> bool` - Checks if snap is enabled
- `spacing(&self) -> Vector2D` - Returns the grid spacing
- `set_spacing(&mut self, spacing: Vector2D)` - Sets the grid spacing
- `snap(&self, point: Point) -> Point` - Snaps a point to the grid
- `set_origin(&mut self, origin: Point)` - Sets the grid origin

### Performance Module

The performance module provides utilities for optimizing CAD operations.

#### Parallel

```rust
pub struct Parallel;
```

**Methods:**

- `new() -> Self` - Creates a parallel processor
- `map<T, R, F>(&self, data: &[T], f: F) -> Vec<R>` - Parallel map operation
- `reduce<T, F>(&self, data: &[T], f: F) -> T` - Parallel reduce operation
- `filter_map<T, R, F>(&self, data: &[T], f: F) -> Vec<R>` - Parallel filter-map
- `geometry_bvh<T: Bounded>(&self, objects: &[T]) -> BVH` - Builds a BVH for geometry

#### Memory

```rust
pub struct Memory;
```

**Methods:**

- `new() -> Self` - Creates a memory manager
- `allocate(&self, size: usize) -> *mut u8` - Allocates memory
- `deallocate(&self, ptr: *mut u8)` - Deallocates memory
- `reallocate(&self, ptr: *mut u8, new_size: usize) -> *mut u8` - Reallocates memory
- `set_memory_pool(&self, pool: MemoryPool)` - Sets a custom memory pool
- `optimize_for_geometry(&self, entities: &[Entity])` - Optimizes memory for geometry entities

### Selection Module

The selection module provides entity selection functionality.

#### Selection

```rust
pub struct Selection {
    selected_entities: BTreeSet<EntityId>,
    selection_mode: SelectionMode,
    selection_window: Option<SelectionWindow>,
    cross_selection: bool,
}
```

**Methods:**

- `new() -> Self` - Creates a new selection
- `select(&mut self, id: EntityId)` - Selects an entity
- `deselect(&mut self, id: EntityId)` - Deselects an entity
- `select_all(&mut self, document: &Document)` - Selects all visible entities
- `clear(&mut self)` - Clears all selections
- `is_selected(&self, id: EntityId) -> bool` - Checks if entity is selected
- `selected_count(&self) -> usize` - Returns the count of selected entities
- `window_select(&mut self, window: SelectionWindow, add: bool)` - Performs window selection
- `set_selection_mode(&mut self, mode: SelectionMode)` - Sets the selection mode
- `add_to_selection(&mut self, id: EntityId)` - Adds to selection without clearing

### Hatch Module

The hatch module provides hatching functionality for closed areas.

#### Hatch

```rust
pub struct Hatch {
    pattern_name: String,
    pattern_scale: f64,
    pattern_angle: f64,
    boundary_curves: Vec<Box<dyn Curve>>,
    islands: Vec<HatchIsland>,
    origin_point: Point,
    is_associative: bool,
}
```

**Methods:**

- `new(pattern_name: &str) -> Self` - Creates a new hatch
- `set_pattern(&mut self, name: &str, scale: f64, angle: f64)` - Sets the hatch pattern
- `add_boundary(&mut self, curve: Box<dyn Curve>)` - Adds a boundary curve
- `set_origin(&mut self, point: Point)` - Sets the hatch origin
- `generate_pattern(&self) -> Vec<Line>` - Generates the hatch lines
- `is_point_inside(&self, point: Point) -> bool` - Tests if point is inside hatch

#### HatchPattern

```rust
pub struct HatchPattern {
    name: String,
    description: String,
    angle: f64,
    scale: f64,
    pattern_lines: Vec<PatternLine>,
}
```

**Methods:**

- `standard() -> &'static Self` - Returns the standard hatch pattern
- `dots() -> &'static Self` - Returns the dot pattern
- `angled() -> &'static Self` - Returns an angled pattern
- `cross() -> &'static Self` - Returns a cross hatch pattern

## Error Handling

### CadrsError

```rust
#[derive(Debug, Error)]
pub enum CadrsError {
    #[error("Entity not found: {0}")]
    EntityNotFound(EntityId),
    
    #[error("Layer not found: {0}")]
    LayerNotFound(String),
    
    #[error("Invalid operation: {0}")]
    InvalidOperation(String),
    
    #[error("Geometry error: {0}")]
    GeometryError(String),
    
    #[error("I/O error: {0}")]
    IoError(#[from] std::io::Error),
    
    #[error("Constraint error: {0}")]
    ConstraintError(String),
    
    #[error("Solver error: {0}")]
    SolverError(String),
    
    #[error("Render error: {0}")]
    RenderError(String),
}
```

## Usage Examples

### Basic Document Operations

```rust
use cadrs::prelude::*;

fn main() -> Result<(), CadrsError> {
    // Create a new document
    let mut doc = Document::new();
    
    // Add a layer
    let layer_id = doc.add_layer(Layer::new("MyLayer"))?;
    
    // Create entities
    let line = Line::new(Point::new(0.0, 0.0), Point::new(10.0, 10.0));
    let circle = Circle::new(Point::new(5.0, 5.0), 3.0);
    
    // Add to document
    doc.add_entity(&line)?;
    doc.add_entity(&circle)?;
    
    Ok(())
}
```

### Geometric Transformations

```rust
fn transform_example() {
    let point = Point::new(10.0, 10.0);
    let transform = Transformation2D::rotation(std::f64::consts::PI / 4.0);
    let rotated = transform.apply(point);
    
    let scale = Transformation2D::scale_uniform(2.0);
    let scaled = scale.apply(point);
}
```

### File I/O

```rust
use cadrs::io::{DXFExporter, DXFImporter, Exporter, Importer};

fn io_example() -> Result<(), CadrsError> {
    let doc = Document::new();
    
    // Export to DXF
    let exporter = DXFExporter;
    let mut output = Vec::new();
    exporter.export(&doc, &mut output)?;
    
    // Import from DXF
    let importer = DXFImporter;
    let imported_doc = importer.import_document(&mut &output[..])?;
    
    Ok(())
}
```

### Rendering with SVG

```rust
fn svg_example(document: &Document) -> Result<(), CadrsError> {
    let mut renderer = SVGRenderer::new();
    renderer.set_size(800.0, 600.0);
    
    for entity in document.entities() {
        renderer.add_entity(entity)?;
    }
    
    renderer.save(Path::new("output.svg"))?;
    Ok(())
}
```

### Command System

```rust
fn command_example() {
    let doc = Document::new();
    let manager = CommandManager::new(&doc);
    
    // Commands can be executed through the command manager
    // They support undo/redo functionality
}
```

## Feature Flags

The following feature flags control which modules are compiled:

| Flag | Description |
|------|-------------|
| `geometry` | Enable geometry primitives (default) |
| `math` | Enable mathematical utilities (default) |
| `data_structure` | Enable document management (default) |
| `io` | Enable file I/O support (default) |
| `render` | Enable rendering engine (default) |
| `api` | Enable high-level API |
| `performance` | Enable performance utilities |
| `boolean` | Enable boolean operations |
| `constraint` | Enable constraint solving |
| `gpu` | Enable GPU rendering features |

## License

MIT License - see [LICENSE](LICENSE) file for details.
