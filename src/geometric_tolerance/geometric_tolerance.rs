use super::geometry::Point;

#[derive(Debug, Clone)]
pub struct GeometricTolerance {
    pub frame: ToleranceFrame,
    pub position: Point,
    pub rotation: f64,
    pub tolerance_zone_shape: ToleranceZoneShape,
    pub material_condition: MaterialCondition,
    pub datum_identifier: Option<DatumIdentifier>,
}

impl GeometricTolerance {
    pub fn new(
        tolerance_type: ToleranceType,
        value: f64,
        primary_datum: Option<Datum>,
    ) -> Self {
        let frame = ToleranceFrame::new(tolerance_type, value, primary_datum);

        Self {
            frame,
            position: Point::default(),
            rotation: 0.0,
            tolerance_zone_shape: ToleranceZoneShape::Cylindrical,
            material_condition: MaterialCondition::MaximumMaterialCondition,
            datum_identifier: None,
        }
    }

    pub fn with_position(mut self, position: Point) -> Self {
        self.position = position;
        self
    }

    pub fn with_rotation(mut self, rotation: f64) -> Self {
        self.rotation = rotation;
        self
    }

    pub fn with_secondary_datum(mut self, datum: Datum) -> Self {
        self.frame.secondary_datum = Some(datum);
        self
    }

    pub fn with_tertiary_datum(mut self, datum: Datum) -> Self {
        self.frame.tertiary_datum = Some(datum);
        self
    }

    pub fn generate_frame_box(&self) -> Vec<FrameBox> {
        self.frame.generate_boxes()
    }
}

#[derive(Debug, Clone)]
pub struct ToleranceFrame {
    pub tolerance_type: ToleranceType,
    pub value: f64,
    pub primary_datum: Option<Datum>,
    pub secondary_datum: Option<Datum>,
    pub tertiary_datum: Option<Datum>,
    pub projected_tolerance: Option<f64>,
    pub modifier: Option<FrameModifier>,
}

impl ToleranceFrame {
    pub fn new(
        tolerance_type: ToleranceType,
        value: f64,
        primary_datum: Option<Datum>,
    ) -> Self {
        Self {
            tolerance_type,
            value,
            primary_datum,
            secondary_datum: None,
            tertiary_datum: None,
            projected_tolerance: None,
            modifier: None,
        }
    }

    pub fn with_projected_tolerance(mut self, value: f64) -> Self {
        self.projected_tolerance = Some(value);
        self
    }

    pub fn with_modifier(mut self, modifier: FrameModifier) -> Self {
        self.modifier = Some(modifier);
        self
    }

    pub fn generate_boxes(&self) -> Vec<FrameBox> {
        let mut boxes = Vec::new();

        let symbol_char = self.tolerance_type.symbol_char();
        let value_str = if self.value == 0.0 {
            String::new()
        } else {
            format!("{:.3}", self.value)
        };

        let tolerance_text = format!("{}{}", symbol_char, value_str);

        boxes.push(FrameBox {
            text: tolerance_text,
            width: tolerance_text.len() as f64 * 3.0 + 4.0,
        });

        if let Some(ref datum) = self.primary_datum {
            let datum_text = format!("[{}]", datum.identifier);
            boxes.push(FrameBox {
                text: datum_text,
                width: datum_text.len() as f64 * 3.0 + 2.0,
            });
        }

        if let Some(ref datum) = self.secondary_datum {
            let datum_text = format!("[{}]", datum.identifier);
            boxes.push(FrameBox {
                text: datum_text,
                width: datum_text.len() as f64 * 3.0 + 2.0,
            });
        }

        if let Some(ref datum) = self.tertiary_datum {
            let datum_text = format!("[{}]", datum.identifier);
            boxes.push(FrameBox {
                text: datum_text,
                width: datum_text.len() as f64 * 3.0 + 2.0,
            });
        }

        boxes
    }
}

#[derive(Debug, Clone)]
pub struct FrameBox {
    pub text: String,
    pub width: f64,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ToleranceType {
    Flatness,
    Straightness,
    Circularity,
    Cylindricity,
    ProfileOfLine,
    ProfileOfSurface,
    Angularity,
    Perpendicularity,
    Parallelism,
    Position,
    Symmetry,
    CircularRunout,
    TotalRunout,
}

impl ToleranceType {
    pub fn symbol_char(&self) -> &'static str {
        match self {
            ToleranceType::Flatness => "⏤",
            ToleranceType::Straightness => "—",
            ToleranceType::Circularity => "○",
            ToleranceType::Cylindricity => "⌥",
            ToleranceType::ProfileOfLine => "⌒",
            ToleranceType::ProfileOfSurface => "⏭",
            ToleranceType::Angularity => "∠",
            ToleranceType::Perpendicularity => "⊥",
            ToleranceType::Parallelism => "∥",
            ToleranceType::Position => "⏨",
            ToleranceType::Symmetry => "⌖",
            ToleranceType::CircularRunout => "↻",
            ToleranceType::TotalRunout => "⟳",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ToleranceZoneShape {
    Cylindrical,
    Spherical,
    TwoParallelLines,
    TwoParallelPlanes,
    Circle,
    Sphere,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum MaterialCondition {
    MaximumMaterialCondition,
    LeastMaterialCondition,
    RegardlessOfFeatureSize,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum FrameModifier {
    MaximumMaterialCondition,
    LeastMaterialCondition,
    RegardlessOfFeatureSize,
    ProjectedToleranceZone,
    EnvelopeRequirement,
    TangentPlane,
}

#[derive(Debug, Clone)]
pub struct Datum {
    pub identifier: String,
    pub datum_feature: Option<DatumFeature>,
}

impl Datum {
    pub fn new(identifier: String) -> Self {
        Self {
            identifier,
            datum_feature: None,
        }
    }
}

#[derive(Debug, Clone)]
pub struct DatumFeature {
    pub feature_type: DatumFeatureType,
    pub target_points: Vec<Point>,
    pub target_lines: Vec<TargetLine>,
    pub target_areas: Vec<TargetArea>,
}

impl DatumFeature {
    pub fn new(feature_type: DatumFeatureType) -> Self {
        Self {
            feature_type,
            target_points: Vec::new(),
            target_lines: Vec::new(),
            target_areas: Vec::new(),
        }
    }

    pub fn add_target_point(mut self, point: Point) -> Self {
        self.target_points.push(point);
        self
    }

    pub fn add_target_line(mut self, line: TargetLine) -> Self {
        self.target_lines.push(line);
        self
    }

    pub fn add_target_area(mut self, area: TargetArea) -> Self {
        self.target_areas.push(area);
        self
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum DatumFeatureType {
    Plane,
    Line,
    Point,
    Cylinder,
    Sphere,
    Cone,
    Torus,
}

#[derive(Debug, Clone)]
pub struct TargetPoint {
    pub position: Point,
    pub diameter: f64,
}

#[derive(Debug, Clone)]
pub struct TargetLine {
    pub start: Point,
    pub end: Point,
    pub length: f64,
}

#[derive(Debug, Clone)]
pub struct TargetArea {
    pub center: Point,
    pub diameter: f64,
}

#[derive(Debug, Clone)]
pub struct DatumIdentifier {
    pub datum: Datum,
    pub position: Point,
    pub rotation: f64,
    pub style: DatumIdentifierStyle,
    pub height: f64,
}

impl DatumIdentifier {
    pub fn new(datum: Datum, position: Point) -> Self {
        Self {
            datum,
            position,
            rotation: 0.0,
            style: DatumIdentifierStyle::Basic,
            height: 3.5,
        }
    }

    pub fn with_rotation(mut self, rotation: f64) -> Self {
        self.rotation = rotation;
        self
    }

    pub fn with_style(mut self, style: DatumIdentifierStyle) -> Self {
        self.style = style;
        self
    }

    pub fn generate_display_geometry(&self) -> DisplayGeometry {
        let label = format!("[{}]", self.datum.identifier);

        DisplayGeometry {
            label,
            position: self.position,
            rotation: self.rotation,
            height: self.height,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum DatumIdentifierStyle {
    Basic,
    WithFrame,
    WithoutBracket,
}

#[derive(Debug, Clone)]
pub struct DisplayGeometry {
    pub label: String,
    pub position: Point,
    pub rotation: f64,
    pub height: f64,
}

#[derive(Debug, Clone)]
pub struct SurfaceTexture {
    pub symbol: SurfaceTextureSymbol,
    pub value: f64,
    pub evaluation_length: Option<f64>,
    pub material_condition: SurfaceTextureCondition,
    pub production_method: Option<String>,
    pub direction_ofLay: Option<LayDirection>,
    pub sampling_length: Option<f64>,
    pub position: Point,
    pub rotation: f64,
    pub height: f64,
}

impl SurfaceTexture {
    pub fn new(
        symbol: SurfaceTextureSymbol,
        value: f64,
        position: Point,
    ) -> Self {
        Self {
            symbol,
            value,
            evaluation_length: None,
            material_condition: SurfaceTextureCondition::Machined,
            production_method: None,
            direction_ofLay: None,
            sampling_length: None,
            position,
            rotation: 0.0,
            height: 3.5,
        }
    }

    pub fn with_evaluation_length(mut self, length: f64) -> Self {
        self.evaluation_length = Some(length);
        self
    }

    pub fn with_material_condition(mut self, condition: SurfaceTextureCondition) -> Self {
        self.material_condition = condition;
        self
    }

    pub fn with_production_method(mut self, method: String) -> Self {
        self.production_method = Some(method);
        self
    }

    pub fn with_lay_direction(mut self, lay: LayDirection) -> Self {
        self.direction_ofLay = Some(lay);
        self
    }

    pub fn with_rotation(mut self, rotation: f64) -> Self {
        self.rotation = rotation;
        self
    }

    pub fn generate_label(&self) -> String {
        let mut parts = Vec::new();

        parts.push(self.symbol.symbol_char());

        let value_str = format!("{:.3}", self.value);
        if let Some(ref method) = self.production_method {
            parts.push(format!("{} {}", method, value_str));
        } else {
            parts.push(value_str);
        }

        if let Some(ref lay) = self.direction_ofLay {
            parts.push(lay.symbol_char());
        }

        if let Some(length) = self.sampling_length {
            parts.push(format!("×{:.3}", length));
        }

        if let Some(length) = self.evaluation_length {
            parts.push(format!("({:.3})", length));
        }

        parts.join("")
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum SurfaceTextureSymbol {
    Roughness,
    RoughnessWithMachining,
    RemovalByMachining,
    NoRemovalByMachining,
    RoughnessParam1,
    RoughnessParam2,
    RoughnessParam3,
}

impl SurfaceTextureSymbol {
    pub fn symbol_char(&self) -> &'static str {
        match self {
            SurfaceTextureSymbol::Roughness => "⌔",
            SurfaceTextureSymbol::RoughnessWithMachining => "⌒",
            SurfaceTextureSymbol::RemovalByMachining => "▭",
            SurfaceTextureSymbol::NoRemovalByMachining => "□",
            SurfaceTextureSymbol::RoughnessParam1 => "Ra",
            SurfaceTextureSymbol::RoughnessParam2 => "Ry",
            SurfaceTextureSymbol::RoughnessParam3 => "Rz",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum SurfaceTextureCondition {
    Machined,
    NonMachined,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum LayDirection {
    Parallel,
    Perpendicular,
    Angular,
    Multidirectional,
    Radial,
    Circular,
}

impl LayDirection {
    pub fn symbol_char(&self) -> &'static str {
        match self {
            LayDirection::Parallel => "∥",
            LayDirection::Perpendicular => "⊥",
            LayDirection::Angular => "∠",
            LayDirection::Multidirectional => "≋",
            LayDirection::Radial => "⨀",
            LayDirection::Circular => "○",
        }
    }
}

#[derive(Debug, Clone)]
pub struct WeldSymbol {
    pub weld_type: WeldType,
    pub size: Option<f64>,
    pub length: Option<f64>,
    pub pitch: Option<f64>,
    pub tail: Option<WeldTail>,
    pub field_weld: bool,
    pub all_around: bool,
    pub reference_line: WeldReferenceLine,
    pub symbols: Vec<SupplementarySymbol>,
    pub position: Point,
    pub rotation: f64,
}

impl WeldSymbol {
    pub fn new(weld_type: WeldType) -> Self {
        Self {
            weld_type,
            size: None,
            length: None,
            pitch: None,
            tail: None,
            field_weld: false,
            all_around: false,
            reference_line: WeldReferenceLine::Top,
            symbols: Vec::new(),
            position: Point::default(),
            rotation: 0.0,
        }
    }

    pub fn with_size(mut self, size: f64) -> Self {
        self.size = Some(size);
        self
    }

    pub fn with_length(mut self, length: f64) -> Self {
        self.length = Some(length);
        self
    }

    pub fn with_pitch(mut self, pitch: f64) -> Self {
        self.pitch = Some(pitch);
        self
    }

    pub fn as_field_weld(mut self) -> Self {
        self.field_weld = true;
        self
    }

    pub fn as_all_around(mut self) -> Self {
        self.all_around = true;
        self
    }

    pub fn with_supplementary_symbol(mut self, symbol: SupplementarySymbol) -> Self {
        self.symbols.push(symbol);
        self
    }

    pub fn with_position(mut self, position: Point) -> Self {
        self.position = position;
        self
    }

    pub fn with_rotation(mut self, rotation: f64) -> Self {
        self.rotation = rotation;
        self
    }

    pub fn generate_symbol_string(&self) -> String {
        let mut parts = Vec::new();

        if let Some(size) = self.size {
            parts.push(format!("{}", size));
        }

        if let Some(length) = self.length {
            parts.push(format!("-{}", length));
        }

        if let Some(pitch) = self.pitch {
            parts.push(format!("@{}", pitch));
        }

        for symbol in &self.symbols {
            parts.push(symbol.symbol_char());
        }

        if self.field_weld {
            parts.push(" flag".to_string());
        }

        if self.all_around {
            parts.push("◠".to_string());
        }

        parts.join("")
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum WeldType {
    Fillet,
    Groove,
    Spot,
    Seam,
    Plug,
    Slot,
    Surface,
    Back,
    MeltThrough,
    Stud,
    Flange,
    Edge,
    SquareGroove,
    VGroove,
    BevelGroove,
    UGroove,
    JGroove,
}

impl WeldType {
    pub fn symbol_char(&self) -> &'static str {
        match self {
            WeldType::Fillet => "▭",
            WeldType::Groove => "⌒",
            WeldType::Spot => "○",
            WeldType::Seam => "≡",
            WeldType::Plug => "▭",
            WeldType::Slot => "□",
            WeldType::Surface => "▬",
            WeldType::Back => "▱",
            WeldType::MeltThrough => "▱",
            WeldType::Stud => "▭",
            WeldType::Flange => "⌒",
            WeldType::Edge => "▬",
            WeldType::SquareGroove => "▬",
            WeldType::V Groove => "V",
            WeldType::BevelGroove => "L",
            WeldType::UGroove => "U",
            WeldType::JGroove => "J",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum WeldReferenceLine {
    Top,
    Bottom,
    Both,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum SupplementarySymbol {
    WeldRoot,
    Contour,
    Grind,
    Flush,
    Convex,
    Concave,
    Flat,
    ConvexContour,
    ConcaveContour,
    Melted,
    Unmelted,
    Backing,
}

impl SupplementarySymbol {
    pub fn symbol_char(&self) -> &'static str {
        match self {
            SupplementarySymbol::WeldRoot => "▸",
            SupplementarySymbol::Contour => "⌒",
            SupplementarySymbol::Grind => "G",
            SupplementarySymbol::Flush => "F",
            SupplementarySymbol::Convex => "C",
            SupplementarySymbol::Concave => "CC",
            SupplementarySymbol::Flat => "FL",
            SupplementarySymbol::ConvexContour => "C⌒",
            SupplementarySymbol::ConcaveContour => "CC⌒",
            SupplementarySymbol::Melted => "M",
            SupplementarySymbol::Unmelted => "UM",
            SupplementarySymbol::Backing => "B",
        }
    }
}

#[derive(Debug, Clone)]
pub struct WeldTail {
    pub specification: String,
    pub reference: String,
}

#[derive(Debug, Clone)]
pub struct WeldAllAround {
    pub symbol: char,
    pub position: Point,
    pub rotation: f64,
}

impl WeldAllAround {
    pub fn new(position: Point) -> Self {
        Self {
            symbol: '◠',
            position,
            rotation: 0.0,
        }
    }
}
