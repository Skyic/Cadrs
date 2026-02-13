use crate::geometry::Point;
use std::fmt;
use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum OrdinateDimensionType {
    X,
    Y,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct OrdinateDimension {
    pub dim_type: OrdinateDimensionType,
    pub origin: Point,
    pub feature_point: Point,
    pub leader_end: Point,
    pub text: String,
    pub text_height: f64,
    pub arrow_size: f64,
    pub leader_length: f64,
    pub measurement: f64,
    pub is_ordinate_from_xdatum: bool,
    pub datum_offset: f64,
    pub tolerance: Option<super::linear_dimension::DimensionTolerance>,
    pub layer_id: Option<String>,
}

impl OrdinateDimension {
    #[inline]
    pub fn x_dimension(
        origin: Point,
        feature_point: Point,
        leader_length: f64,
    ) -> Self {
        let measurement = feature_point.y - origin.y;
        let leader_end = Point::new(
            feature_point.x,
            origin.y + measurement,
            feature_point.z,
        );

        Self {
            dim_type: OrdinateDimensionType::X,
            origin,
            feature_point,
            leader_end,
            text: format!("{:.2}", measurement.abs()),
            text_height: 2.5,
            arrow_size: 2.5,
            leader_length,
            measurement: measurement.abs(),
            is_ordinate_from_xdatum: false,
            datum_offset: 0.0,
            tolerance: None,
            layer_id: None,
        }
    }

    #[inline]
    pub fn y_dimension(
        origin: Point,
        feature_point: Point,
        leader_length: f64,
    ) -> Self {
        let measurement = feature_point.x - origin.x;
        let leader_end = Point::new(
            origin.x + measurement,
            feature_point.y,
            feature_point.z,
        );

        Self {
            dim_type: OrdinateDimensionType::Y,
            origin,
            feature_point,
            leader_end,
            text: format!("{:.2}", measurement.abs()),
            text_height: 2.5,
            arrow_size: 2.5,
            leader_length,
            measurement: measurement.abs(),
            is_ordinate_from_xdatum: false,
            datum_offset: 0.0,
            tolerance: None,
            layer_id: None,
        }
    }

    #[inline]
    pub fn with_text(mut self, text: String) -> Self {
        self.text = text;
        self
    }

    #[inline]
    pub fn with_tolerance(mut self, upper: f64, lower: f64) -> Self {
        self.tolerance = Some(super::linear_dimension::DimensionTolerance {
            upper_tolerance: upper,
            lower_tolerance: lower,
            is_symmetrical: (upper - lower).abs() < 1e-6,
            decimal_places: 2,
        });
        self
    }

    #[inline]
    pub fn set_measurement(&mut self, value: f64) {
        self.measurement = value.abs();
        self.text = format!("{:.2}", value);
    }

    #[inline]
    pub fn definition_points(&self) -> [Point; 3] {
        [
            self.origin,
            self.feature_point,
            self.leader_end,
        ]
    }

    #[inline]
    pub fn is_valid(&self) -> bool {
        self.measurement >= 0.0 &&
        self.arrow_size > 0.0 &&
        self.text_height > 0.0
    }

    #[inline]
    pub fn bounding_box(&self) -> (Point, Point) {
        let mut min_x = self.origin.x.min(self.feature_point.x);
        let mut min_y = self.origin.y.min(self.feature_point.y);
        let mut max_x = self.origin.x.max(self.feature_point.x);
        let mut max_y = self.origin.y.max(self.feature_point.y);

        min_x -= self.leader_length;
        min_y -= self.leader_length;
        max_x += self.leader_length;
        max_y += self.leader_length;

        (
            Point::new(min_x, min_y, 0.0),
            Point::new(max_x, max_y, 0.0)
        )
    }
}

impl fmt::Display for OrdinateDimension {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "OrdinateDimension(type: {:?}, measurement: {}, text: '{}')",
            self.dim_type,
            self.measurement,
            self.text
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_x_dimension() {
        let origin = Point::origin();
        let feature = Point::new(0.0, 10.0, 0.0);
        let dim = OrdinateDimension::x_dimension(origin, feature, 5.0);
        
        assert!(dim.is_valid());
        assert!((dim.measurement - 10.0).abs() < 1e-6);
        assert_eq!(dim.dim_type, OrdinateDimensionType::X);
    }

    #[test]
    fn test_y_dimension() {
        let origin = Point::origin();
        let feature = Point::new(15.0, 0.0, 0.0);
        let dim = OrdinateDimension::y_dimension(origin, feature, 5.0);
        
        assert!((dim.measurement - 15.0).abs() < 1e-6);
        assert_eq!(dim.dim_type, OrdinateDimensionType::Y);
    }

    #[test]
    fn test_negative_coordinate() {
        let origin = Point::new(0.0, 0.0, 0.0);
        let feature = Point::new(-8.0, -5.0, 0.0);
        let dim = OrdinateDimension::x_dimension(origin, feature, 3.0);
        
        assert!((dim.measurement - 5.0).abs() < 1e-6);
    }
}
