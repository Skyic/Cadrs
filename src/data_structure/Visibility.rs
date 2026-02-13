use serde::{Serialize, Deserialize};
use std::fmt;

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub enum Visibility {
    Visible,
    Hidden,
    Frozen,
    Thawed,
}

impl Default for Visibility {
    fn default() -> Self {
        Visibility::Visible
    }
}

impl Visibility {
    pub fn is_visible(&self) -> bool {
        matches!(self, Visibility::Visible)
    }

    pub fn is_hidden(&self) -> bool {
        matches!(self, Visibility::Hidden)
    }

    pub fn is_frozen(&self) -> bool {
        matches!(self, Visibility::Frozen)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct Transform {
    pub matrix: [[f64; 4]; 4],
    pub translation_x: f64,
    pub translation_y: f64,
    pub translation_z: f64,
    pub rotation_x: f64,
    pub rotation_y: f64,
    pub rotation_z: f64,
    pub scale_x: f64,
    pub scale_y: f64,
    pub scale_z: f64,
}

impl Default for Transform {
    fn default() -> Self {
        Self::identity()
    }
}

impl Transform {
    pub fn identity() -> Self {
        Self {
            matrix: [
                [1.0, 0.0, 0.0, 0.0],
                [0.0, 1.0, 0.0, 0.0],
                [0.0, 0.0, 1.0, 0.0],
                [0.0, 0.0, 0.0, 1.0],
            ],
            translation_x: 0.0,
            translation_y: 0.0,
            translation_z: 0.0,
            rotation_x: 0.0,
            rotation_y: 0.0,
            rotation_z: 0.0,
            scale_x: 1.0,
            scale_y: 1.0,
            scale_z: 1.0,
        }
    }

    pub fn translation(tx: f64, ty: f64, tz: f64) -> Self {
        let mut transform = Self::identity();
        transform.translation_x = tx;
        transform.translation_y = ty;
        transform.translation_z = tz;

        transform.matrix[0][3] = tx;
        transform.matrix[1][3] = ty;
        transform.matrix[2][3] = tz;

        transform
    }

    pub fn rotation(rx: f64, ry: f64, rz: f64) -> Self {
        let mut transform = Self::identity();
        transform.rotation_x = rx;
        transform.rotation_y = ry;
        transform.rotation_z = rz;
        transform
    }

    pub fn scale(sx: f64, sy: f64, sz: f64) -> Self {
        let mut transform = Self::identity();
        transform.scale_x = sx;
        transform.scale_y = sy;
        transform.scale_z = sz;

        transform.matrix[0][0] = sx;
        transform.matrix[1][1] = sy;
        transform.matrix[2][2] = sz;

        transform
    }

    pub fn is_identity(&self) -> bool {
        self.translation_x == 0.0 && self.translation_y == 0.0 && self.translation_z == 0.0 &&
        self.rotation_x == 0.0 && self.rotation_y == 0.0 && self.rotation_z == 0.0 &&
        self.scale_x == 1.0 && self.scale_y == 1.0 && self.scale_z == 1.0
    }
}

impl fmt::Display for Transform {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "Transform(trans=({}, {}, {}), rot=({}, {}, {}), scale=({}, {}, {}))",
            self.translation_x, self.translation_y, self.translation_z,
            self.rotation_x, self.rotation_y, self.rotation_z,
            self.scale_x, self.scale_y, self.scale_z
        )
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum HatchBoundary {
    External,
    Outer,
    Inner,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum BoundaryType {
    External,
    Polyline,
    Derived,
    Outermost,
    Notch,
    Hull,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct HatchEdge {
    pub start_point: crate::geometry::Point,
    pub end_point: crate::geometry::Point,
    pub bulge: f64,
    pub edge_type: EdgeType,
}

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub enum EdgeType {
    Line,
    CircularArc,
    EllipticArc,
    Spline,
}

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub enum DimensionType {
    Linear,
    Aligned,
    Angular,
    Radial,
    Diameter,
    ArcLength,
    Ordinate,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TextStyle {
    pub name: String,
    pub font: String,
    pub height: f64,
    pub width_factor: f64,
    pub oblique_angle: f64,
    pub is_backwards: bool,
    pub is_upside_down: bool,
}

impl Default for TextStyle {
    fn default() -> Self {
        Self {
            name: "Standard".to_string(),
            font: "Arial".to_string(),
            height: 2.5,
            width_factor: 1.0,
            oblique_angle: 0.0,
            is_backwards: false,
            is_upside_down: false,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub enum TextAlignment {
    Left,
    Center,
    Right,
    Middle,
    Top,
    MiddleTop,
    MiddleMiddle,
    MiddleBottom,
    Bottom,
}

impl Default for TextAlignment {
    fn default() -> Self {
        TextAlignment::Left
    }
}
