use crate::math::Matrix3;
use crate::math::Vector2;
use std::fmt;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Transform2D {
    pub translation: Vector2,
    pub rotation: f64,
    pub scale: Vector2,
}

impl Transform2D {
    #[inline]
    pub fn new() -> Self {
        Self {
            translation: Vector2::zero(),
            rotation: 0.0,
            scale: Vector2::new(1.0, 1.0),
        }
    }

    #[inline]
    pub fn identity() -> Self {
        Self::new()
    }

    #[inline]
    pub fn from_translation(x: f64, y: f64) -> Self {
        Self {
            translation: Vector2::new(x, y),
            rotation: 0.0,
            scale: Vector2::new(1.0, 1.0),
        }
    }

    #[inline]
    pub fn from_rotation(angle: f64) -> Self {
        Self {
            translation: Vector2::zero(),
            rotation: angle,
            scale: Vector2::new(1.0, 1.0),
        }
    }

    #[inline]
    pub fn from_scale(sx: f64, sy: f64) -> Self {
        Self {
            translation: Vector2::zero(),
            rotation: 0.0,
            scale: Vector2::new(sx, sy),
        }
    }

    #[inline]
    pub fn to_matrix(&self) -> Matrix3 {
        let cos_r = self.rotation.cos();
        let sin_r = self.rotation.sin();
        let sx = self.scale.x;
        let sy = self.scale.y;
        let tx = self.translation.x;
        let ty = self.translation.y;

        let data = [
            [sx * cos_r, sx * sin_r, tx],
            [-sy * sin_r, sy * cos_r, ty],
            [0.0, 0.0, 1.0],
        ];

        Matrix3::new(data)
    }

    #[inline]
    pub fn apply(&self, point: &Vector2) -> Vector2 {
        let cos_r = self.rotation.cos();
        let sin_r = self.rotation.sin();
        
        let scaled = Vector2::new(
            point.x * self.scale.x,
            point.y * self.scale.y,
        );
        
        Vector2::new(
            scaled.x * cos_r - scaled.y * sin_r + self.translation.x,
            scaled.x * sin_r + scaled.y * cos_r + self.translation.y,
        )
    }

    #[inline]
    pub fn compose(&self, other: &Self) -> Self {
        let cos_r = self.rotation.cos();
        let sin_r = self.rotation.sin();
        
        let new_scale = Vector2::new(
            self.scale.x * other.scale.x,
            self.scale.y * other.scale.y,
        );
        
        let new_rotation = self.rotation + other.rotation;
        
        let tx = other.translation.x;
        let ty = other.translation.y;
        
        Self {
            translation: Vector2::new(
                tx * cos_r - ty * sin_r,
                tx * sin_r + ty * cos_r,
            ),
            rotation: new_rotation,
            scale: new_scale,
        }
    }
    
    #[inline]
    pub fn translate(mut self, x: f64, y: f64) -> Self {
        self.translation = Vector2::new(x, y);
        self
    }
    
    #[inline]
    pub fn rotate(mut self, angle: f64) -> Self {
        self.rotation = angle;
        self
    }
    
    #[inline]
    pub fn scale(mut self, sx: f64, sy: f64) -> Self {
        self.scale = Vector2::new(sx, sy);
        self
    }
    
    #[inline]
    pub fn inverse(&self) -> Self {
        let cos_r = self.rotation.cos();
        let sin_r = self.rotation.sin();
        
        let inv_scale_x = if self.scale.x.abs() > 1e-10 { 1.0 / self.scale.x } else { 0.0 };
        let inv_scale_y = if self.scale.y.abs() > 1e-10 { 1.0 / self.scale.y } else { 0.0 };
        
        let inv_rot_cos = cos_r;
        let inv_rot_sin = -sin_r;
        
        let inv_translation = Vector2::new(
            -self.translation.x * inv_scale_x * inv_rot_cos - self.translation.y * inv_scale_y * inv_rot_sin,
            self.translation.x * inv_scale_x * inv_rot_sin - self.translation.y * inv_scale_y * inv_rot_cos,
        );
        
        Self {
            translation: inv_translation,
            rotation: -self.rotation,
            scale: Vector2::new(inv_scale_x, inv_scale_y),
        }
    }
}

impl Default for Transform2D {
    fn default() -> Self {
        Self::new()
    }
}

impl fmt::Display for Transform2D {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Transform2D(translation: {}, rotation: {}, scale: {})", 
               self.translation, self.rotation, self.scale)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_transform2d_identity() {
        let t = Transform2D::identity();
        let p = Vector2::new(1.0, 2.0);
        assert_eq!(t.apply(&p), p);
    }

    #[test]
    fn test_transform2d_translation() {
        let t = Transform2D::from_translation(1.0, 2.0);
        let p = Vector2::new(0.0, 0.0);
        assert_eq!(t.apply(&p), Vector2::new(1.0, 2.0));
    }
}
