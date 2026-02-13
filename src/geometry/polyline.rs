use crate::geometry::{Point, Line};
use crate::math::Vector2;
use std::fmt;
use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Polyline {
    pub vertices: Vec<Point>,
    pub is_closed: bool,
}

impl Polyline {
    #[inline]
    pub fn new() -> Self {
        Self {
            vertices: Vec::new(),
            is_closed: false,
        }
    }

    #[inline]
    pub fn with_capacity(capacity: usize) -> Self {
        Self {
            vertices: Vec::with_capacity(capacity),
            is_closed: false,
        }
    }

    #[inline]
    pub fn from_points(points: &[Point]) -> Self {
        Self {
            vertices: points.to_vec(),
            is_closed: false,
        }
    }

    #[inline]
    pub fn push(&mut self, point: Point) {
        self.vertices.push(point);
    }

    #[inline]
    pub fn vertex_count(&self) -> usize {
        self.vertices.len()
    }

    #[inline]
    pub fn is_empty(&self) -> bool {
        self.vertices.is_empty()
    }

    #[inline]
    pub fn close(&mut self) {
        if !self.is_closed && self.vertices.len() >= 3 {
            self.is_closed = true;
        }
    }

    #[inline]
    pub fn open(&mut self) {
        self.is_closed = false;
    }

    #[inline]
    pub fn segment(&self, index: usize) -> Option<Line> {
        if self.vertices.len() < 2 {
            return None;
        }
        
        let next_index = if self.is_closed {
            (index + 1) % self.vertices.len()
        } else if index + 1 < self.vertices.len() {
            index + 1
        } else {
            return None;
        };
        
        Some(Line::new(self.vertices[index], self.vertices[next_index]))
    }

    #[inline]
    pub fn total_length(&self) -> f64 {
        if self.vertices.len() < 2 {
            return 0.0;
        }

        let mut length = 0.0;
        for i in 0..self.vertices.len() - 1 {
            length += self.vertices[i].distance_to(&self.vertices[i + 1]);
        }
        
        if self.is_closed && self.vertices.len() >= 3 {
            length += self.vertices.last().unwrap().distance_to(&self.vertices[0]);
        }
        
        length
    }

    #[inline]
    pub fn bounding_box(&self) -> Option<(Point, Point)> {
        if self.vertices.is_empty() {
            return None;
        }

        let mut min_x = self.vertices[0].x;
        let mut max_x = self.vertices[0].x;
        let mut min_y = self.vertices[0].y;
        let mut max_y = self.vertices[0].y;
        let mut min_z = self.vertices[0].z;
        let mut max_z = self.vertices[0].z;

        for vertex in &self.vertices {
            min_x = min_x.min(vertex.x);
            max_x = max_x.max(vertex.x);
            min_y = min_y.min(vertex.y);
            max_y = max_y.max(vertex.y);
            min_z = min_z.min(vertex.z);
            max_z = max_z.max(vertex.z);
        }

        Some((
            Point::new(min_x, min_y, min_z),
            Point::new(max_x, max_y, max_z),
        ))
    }

    #[inline]
    pub fn centroid(&self) -> Option<Point> {
        if self.vertices.is_empty() {
            return None;
        }

        let mut cx = 0.0;
        let mut cy = 0.0;
        let mut cz = 0.0;

        for vertex in &self.vertices {
            cx += vertex.x;
            cy += vertex.y;
            cz += vertex.z;
        }

        let n = self.vertices.len() as f64;
        Some(Point::new(cx / n, cy / n, cz / n))
    }
}

impl Default for Polyline {
    fn default() -> Self {
        Self::new()
    }
}

impl fmt::Display for Polyline {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Polyline(vertices: {}, is_closed: {})",
            self.vertex_count(),
            self.is_closed
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_polyline_creation() {
        let polyline = Polyline::new();
        assert!(polyline.is_empty());
        assert_eq!(polyline.vertex_count(), 0);
    }

    #[test]
    fn test_polyline_push() {
        let mut polyline = Polyline::new();
        polyline.push(Point::new(0.0, 0.0, 0.0));
        polyline.push(Point::new(1.0, 0.0, 0.0));
        polyline.push(Point::new(1.0, 1.0, 0.0));
        
        assert_eq!(polyline.vertex_count(), 3);
    }

    #[test]
    fn test_polyline_length() {
        let mut polyline = Polyline::new();
        polyline.push(Point::new(0.0, 0.0, 0.0));
        polyline.push(Point::new(3.0, 0.0, 0.0));
        polyline.push(Point::new(3.0, 4.0, 0.0));
        
        assert!((polyline.total_length() - 7.0).abs() < 1e-10);
    }

    #[test]
    fn test_polyline_close() {
        let mut polyline = Polyline::new();
        polyline.push(Point::new(0.0, 0.0, 0.0));
        polyline.push(Point::new(1.0, 0.0, 0.0));
        polyline.push(Point::new(1.0, 1.0, 0.0));
        polyline.close();
        
        assert!(polyline.is_closed);
    }
}
