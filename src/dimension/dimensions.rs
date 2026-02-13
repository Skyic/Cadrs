use super::geometry::{Point, Line, Arc};

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum DimensionType {
    Linear,
    Aligned,
    Horizontal,
    Vertical,
    Rotated,
    Angular,
    Radial,
    Diameter,
    Ordinate,
    ArcLength,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum LinearDimensionType {
    Horizontal,
    Vertical,
    Aligned,
    Rotated(f64),
}

#[derive(Debug, Clone)]
pub struct LinearDimension {
    pub dim_type: LinearDimensionType,
    pub extension_lines: [Point; 2],
    pub text: String,
    pub text_height: f64,
    pub arrow_size: f64,
    pub extension_extend: f64,
    pub gap_size: f64,
    pub tolerance: Option<DimensionTolerance>,
    pub dimension_line: Point,
    pub is_left_to_right: bool,
}

impl LinearDimension {
    pub fn new(
        dim_type: LinearDimensionType,
        first_point: Point,
        second_point: Point,
        text: String,
    ) -> Self {
        let dimension_line_y = match dim_type {
            LinearDimensionType::Horizontal => (first_point.y + second_point.y) / 2.0,
            LinearDimensionType::Vertical => (first_point.x + second_point.x) / 2.0,
            LinearDimensionType::Aligned => {
                (first_point.y + second_point.y) / 2.0
            }
            LinearDimensionType::Rotated(_) => {
                (first_point.y + second_point.y) / 2.0
            }
        };

        let dimension_line = Point::new(
            (first_point.x + second_point.x) / 2.0,
            dimension_line_y,
        );

        Self {
            dim_type,
            extension_lines: [first_point, second_point],
            text,
            text_height: 2.5,
            arrow_size: 2.5,
            extension_extend: 3.0,
            gap_size: 1.5,
            tolerance: None,
            dimension_line,
            is_left_to_right: true,
        }
    }

    pub fn with_tolerance(mut self, tolerance: DimensionTolerance) -> Self {
        self.tolerance = Some(tolerance);
        self
    }

    pub fn with_text_height(mut self, height: f64) -> Self {
        self.text_height = height;
        self
    }

    pub fn with_arrow_size(mut self, size: f64) -> Self {
        self.arrow_size = size;
        self
    }

    pub fn measurement(&self) -> f64 {
        self.extension_lines[0].distance_to(self.extension_lines[1])
    }

    pub fn generate_geometry(&self) -> DimensionGeometry {
        let p1 = self.extension_lines[0];
        let p2 = self.extension_lines[1];

        let dim_line_start = Point::new(
            p1.x.min(p2.x) - self.extension_extend,
            self.dimension_line.y,
        );
        let dim_line_end = Point::new(
            p1.x.max(p2.x) + self.extension_extend,
            self.dimension_line.y,
        );

        let mut geometry = DimensionGeometry::default();

        geometry.extension_lines = vec![
            Line::new(
                Point::new(p1.x, p1.y - self.extension_extend),
                Point::new(p1.x, p1.y + self.gap_size),
            ),
            Line::new(
                Point::new(p2.x, p2.y - self.extension_extend),
                Point::new(p2.x, p2.y + self.gap_size),
            ),
        ];

        geometry.dimension_line = Line::new(dim_line_start, dim_line_end);

        let text_pos = Point::new(
            self.dimension_line.x,
            self.dimension_line.y + self.text_height / 2.0,
        );
        geometry.text_position = text_pos;

        geometry.arrows = vec![
            Arrow::new(dim_line_start, self.arrow_size, ArrowDirection::Right),
            Arrow::new(dim_line_end, self.arrow_size, ArrowDirection::Left),
        ];

        geometry
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum AngularDimensionType {
    ThreePoint,
    TwoLine,
    ArcCenter,
}

#[derive(Debug, Clone)]
pub struct AngularDimension {
    pub dim_type: AngularDimensionType,
    pub center: Point,
    pub first_point: Point,
    pub second_point: Point,
    pub arc_point: Point,
    pub text: String,
    pub text_height: f64,
    pub arrow_size: f64,
    pub arc_radius: f64,
    pub start_angle: f64,
    pub end_angle: f64,
    pub is_counter_clockwise: bool,
    pub tolerance: Option<DimensionTolerance>,
}

impl AngularDimension {
    pub fn three_point(
        center: Point,
        first_point: Point,
        second_point: Point,
        text: String,
    ) -> Self {
        let start_angle = center.angle_to(first_point);
        let end_angle = center.angle_to(second_point);
        let arc_radius = center.distance_to(first_point);

        let arc_mid_angle = if end_angle >= start_angle {
            if end_angle - start_angle <= PI {
                (start_angle + end_angle) / 2.0
            } else {
                start_angle + (end_angle - start_angle) / 2.0 + PI
            }
        } else {
            if start_angle - end_angle > PI {
                (end_angle + start_angle) / 2.0 + PI
            } else {
                (start_angle + end_angle) / 2.0
            }
        };

        let arc_point = Point::new(
            center.x + arc_radius * arc_mid_angle.cos(),
            center.y + arc_radius * arc_mid_angle.sin(),
        );

        let angle_measurement = if end_angle >= start_angle {
            if end_angle - start_angle <= PI {
                end_angle - start_angle
            } else {
                2.0 * PI - (end_angle - start_angle)
            }
        } else {
            if start_angle - end_angle > PI {
                start_angle - end_angle
            } else {
                2.0 * PI - (start_angle - end_angle)
            }
        };

        let is_counter_clockwise = angle_measurement <= PI;

        Self {
            dim_type: AngularDimensionType::ThreePoint,
            center,
            first_point,
            second_point,
            arc_point,
            text,
            text_height: 2.5,
            arrow_size: 2.5,
            arc_radius,
            start_angle,
            end_angle,
            is_counter_clockwise,
            tolerance: None,
        }
    }

    pub fn two_line(
        vertex: Point,
        first_line_end: Point,
        second_line_end: Point,
        arc_radius: f64,
        text: String,
    ) -> Self {
        let start_angle = vertex.angle_to(first_line_end);
        let end_angle = vertex.angle_to(second_line_end);

        let arc_mid_angle = if end_angle >= start_angle {
            if end_angle - start_angle <= PI {
                (start_angle + end_angle) / 2.0
            } else {
                start_angle + (end_angle - start_angle) / 2.0 + PI
            }
        } else {
            if start_angle - end_angle > PI {
                (end_angle + start_angle) / 2.0 + PI
            } else {
                (start_angle + end_angle) / 2.0
            }
        };

        let arc_point = Point::new(
            vertex.x + arc_radius * arc_mid_angle.cos(),
            vertex.y + arc_radius * arc_mid_angle.sin(),
        );

        let angle_measurement = if end_angle >= start_angle {
            if end_angle - start_angle <= PI {
                end_angle - start_angle
            } else {
                2.0 * PI - (end_angle - start_angle)
            }
        } else {
            if start_angle - end_angle > PI {
                start_angle - end_angle
            } else {
                2.0 * PI - (start_angle - end_angle)
            }
        };

        let is_counter_clockwise = angle_measurement <= PI;

        Self {
            dim_type: AngularDimensionType::TwoLine,
            center: vertex,
            first_point: first_line_end,
            second_point: second_line_end,
            arc_point,
            text,
            text_height: 2.5,
            arrow_size: 2.5,
            arc_radius,
            start_angle,
            end_angle,
            is_counter_clockwise,
            tolerance: None,
        }
    }

    pub fn arc_center(
        center: Point,
        radius: f64,
        start_angle: f64,
        end_angle: f64,
        text: String,
    ) -> Self {
        let arc_radius = radius;

        let arc_mid_angle = if end_angle >= start_angle {
            if end_angle - start_angle <= PI {
                (start_angle + end_angle) / 2.0
            } else {
                start_angle + (end_angle - start_angle) / 2.0 + PI
            }
        } else {
            if start_angle - end_angle > PI {
                (end_angle + start_angle) / 2.0 + PI
            } else {
                (start_angle + end_angle) / 2.0
            }
        };

        let arc_point = Point::new(
            center.x + arc_radius * arc_mid_angle.cos(),
            center.y + arc_radius * arc_mid_angle.sin(),
        );

        let first_point = Point::new(
            center.x + arc_radius * start_angle.cos(),
            center.y + arc_radius * start_angle.sin(),
        );

        let second_point = Point::new(
            center.x + arc_radius * end_angle.cos(),
            center.y + arc_radius * end_angle.sin(),
        );

        let angle_measurement = if end_angle >= start_angle {
            if end_angle - start_angle <= PI {
                end_angle - start_angle
            } else {
                2.0 * PI - (end_angle - start_angle)
            }
        } else {
            if start_angle - end_angle > PI {
                start_angle - end_angle
            } else {
                2.0 * PI - (start_angle - end_angle)
            }
        };

        let is_counter_clockwise = angle_measurement <= PI;

        Self {
            dim_type: AngularDimensionType::ArcCenter,
            center,
            first_point,
            second_point,
            arc_point,
            text,
            text_height: 2.5,
            arrow_size: 2.5,
            arc_radius,
            start_angle,
            end_angle,
            is_counter_clockwise,
            tolerance: None,
        }
    }

    pub fn with_tolerance(mut self, tolerance: DimensionTolerance) -> Self {
        self.tolerance = Some(tolerance);
        self
    }

    pub fn measurement(&self) -> f64 {
        let angle = if self.is_counter_clockwise {
            let angle = self.end_angle - self.start_angle;
            if angle < 0.0 { angle + 2.0 * PI } else { angle }
        } else {
            let angle = self.start_angle - self.end_angle;
            if angle < 0.0 { angle + 2.0 * PI } else { angle }
        };
        angle * 180.0 / PI
    }

    pub fn generate_geometry(&self) -> DimensionGeometry {
        let mut geometry = DimensionGeometry::default();

        geometry.arc = Some(Arc {
            center: self.center,
            radius: self.arc_radius,
            start_angle: self.start_angle,
            end_angle: self.end_angle,
            is_counter_clockwise: self.is_counter_clockwise,
        });

        let text_pos = self.arc_point;
        geometry.text_position = text_pos;

        let arrow1 = Point::new(
            self.center.x + self.arc_radius * self.start_angle.cos(),
            self.center.y + self.arc_radius * self.start_angle.sin(),
        );
        let arrow2 = Point::new(
            self.center.x + self.arc_radius * self.end_angle.cos(),
            self.center.y + self.arc_radius * self.end_angle.sin(),
        );

        geometry.arrows = vec![
            Arrow::new(arrow1, self.arrow_size, self.start_angle + PI / 2.0),
            Arrow::new(arrow2, self.arrow_size, self.end_angle - PI / 2.0),
        ];

        geometry
    }
}

#[derive(Debug, Clone)]
pub struct RadialDimension {
    pub center: Point,
    pub arc_point: Point,
    pub radius: f64,
    pub text: String,
    pub text_height: f64,
    pub arrow_size: f64,
    pub leader_length: f64,
    pub is_diameter: bool,
    pub tolerance: Option<DimensionTolerance>,
}

impl RadialDimension {
    pub fn new(
        center: Point,
        arc_point: Point,
        text: String,
        is_diameter: bool,
    ) -> Self {
        let radius = center.distance_to(arc_point);

        Self {
            center,
            arc_point,
            radius,
            text,
            text_height: 2.5,
            arrow_size: 2.5,
            leader_length: radius * 0.3,
            is_diameter,
            tolerance: None,
        }
    }

    pub fn with_tolerance(mut self, tolerance: DimensionTolerance) -> Self {
        self.tolerance = Some(tolerance);
        self
    }

    pub fn measurement(&self) -> f64 {
        if self.is_diameter {
            self.radius * 2.0
        } else {
            self.radius
        }
    }

    pub fn generate_geometry(&self) -> DimensionGeometry {
        let mut geometry = DimensionGeometry::default();

        let angle = self.center.angle_to(self.arc_point);
        let leader_end = Point::new(
            self.arc_point.x - self.leader_length * angle.cos(),
            self.arc_point.y - self.leader_length * angle.sin(),
        );

        geometry.dimension_line = Line::new(self.center, leader_end);

        geometry.text_position = Point::new(
            self.arc_point.x + self.leader_length * angle.cos() * 1.5,
            self.arc_point.y + self.leader_length * angle.sin() * 1.5,
        );

        geometry.arrows = vec![Arrow::new(
            self.arc_point,
            self.arrow_size,
            angle + PI,
        )];

        if self.is_diameter {
            geometry.text = format!("Ø{}", self.text);
        } else {
            geometry.text = format!("R{}", self.text);
        }

        geometry
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum OrdinateDimensionType {
    XCoordinate,
    YCoordinate,
}

#[derive(Debug, Clone)]
pub struct OrdinateDimension {
    pub dim_type: OrdinateDimensionType,
    pub origin: Point,
    pub feature_point: Point,
    pub leader_length: f64,
    pub text: String,
    pub text_height: f64,
    pub arrow_size: f64,
    pub extension_extend: f64,
    pub tolerance: Option<DimensionTolerance>,
}

impl OrdinateDimension {
    pub fn new(
        dim_type: OrdinateDimensionType,
        origin: Point,
        feature_point: Point,
        text: String,
    ) -> Self {
        Self {
            dim_type,
            origin,
            feature_point,
            leader_length: 5.0,
            text,
            text_height: 2.5,
            arrow_size: 2.5,
            extension_extend: 3.0,
            tolerance: None,
        }
    }

    pub fn with_tolerance(mut self, tolerance: DimensionTolerance) -> Self {
        self.tolerance = Some(tolerance);
        self
    }

    pub fn measurement(&self) -> f64 {
        match self.dim_type {
            OrdinateDimensionType::XCoordinate => self.feature_point.x - self.origin.x,
            OrdinateDimensionType::YCoordinate => self.feature_point.y - self.origin.y,
        }
    }

    pub fn generate_geometry(&self) -> DimensionGeometry {
        let mut geometry = DimensionGeometry::default();

        match self.dim_type {
            OrdinateDimensionType::XCoordinate => {
                let text_y = self.feature_point.y + self.extension_extend / 2.0;
                let dim_line_end = Point::new(
                    self.feature_point.x + self.leader_length,
                    self.feature_point.y,
                );

                geometry.extension_lines = vec![Line::new(
                    Point::new(self.feature_point.x, self.feature_point.y - self.extension_extend),
                    Point::new(self.feature_point.x, self.feature_point.y + self.extension_extend),
                )];

                geometry.dimension_line = Line::new(
                    Point::new(self.origin.x, text_y),
                    dim_line_end,
                );

                geometry.text_position = Point::new(
                    dim_line_end.x - self.leader_length / 2.0,
                    text_y + self.text_height / 2.0,
                );
            }
            OrdinateDimensionType::YCoordinate => {
                let text_x = self.feature_point.x + self.extension_extend / 2.0;
                let dim_line_end = Point::new(
                    self.feature_point.x,
                    self.feature_point.y + self.leader_length,
                );

                geometry.extension_lines = vec![Line::new(
                    Point::new(self.feature_point.x - self.extension_extend, self.feature_point.y),
                    Point::new(self.feature_point.x + self.extension_extend, self.feature_point.y),
                )];

                geometry.dimension_line = Line::new(
                    Point::new(text_x, self.origin.y),
                    dim_line_end,
                );

                geometry.text_position = Point::new(
                    text_x + self.text_height / 2.0,
                    dim_line_end.y - self.leader_length / 2.0,
                );
            }
        }

        geometry
    }
}

#[derive(Debug, Clone)]
pub struct ArcLengthDimension {
    pub arc: Arc,
    pub text: String,
    pub text_height: f64,
    pub arrow_size: f64,
    pub offset: f64,
    pub tolerance: Option<DimensionTolerance>,
}

impl ArcLengthDimension {
    pub fn new(arc: Arc, text: String) -> Self {
        Self {
            arc,
            text,
            text_height: 2.5,
            arrow_size: 2.5,
            offset: 5.0,
            tolerance: None,
        }
    }

    pub fn with_tolerance(mut self, tolerance: DimensionTolerance) -> Self {
        self.tolerance = Some(tolerance);
        self
    }

    pub fn measurement(&self) -> f64 {
        self.arc.length()
    }

    pub fn generate_geometry(&self) -> DimensionGeometry {
        let mut geometry = DimensionGeometry::default();

        let mid_angle = if self.arc.is_counter_clockwise {
            let angle = self.arc.end_angle - self.arc.start_angle;
            if angle < 0.0 {
                self.arc.start_angle + (angle + 2.0 * PI) / 2.0
            } else {
                self.arc.start_angle + angle / 2.0
            }
        } else {
            let angle = self.arc.start_angle - self.arc.end_angle;
            if angle < 0.0 {
                self.arc.end_angle + (angle + 2.0 * PI) / 2.0
            } else {
                self.arc.end_angle + angle / 2.0
            }
        };

        let dim_radius = self.arc.radius + self.offset;
        let text_pos = Point::new(
            self.arc.center.x + dim_radius * mid_angle.cos(),
            self.arc.center.y + dim_radius * mid_angle.sin(),
        );

        geometry.arc = Some(Arc {
            center: self.arc.center,
            radius: dim_radius,
            start_angle: self.arc.start_angle,
            end_angle: self.arc.end_angle,
            is_counter_clockwise: self.arc.is_counter_clockwise,
        });

        geometry.text_position = text_pos;

        geometry.text = format!("⌒{}", self.text);

        let arrow1 = Point::new(
            self.arc.center.x + dim_radius * self.arc.start_angle.cos(),
            self.arc.center.y + dim_radius * self.arc.start_angle.sin(),
        );
        let arrow2 = Point::new(
            self.arc.center.x + dim_radius * self.arc.end_angle.cos(),
            self.arc.center.y + dim_radius * self.arc.end_angle.sin(),
        );

        geometry.arrows = vec![
            Arrow::new(arrow1, self.arrow_size, self.arc.start_angle + PI / 2.0),
            Arrow::new(arrow2, self.arrow_size, self.arc.end_angle - PI / 2.0),
        ];

        geometry
    }
}

#[derive(Debug, Clone)]
pub struct Leader {
    pub start_point: Point,
    pub landing_point: Point,
    pub vertices: Vec<Point>,
    pub arrow_head: bool,
    pub arrow_size: f64,
    pub mtext: Option<MText>,
    pub slope: Option<f64>,
}

impl Leader {
    pub fn new(start_point: Point, landing_point: Point) -> Self {
        Self {
            start_point,
            landing_point,
            vertices: Vec::new(),
            arrow_head: true,
            arrow_size: 2.5,
            mtext: None,
            slope: None,
        }
    }

    pub fn with_vertices(mut self, vertices: Vec<Point>) -> Self {
        self.vertices = vertices;
        self
    }

    pub fn with_arrow(mut self, has_arrow: bool) -> Self {
        self.arrow_head = has_arrow;
        self
    }

    pub fn with_mtext(mut self, mtext: MText) -> Self {
        self.mtext = Some(mtext);
        self
    }

    pub fn generate_geometry(&self) -> DimensionGeometry {
        let mut geometry = DimensionGeometry::default();

        let mut path = Vec::new();
        path.push(self.start_point);
        path.extend(&self.vertices);
        path.push(self.landing_point);

        geometry.leader_path = Some(path);

        if self.arrow_head {
            let last_segment_angle = if path.len() >= 2 {
                let p1 = path[path.len() - 2];
                let p2 = path[path.len() - 1];
                p2.angle_to(p1)
            } else {
                0.0
            };

            geometry.arrows = vec![Arrow::new(
                self.start_point,
                self.arrow_size,
                last_segment_angle,
            )];
        }

        geometry.text_position = self.landing_point;

        geometry
    }
}

#[derive(Debug, Clone)]
pub struct Multileader {
    pub landing_point: Point,
    pub leaders: Vec<Leader>,
    pub block_reference: Option<BlockReference>,
    pub content: Option<MText>,
    pub landing_length: f64,
    pub arrow_size: f64,
    pub text_height: f64,
}

impl Multileader {
    pub fn new(landing_point: Point) -> Self {
        Self {
            landing_point,
            leaders: Vec::new(),
            block_reference: None,
            content: None,
            landing_length: 8.0,
            arrow_size: 2.5,
            text_height: 2.5,
        }
    }

    pub fn add_leader(mut self, leader: Leader) -> Self {
        self.leaders.push(leader);
        self
    }

    pub fn with_block(mut self, block_ref: BlockReference) -> Self {
        self.block_reference = Some(block_ref);
        self
    }

    pub fn with_content(mut self, content: MText) -> Self {
        self.content = Some(content);
        self
    }

    pub fn generate_geometry(&self) -> Vec<DimensionGeometry> {
        let mut geometries = Vec::new();

        for leader in &self.leaders {
            geometries.push(leader.generate_geometry());
        }

        geometries
    }
}

#[derive(Debug, Clone)]
pub struct BlockReference {
    pub block_name: String,
    pub position: Point,
    pub rotation: f64,
    pub scale: f64,
}

#[derive(Debug, Clone)]
pub struct MText {
    pub content: String,
    pub position: Point,
    pub height: f64,
    pub width: f64,
    pub attachment: TextAttachment,
    pub line_spacing: f64,
    pub rotation: f64,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum TextAttachment {
    TopLeft,
    TopCenter,
    TopRight,
    MiddleLeft,
    MiddleCenter,
    MiddleRight,
    BottomLeft,
    BottomCenter,
    BottomRight,
}

impl Default for MText {
    fn default() -> Self {
        Self {
            content: String::new(),
            position: Point::default(),
            height: 2.5,
            width: 0.0,
            attachment: TextAttachment::MiddleLeft,
            line_spacing: 1.5,
            rotation: 0.0,
        }
    }
}

#[derive(Debug, Clone)]
pub struct TextStyle {
    pub name: String,
    pub font: String,
    pub height: f64,
    pub width_factor: f64,
    pub oblique_angle: f64,
    pub is_upside_down: bool,
    pub is_backwards: bool,
}

impl Default for TextStyle {
    fn default() -> Self {
        Self {
            name: String::from("Standard"),
            font: String::from("simplex.shx"),
            height: 2.5,
            width_factor: 0.7,
            oblique_angle: 0.0,
            is_upside_down: false,
            is_backwards: false,
        }
    }
}

#[derive(Debug, Clone)]
pub struct DimensionTolerance {
    pub upper_deviation: f64,
    pub lower_deviation: f64,
    pub text_height_factor: f64,
    pub vertical_placement: ToleranceVerticalPlacement,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ToleranceVerticalPlacement {
    Middle,
    Top,
    Bottom,
}

#[derive(Debug, Clone)]
pub struct DimensionGeometry {
    pub extension_lines: Vec<Line>,
    pub dimension_line: Option<Line>,
    pub arc: Option<Arc>,
    pub arrows: Vec<Arrow>,
    pub text_position: Point,
    pub text: String,
    pub leader_path: Option<Vec<Point>>,
}

impl Default for DimensionGeometry {
    fn default() -> Self {
        Self {
            extension_lines: Vec::new(),
            dimension_line: None,
            arc: None,
            arrows: Vec::new(),
            text_position: Point::default(),
            text: String::new(),
            leader_path: None,
        }
    }
}

#[derive(Debug, Clone)]
pub struct Arrow {
    pub position: Point,
    pub size: f64,
    pub direction: f64,
}

impl Arrow {
    pub fn new(position: Point, size: f64, direction: f64) -> Self {
        Self {
            position,
            size,
            direction,
        }
    }
}
