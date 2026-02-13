use super::Point;
use std::collections::HashMap;
use std::any::Any;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct BoundingBox {
    pub min: Point,
    pub max: Point,
}

impl BoundingBox {
    #[inline]
    pub fn new(min: Point, max: Point) -> Self {
        Self { min, max }
    }

    #[inline]
    pub fn from_center_size(center: Point, size: (f64, f64, f64)) -> Self {
        let half_x = size.0 / 2.0;
        let half_y = size.1 / 2.0;
        let half_z = size.2 / 2.0;
        
        Self {
            min: Point::new(center.x - half_x, center.y - half_y, center.z - half_z),
            max: Point::new(center.x + half_x, center.y + half_y, center.z + half_z),
        }
    }

    #[inline]
    pub fn from_points(points: &[Point]) -> Option<Self> {
        if points.is_empty() {
            return None;
        }
        
        let mut min_x = f64::MAX;
        let mut max_x = f64::MIN;
        let mut min_y = f64::MAX;
        let mut max_y = f64::MIN;
        let mut min_z = f64::MAX;
        let mut max_z = f64::MIN;
        
        for point in points {
            min_x = min_x.min(point.x);
            max_x = max_x.max(point.x);
            min_y = min_y.min(point.y);
            max_y = max_y.max(point.y);
            min_z = min_z.min(point.z);
            max_z = max_z.max(point.z);
        }
        
        Some(Self {
            min: Point::new(min_x, min_y, min_z),
            max: Point::new(max_x, max_y, max_z),
        })
    }

    #[inline]
    pub fn invalid() -> Self {
        Self {
            min: Point::new(f64::MAX, f64::MAX, f64::MAX),
            max: Point::new(f64::MIN, f64::MIN, f64::MIN),
        }
    }

    #[inline]
    pub fn is_valid(&self) -> bool {
        self.min.x <= self.max.x && self.min.y <= self.max.y && self.min.z <= self.max.z
    }

    #[inline]
    pub fn center(&self) -> Point {
        Point::new(
            (self.min.x + self.max.x) / 2.0,
            (self.min.y + self.max.y) / 2.0,
            (self.min.z + self.max.z) / 2.0,
        )
    }

    #[inline]
    pub fn size(&self) -> (f64, f64, f64) {
        (
            self.max.x - self.min.x,
            self.max.y - self.min.y,
            self.max.z - self.min.z,
        )
    }

    #[inline]
    pub fn area(&self) -> f64 {
        let (w, h, _) = self.size();
        w * h
    }

    #[inline]
    pub fn volume(&self) -> f64 {
        let (w, h, d) = self.size();
        w * h * d
    }

    #[inline]
    pub fn contains(&self, point: &Point) -> bool {
        point.x >= self.min.x && point.x <= self.max.x &&
        point.y >= self.min.y && point.y <= self.max.y &&
        point.z >= self.min.z && point.z <= self.max.z
    }

    #[inline]
    pub fn contains_bbox(&self, other: &BoundingBox) -> bool {
        self.min.x <= other.min.x && self.max.x >= other.max.x &&
        self.min.y <= other.min.y && self.max.y >= other.max.y &&
        self.min.z <= other.min.z && self.max.z >= other.max.z
    }

    #[inline]
    pub fn intersects(&self, other: &BoundingBox) -> bool {
        self.min.x <= other.max.x && self.max.x >= other.min.x &&
        self.min.y <= other.max.y && self.max.y >= other.min.y &&
        self.min.z <= other.max.z && self.max.z >= other.min.z
    }

    #[inline]
    pub fn intersects_sphere(&self, center: Point, radius: f64) -> bool {
        let closest_x = self.min.x.max(center.x.min(self.max.x));
        let closest_y = self.min.y.max(center.y.min(self.max.y));
        let closest_z = self.min.z.max(center.z.min(self.max.z));
        
        let distance_x = center.x - closest_x;
        let distance_y = center.y - closest_y;
        let distance_z = center.z - closest_z;
        
        (distance_x * distance_x + distance_y * distance_y + distance_z * distance_z) <= (radius * radius)
    }

    #[inline]
    pub fn merge(&self, other: &BoundingBox) -> Self {
        Self {
            min: Point::new(
                self.min.x.min(other.min.x),
                self.min.y.min(other.min.y),
                self.min.z.min(other.min.z),
            ),
            max: Point::new(
                self.max.x.max(other.max.x),
                self.max.y.max(other.max.y),
                self.max.z.max(other.max.z),
            ),
        }
    }

    #[inline]
    pub fn expanded_by(&self, margin: f64) -> Self {
        Self {
            min: Point::new(
                self.min.x - margin,
                self.min.y - margin,
                self.min.z - margin,
            ),
            max: Point::new(
                self.max.x + margin,
                self.max.y + margin,
                self.max.z + margin,
            ),
        }
    }

    #[inline]
    pub fn quadrant(&self, index: usize) -> Self {
        let center = self.center();
        match index {
            0 => Self::new(self.min, center),
            1 => Self::new(
                Point::new(center.x, self.min.y, self.min.z),
                Point::new(self.max.x, center.y, center.z),
            ),
            2 => Self::new(
                Point::new(self.min.x, center.y, self.min.z),
                Point::new(center.x, self.max.y, center.z),
            ),
            3 => Self::new(center, self.max),
            _ => panic!("Invalid quadrant index"),
        }
    }
}

#[derive(Debug, Clone)]
pub struct SpatialObject {
    pub id: ObjectId,
    pub bounding_box: BoundingBox,
    pub data: Option<Box<dyn std::any::Any + Send + Sync>>,
}

impl SpatialObject {
    #[inline]
    pub fn new(id: ObjectId, bounding_box: BoundingBox) -> Self {
        Self {
            id,
            bounding_box,
            data: None,
        }
    }

    #[inline]
    pub fn with_data<T: Any + Send + Sync>(mut self, data: T) -> Self {
        self.data = Some(Box::new(data));
        self
    }
}

pub trait SpatialIndex {
    fn insert(&mut self, object: SpatialObject);
    fn remove(&mut self, id: &ObjectId) -> bool;
    fn query_point(&self, point: &Point) -> Vec<ObjectId>;
    fn query_bbox(&self, bbox: &BoundingBox) -> Vec<ObjectId>;
    fn query_radius(&self, center: &Point, radius: f64) -> Vec<ObjectId>;
    fn query_ray(&self, origin: &Point, direction: &Point) -> Vec<(ObjectId, f64)>;
    fn clear(&mut self);
    fn len(&self) -> usize;
    fn is_empty(&self) -> bool;
    fn rebuild(&mut self);
}

#[derive(Debug, Clone)]
pub struct QuadTreeNode {
    bounding_box: BoundingBox,
    objects: Vec<SpatialObject>,
    children: Option<[Box<QuadTreeNode>; 4]>,
    max_objects: usize,
    max_depth: usize,
    current_depth: usize,
    split_threshold: f64,
}

impl QuadTreeNode {
    #[inline]
    pub fn new(bounding_box: BoundingBox) -> Self {
        Self {
            bounding_box,
            objects: Vec::new(),
            children: None,
            max_objects: 8,
            max_depth: 8,
            current_depth: 0,
            split_threshold: 0.5,
        }
    }

    #[inline]
    pub fn with_params(
        bounding_box: BoundingBox,
        max_objects: usize,
        max_depth: usize,
    ) -> Self {
        Self {
            bounding_box,
            objects: Vec::new(),
            children: None,
            max_objects,
            max_depth,
            current_depth: 0,
            split_threshold: 0.5,
        }
    }

    #[inline]
    fn should_split(&self) -> bool {
        self.objects.len() > self.max_objects && self.current_depth < self.max_depth
    }

    #[inline]
    fn split(&mut self) {
        let center = self.bounding_box.center();
        let (width, height, _) = self.bounding_box.size();
        let half_width = width / 2.0;
        let half_height = height / 2.0;

        let child_boxes = [
            BoundingBox::new(
                self.bounding_box.min,
                Point::new(center.x, center.y, center.z),
            ),
            BoundingBox::new(
                Point::new(center.x, self.bounding_box.min.y, self.bounding_box.min.z),
                Point::new(self.bounding_box.max.x, center.y, center.z),
            ),
            BoundingBox::new(
                Point::new(self.bounding_box.min.x, center.y, self.bounding_box.min.z),
                Point::new(center.x, self.bounding_box.max.y, center.z),
            ),
            BoundingBox::new(center, self.bounding_box.max),
        ];

        self.children = Some([
            Box::new(QuadTreeNode::with_params(
                child_boxes[0],
                self.max_objects,
                self.max_depth,
            ).with_depth(self.current_depth + 1)),
            Box::new(QuadTreeNode::with_params(
                child_boxes[1],
                self.max_objects,
                self.max_depth,
            ).with_depth(self.current_depth + 1)),
            Box::new(QuadTreeNode::with_params(
                child_boxes[2],
                self.max_objects,
                self.max_depth,
            ).with_depth(self.current_depth + 1)),
            Box::new(QuadTreeNode::with_params(
                child_boxes[3],
                self.max_objects,
                self.max_depth,
            ).with_depth(self.current_depth + 1)),
        ]);

        let mut remaining_objects = Vec::new();
        for object in self.objects.drain(..) {
            let mut inserted = false;
            if let Some(ref mut children) = self.children {
                for child in children.iter_mut() {
                    if child.bounding_box.contains_bbox(&object.bounding_box) {
                        child.insert(object);
                        inserted = true;
                        break;
                    }
                }
            }
            if !inserted {
                remaining_objects.push(object);
            }
        }
        self.objects = remaining_objects;
    }

    #[inline]
    fn with_depth(mut self, depth: usize) -> Self {
        self.current_depth = depth;
        self
    }

    #[inline]
    fn get_quadrant(&self, bbox: &BoundingBox) -> Option<usize> {
        let center = self.bounding_box.center();
        let bbox_center = bbox.center();
        
        if bbox_center.x < center.x {
            if bbox_center.y < center.y {
                Some(0)
            } else {
                Some(2)
            }
        } else {
            if bbox_center.y < center.y {
                Some(1)
            } else {
                Some(3)
            }
        }
    }

    #[inline]
    pub fn insert(&mut self, object: SpatialObject) {
        if !self.bounding_box.intersects(&object.bounding_box) {
            return;
        }

        if self.children.is_some() {
            if let Some(ref mut children) = self.children {
                for child in children.iter_mut() {
                    if child.bounding_box.intersects(&object.bounding_box) {
                        child.insert(object);
                        return;
                    }
                }
            }
        }

        self.objects.push(object);

        if self.should_split() {
            self.split();
        }
    }

    #[inline]
    pub fn remove(&mut self, id: &ObjectId) -> bool {
        for i in 0..self.objects.len() {
            if self.objects[i].id == *id {
                self.objects.remove(i);
                return true;
            }
        }

        if let Some(ref mut children) = self.children {
            for child in children.iter_mut() {
                if child.remove(id) {
                    return true;
                }
            }
        }

        false
    }

    #[inline]
    pub fn query_point(&self, point: &Point) -> Vec<ObjectId> {
        let mut results = Vec::new();
        self.query_point_recursive(point, &mut results);
        results
    }

    #[inline]
    fn query_point_recursive(&self, point: &Point, results: &mut Vec<ObjectId>) {
        if !self.bounding_box.contains(point) {
            return;
        }

        for obj in &self.objects {
            if obj.bounding_box.contains(point) {
                results.push(obj.id.clone());
            }
        }

        if let Some(ref children) = self.children {
            for child in children.iter() {
                child.query_point_recursive(point, results);
            }
        }
    }

    #[inline]
    pub fn query_bbox(&self, bbox: &BoundingBox) -> Vec<ObjectId> {
        let mut results = Vec::new();
        self.query_bbox_recursive(bbox, &mut results);
        results
    }

    #[inline]
    fn query_bbox_recursive(&self, bbox: &BoundingBox, results: &mut Vec<ObjectId>) {
        if !self.bounding_box.intersects(bbox) {
            return;
        }

        for obj in &self.objects {
            if obj.bounding_box.intersects(bbox) {
                results.push(obj.id.clone());
            }
        }

        if let Some(ref children) = self.children {
            for child in children.iter() {
                child.query_bbox_recursive(bbox, results);
            }
        }
    }

    #[inline]
    pub fn query_radius(&self, center: &Point, radius: f64) -> Vec<ObjectId> {
        let mut results = Vec::new();
        let query_bbox = BoundingBox::from_center_size(
            *center,
            (radius * 2.0, radius * 2.0, radius * 2.0),
        ).unwrap();
        
        self.query_bbox_recursive(&query_bbox, &mut results);
        results.retain(|id| {
            self.find_object(id)
                .map(|obj| obj.bounding_box.intersects_sphere(*center, radius))
                .unwrap_or(false)
        });
        
        results
    }

    #[inline]
    fn find_object(&self, id: &ObjectId) -> Option<&SpatialObject> {
        for obj in &self.objects {
            if obj.id == *id {
                return Some(obj);
            }
        }

        if let Some(ref children) = self.children {
            for child in children.iter() {
                if let Some(obj) = child.find_object(id) {
                    return Some(obj);
                }
            }
        }

        None
    }

    #[inline]
    pub fn query_ray(&self, origin: &Point, direction: &Point) -> Vec<(ObjectId, f64)> {
        let mut results = Vec::new();
        self.query_ray_recursive(origin, direction, &mut results);
        results.sort_by(|a, b| a.1.partial_cmp(&b.1).unwrap());
        results
    }

    #[inline]
    fn query_ray_recursive(&self, origin: &Point, direction: &Point, results: &mut Vec<(ObjectId, f64)>) {
        if !self.bounding_box.intersects_sphere(*origin, 1.0) {
            return;
        }

        for obj in &self.objects {
            if let Some(t) = self.ray_bbox_intersection(origin, direction, &obj.bounding_box) {
                results.push((obj.id.clone(), t));
            }
        }

        if let Some(ref children) = self.children {
            for child in children.iter() {
                child.query_ray_recursive(origin, direction, results);
            }
        }
    }

    #[inline]
    fn ray_bbox_intersection(&self, origin: &Point, direction: &Point, bbox: &BoundingBox) -> Option<f64> {
        let mut tmin = f64::MIN;
        let mut tmax = f64::MAX;

        for i in 0..3 {
            let origin_coord = match i {
                0 => origin.x,
                1 => origin.y,
                _ => origin.z,
            };
            let dir_coord = match i {
                0 => direction.x,
                1 => direction.y,
                _ => direction.z,
            };
            let min_coord = match i {
                0 => bbox.min.x,
                1 => bbox.min.y,
                _ => bbox.min.z,
            };
            let max_coord = match i {
                0 => bbox.max.x,
                1 => bbox.max.y,
                _ => bbox.max.z,
            };

            if dir_coord.abs() < 1e-10 {
                if origin_coord < min_coord || origin_coord > max_coord {
                    return None;
                }
            } else {
                let t1 = (min_coord - origin_coord) / dir_coord;
                let t2 = (max_coord - origin_coord) / dir_coord;

                let (t1, t2) = if t1 < t2 { (t1, t2) } else { (t2, t1) };

                tmin = tmin.max(t1);
                tmax = tmax.min(t2);

                if tmin > tmax {
                    return None;
                }
            }
        }

        if tmax < 0.0 {
            return None;
        }

        Some(if tmin > 0.0 { tmin } else { tmax })
    }

    #[inline]
    pub fn clear(&mut self) {
        self.objects.clear();
        self.children = None;
    }

    #[inline]
    pub fn len(&self) -> usize {
        let mut count = self.objects.len();
        if let Some(ref children) = self.children {
            for child in children.iter() {
                count += child.len();
            }
        }
        count
    }

    #[inline]
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    #[inline]
    pub fn balance(&mut self) {
        if let Some(ref mut children) = self.children {
            for child in children.iter_mut() {
                child.balance();
            }
        }

        if self.children.is_some() && !self.objects.is_empty() {
            let objects = self.objects.drain(..).collect::<Vec<_>>();
            for object in objects {
                self.insert(object);
            }
        }
    }
}

#[derive(Debug, Clone)]
pub struct QuadTree {
    root: QuadTreeNode,
}

impl QuadTree {
    #[inline]
    pub fn new(bounding_box: BoundingBox) -> Self {
        Self {
            root: QuadTreeNode::new(bounding_box),
        }
    }

    #[inline]
    pub fn with_params(
        bounding_box: BoundingBox,
        max_objects: usize,
        max_depth: usize,
    ) -> Self {
        Self {
            root: QuadTreeNode::with_params(bounding_box, max_objects, max_depth),
        }
    }

    #[inline]
    pub fn insert(&mut self, object: SpatialObject) {
        self.root.insert(object);
    }

    #[inline]
    pub fn remove(&mut self, id: &ObjectId) -> bool {
        self.root.remove(id)
    }

    #[inline]
    pub fn query_point(&self, point: &Point) -> Vec<ObjectId> {
        self.root.query_point(point)
    }

    #[inline]
    pub fn query_bbox(&self, bbox: &BoundingBox) -> Vec<ObjectId> {
        self.root.query_bbox(bbox)
    }

    #[inline]
    pub fn query_radius(&self, center: &Point, radius: f64) -> Vec<ObjectId> {
        self.root.query_radius(center, radius)
    }

    #[inline]
    pub fn query_ray(&self, origin: &Point, direction: &Point) -> Vec<(ObjectId, f64)> {
        self.root.query_ray(origin, direction)
    }

    #[inline]
    pub fn clear(&mut self) {
        self.root.clear();
    }

    #[inline]
    pub fn len(&self) -> usize {
        self.root.len()
    }

    #[inline]
    pub fn is_empty(&self) -> bool {
        self.root.is_empty()
    }

    #[inline]
    pub fn balance(&mut self) {
        self.root.balance();
    }
}

impl SpatialIndex for QuadTree {
    #[inline]
    fn insert(&mut self, object: SpatialObject) {
        self.insert(object);
    }

    #[inline]
    fn remove(&mut self, id: &ObjectId) -> bool {
        self.remove(id)
    }

    #[inline]
    fn query_point(&self, point: &Point) -> Vec<ObjectId> {
        self.query_point(point)
    }

    #[inline]
    fn query_bbox(&self, bbox: &BoundingBox) -> Vec<ObjectId> {
        self.query_bbox(bbox)
    }

    #[inline]
    fn query_radius(&self, center: &Point, radius: f64) -> Vec<ObjectId> {
        self.query_radius(center, radius)
    }

    #[inline]
    fn query_ray(&self, origin: &Point, direction: &Point) -> Vec<(ObjectId, f64)> {
        self.query_ray(origin, direction)
    }

    #[inline]
    fn clear(&mut self) {
        self.clear();
    }

    #[inline]
    fn len(&self) -> usize {
        self.len()
    }

    #[inline]
    fn is_empty(&self) -> bool {
        self.is_empty()
    }

    #[inline]
    fn rebuild(&mut self) {
        self.clear();
    }
}

#[derive(Debug, Clone)]
pub struct BVHNode {
    bounding_box: BoundingBox,
    object_index: Option<usize>,
    left: Option<Box<BVHNode>>,
    right: Option<Box<BVHNode>>,
}

impl BVHNode {
    #[inline]
    pub fn new(object_index: Option<usize>, bounding_box: BoundingBox) -> Self {
        Self {
            bounding_box,
            object_index,
            left: None,
            right: None,
        }
    }

    #[inline]
    fn compute_bbox(objects: &[SpatialObject]) -> BoundingBox {
        if objects.is_empty() {
            return BoundingBox::invalid();
        }

        let mut bbox = objects[0].bounding_box;
        for obj in objects.iter().skip(1) {
            bbox = bbox.merge(&obj.bounding_box);
        }
        bbox
    }

    #[inline]
    fn split(objects: &[SpatialObject], axis: usize) -> (Vec<SpatialObject>, Vec<SpatialObject>) {
        if objects.len() <= 1 {
            return (objects.to_vec(), Vec::new());
        }

        let mut sorted_objects = objects.to_vec();
        sorted_objects.sort_by(|a, b| {
            let a_center = a.bounding_box.center();
            let b_center = b.bounding_box.center();
            let a_coord = match axis {
                0 => a_center.x,
                1 => a_center.y,
                _ => a_center.z,
            };
            let b_coord = match axis {
                0 => b_center.x,
                1 => b_center.y,
                _ => b_center.z,
            };
            a_coord.partial_cmp(&b_coord).unwrap()
        });

        let mid = sorted_objects.len() / 2;
        let right_objects = sorted_objects.split_off(mid);
        (sorted_objects, right_objects)
    }

    #[inline]
    pub fn build(objects: &[SpatialObject], depth: usize) -> Option<Box<Self>> {
        if objects.is_empty() {
            return None;
        }

        let bounding_box = Self::compute_bbox(objects);

        if objects.len() == 1 {
            return Some(Box::new(Self {
                bounding_box,
                object_index: Some(0),
                left: None,
                right: None,
            }));
        }

        let axis = depth % 3;
        let (left_objects, right_objects) = Self::split(objects, axis);

        let mut node = Box::new(Self {
            bounding_box,
            object_index: None,
            left: None,
            right: None,
        });

        if !left_objects.is_empty() {
            node.left = Self::build(&left_objects, depth + 1);
        }
        if !right_objects.is_empty() {
            node.right = Self::build(&right_objects, depth + 1);
        }

        Some(node)
    }

    #[inline]
    pub fn query_point(&self, point: &Point) -> Vec<ObjectId> {
        let mut results = Vec::new();
        self.query_point_recursive(point, &mut results);
        results
    }

    #[inline]
    fn query_point_recursive(&self, point: &Point, results: &mut Vec<ObjectId>) {
        if !self.bounding_box.contains(point) {
            return;
        }

        if let Some(idx) = self.object_index {
            results.push(ObjectId::new());
        }

        if let Some(ref left) = self.left {
            left.query_point_recursive(point, results);
        }
        if let Some(ref right) = self.right {
            right.query_point_recursive(point, results);
        }
    }

    #[inline]
    pub fn query_bbox(&self, bbox: &BoundingBox) -> Vec<ObjectId> {
        let mut results = Vec::new();
        self.query_bbox_recursive(bbox, &mut results);
        results
    }

    #[inline]
    fn query_bbox_recursive(&self, bbox: &BoundingBox, results: &mut Vec<ObjectId>) {
        if !self.bounding_box.intersects(bbox) {
            return;
        }

        if let Some(_) = self.object_index {
            results.push(ObjectId::new());
        }

        if let Some(ref left) = self.left {
            left.query_bbox_recursive(bbox, results);
        }
        if let Some(ref right) = self.right {
            right.query_bbox_recursive(bbox, results);
        }
    }

    #[inline]
    pub fn query_radius(&self, center: &Point, radius: f64) -> Vec<ObjectId> {
        let query_bbox = BoundingBox::from_center_size(
            *center,
            (radius * 2.0, radius * 2.0, radius * 2.0),
        ).unwrap();
        
        let mut results = Vec::new();
        self.query_bbox_recursive(&query_bbox, &mut results);
        results
    }

    #[inline]
    pub fn query_ray(&self, origin: &Point, direction: &Point) -> Vec<(ObjectId, f64)> {
        let mut results = Vec::new();
        self.query_ray_recursive(origin, direction, &mut results);
        results.sort_by(|a, b| a.1.partial_cmp(&b.1).unwrap());
        results
    }

    #[inline]
    fn query_ray_recursive(&self, origin: &Point, direction: &Point, results: &mut Vec<(ObjectId, f64)>) {
        if !self.bounding_box.intersects_sphere(*origin, 1.0) {
            return;
        }

        if let Some(_) = self.object_index {
            results.push((ObjectId::new(), 0.0));
        }

        if let Some(ref left) = self.left {
            left.query_ray_recursive(origin, direction, results);
        }
        if let Some(ref right) = self.right {
            right.query_ray_recursive(origin, direction, results);
        }
    }
}

#[derive(Debug, Clone)]
pub struct BVH {
    root: Option<Box<BVHNode>>,
    objects: Vec<SpatialObject>,
}

impl BVH {
    #[inline]
    pub fn new() -> Self {
        Self {
            root: None,
            objects: Vec::new(),
        }
    }

    #[inline]
    pub fn with_objects(objects: Vec<SpatialObject>) -> Self {
        let mut bvh = Self {
            root: None,
            objects,
        };
        bvh.build();
        bvh
    }

    #[inline]
    pub fn build(&mut self) {
        self.root = BVHNode::build(&self.objects, 0);
    }

    #[inline]
    pub fn insert(&mut self, object: SpatialObject) {
        self.objects.push(object);
        self.build();
    }

    #[inline]
    pub fn remove(&mut self, id: &ObjectId) -> bool {
        let original_len = self.objects.len();
        self.objects.retain(|obj| &obj.id != id);
        if self.objects.len() != original_len {
            self.build();
            return true;
        }
        false
    }

    #[inline]
    pub fn query_point(&self, point: &Point) -> Vec<ObjectId> {
        self.root.as_ref().map(|r| r.query_point(point)).unwrap_or_default()
    }

    #[inline]
    pub fn query_bbox(&self, bbox: &BoundingBox) -> Vec<ObjectId> {
        self.root.as_ref().map(|r| r.query_bbox(bbox)).unwrap_or_default()
    }

    #[inline]
    pub fn query_radius(&self, center: &Point, radius: f64) -> Vec<ObjectId> {
        self.root.as_ref().map(|r| r.query_radius(center, radius)).unwrap_or_default()
    }

    #[inline]
    pub fn query_ray(&self, origin: &Point, direction: &Point) -> Vec<(ObjectId, f64)> {
        self.root.as_ref().map(|r| r.query_ray(origin, direction)).unwrap_or_default()
    }

    #[inline]
    pub fn clear(&mut self) {
        self.objects.clear();
        self.root = None;
    }

    #[inline]
    pub fn len(&self) -> usize {
        self.objects.len()
    }

    #[inline]
    pub fn is_empty(&self) -> bool {
        self.objects.is_empty()
    }
}

impl SpatialIndex for BVH {
    #[inline]
    fn insert(&mut self, object: SpatialObject) {
        self.insert(object);
    }

    #[inline]
    fn remove(&mut self, id: &ObjectId) -> bool {
        self.remove(id)
    }

    #[inline]
    fn query_point(&self, point: &Point) -> Vec<ObjectId> {
        self.query_point(point)
    }

    #[inline]
    fn query_bbox(&self, bbox: &BoundingBox) -> Vec<ObjectId> {
        self.query_bbox(bbox)
    }

    #[inline]
    fn query_radius(&self, center: &Point, radius: f64) -> Vec<ObjectId> {
        self.query_radius(center, radius)
    }

    #[inline]
    fn query_ray(&self, origin: &Point, direction: &Point) -> Vec<(ObjectId, f64)> {
        self.query_ray(origin, direction)
    }

    #[inline]
    fn clear(&mut self) {
        self.clear();
    }

    #[inline]
    fn len(&self) -> usize {
        self.len()
    }

    #[inline]
    fn is_empty(&self) -> bool {
        self.is_empty()
    }

    #[inline]
    fn rebuild(&mut self) {
        self.build();
    }
}

pub struct SpatialIndexManager {
    quadtree: QuadTree,
    bvh: BVH,
    object_map: HashMap<ObjectId, BoundingBox>,
    use_bvh: bool,
}

impl SpatialIndexManager {
    #[inline]
    pub fn new(bounding_box: BoundingBox) -> Self {
        Self {
            quadtree: QuadTree::new(bounding_box),
            bvh: BVH::new(),
            object_map: HashMap::new(),
            use_bvh: false,
        }
    }

    #[inline]
    pub fn set_use_bvh(&mut self, use_bvh: bool) {
        self.use_bvh = use_bvh;
    }

    #[inline]
    pub fn insert(&mut self, id: ObjectId, bounding_box: BoundingBox) {
        let spatial_object = SpatialObject::new(id.clone(), bounding_box);
        
        self.quadtree.insert(spatial_object.clone());
        self.bvh.insert(spatial_object);
        
        self.object_map.insert(id, bounding_box);
    }

    #[inline]
    pub fn remove(&mut self, id: &ObjectId) -> bool {
        self.quadtree.remove(id);
        self.bvh.remove(id);
        self.object_map.remove(id).is_some()
    }

    #[inline]
    pub fn update(&mut self, id: &ObjectId, bounding_box: BoundingBox) -> bool {
        if self.remove(id) {
            self.insert(id.clone(), bounding_box);
            return true;
        }
        false
    }

    #[inline]
    pub fn query_point(&self, point: &Point) -> Vec<ObjectId> {
        if self.use_bvh {
            self.bvh.query_point(point)
        } else {
            self.quadtree.query_point(point)
        }
    }

    #[inline]
    pub fn query_bbox(&self, bbox: &BoundingBox) -> Vec<ObjectId> {
        if self.use_bvh {
            self.bvh.query_bbox(bbox)
        } else {
            self.quadtree.query_bbox(bbox)
        }
    }

    #[inline]
    pub fn query_radius(&self, center: &Point, radius: f64) -> Vec<ObjectId> {
        if self.use_bvh {
            self.bvh.query_radius(center, radius)
        } else {
            self.quadtree.query_radius(center, radius)
        }
    }

    #[inline]
    pub fn query_ray(&self, origin: &Point, direction: &Point) -> Vec<(ObjectId, f64)> {
        if self.use_bvh {
            self.bvh.query_ray(origin, direction)
        } else {
            self.quadtree.query_ray(origin, direction)
        }
    }

    #[inline]
    pub fn clear(&mut self) {
        self.quadtree.clear();
        self.bvh.clear();
        self.object_map.clear();
    }

    #[inline]
    pub fn len(&self) -> usize {
        self.object_map.len()
    }

    #[inline]
    pub fn is_empty(&self) -> bool {
        self.object_map.is_empty()
    }

    #[inline]
    pub fn rebuild(&mut self) {
        self.quadtree.clear();
        self.bvh.clear();
        
        for (id, bbox) in &self.object_map {
            let spatial_object = SpatialObject::new(id.clone(), *bbox);
            self.quadtree.insert(spatial_object.clone());
            self.bvh.insert(spatial_object);
        }
    }

    #[inline]
    pub fn balance(&mut self) {
        self.quadtree.balance();
        self.bvh.build();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bounding_box_creation() {
        let min = Point::new(0.0, 0.0, 0.0);
        let max = Point::new(1.0, 1.0, 1.0);
        let bbox = BoundingBox::new(min, max);
        
        assert!(bbox.is_valid());
        assert_eq!(bbox.center(), Point::new(0.5, 0.5, 0.5));
        assert_eq!(bbox.size(), (1.0, 1.0, 1.0));
    }

    #[test]
    fn test_bounding_box_contains() {
        let bbox = BoundingBox::new(
            Point::new(0.0, 0.0, 0.0),
            Point::new(1.0, 1.0, 1.0),
        );
        
        assert!(bbox.contains(&Point::new(0.5, 0.5, 0.5)));
        assert!(!bbox.contains(&Point::new(1.5, 0.5, 0.5)));
        assert!(!bbox.contains(&Point::new(-0.5, 0.5, 0.5)));
    }

    #[test]
    fn test_bounding_box_intersects() {
        let bbox1 = BoundingBox::new(
            Point::new(0.0, 0.0, 0.0),
            Point::new(1.0, 1.0, 1.0),
        );
        
        let bbox2 = BoundingBox::new(
            Point::new(0.5, 0.5, 0.5),
            Point::new(1.5, 1.5, 1.5),
        );
        
        let bbox3 = BoundingBox::new(
            Point::new(2.0, 2.0, 2.0),
            Point::new(3.0, 3.0, 3.0),
        );
        
        assert!(bbox1.intersects(&bbox2));
        assert!(!bbox1.intersects(&bbox3));
    }

    #[test]
    fn test_bounding_box_merge() {
        let bbox1 = BoundingBox::new(
            Point::new(0.0, 0.0, 0.0),
            Point::new(1.0, 1.0, 1.0),
        );
        
        let bbox2 = BoundingBox::new(
            Point::new(2.0, 2.0, 2.0),
            Point::new(3.0, 3.0, 3.0),
        );
        
        let merged = bbox1.merge(&bbox2);
        
        assert_eq!(merged.min, Point::new(0.0, 0.0, 0.0));
        assert_eq!(merged.max, Point::new(3.0, 3.0, 3.0));
    }

    #[test]
    fn test_quadtree_insert() {
        let bbox = BoundingBox::new(
            Point::new(0.0, 0.0, 0.0),
            Point::new(100.0, 100.0, 0.0),
        );
        
        let mut quadtree = QuadTree::new(bbox);
        
        for i in 0..10 {
            let object_bbox = BoundingBox::new(
                Point::new(i as f64 * 10.0, i as f64 * 10.0, 0.0),
                Point::new(i as f64 * 10.0 + 1.0, i as f64 * 10.0 + 1.0, 0.0),
            );
            
            let object = SpatialObject::new(ObjectId::new(), object_bbox);
            quadtree.insert(object);
        }
        
        assert_eq!(quadtree.len(), 10);
    }

    #[test]
    fn test_quadtree_point_query() {
        let bbox = BoundingBox::new(
            Point::new(0.0, 0.0, 0.0),
            Point::new(100.0, 100.0, 0.0),
        );
        
        let mut quadtree = QuadTree::new(bbox);
        
        let object_bbox = BoundingBox::new(
            Point::new(10.0, 10.0, 0.0),
            Point::new(20.0, 20.0, 0.0),
        );
        let id = ObjectId::new();
        let object = SpatialObject::new(id.clone(), object_bbox);
        quadtree.insert(object);
        
        let results = quadtree.query_point(&Point::new(15.0, 15.0, 0.0));
        
        assert!(results.contains(&id));
    }

    #[test]
    fn test_quadtree_bbox_query() {
        let bbox = BoundingBox::new(
            Point::new(0.0, 0.0, 0.0),
            Point::new(100.0, 100.0, 0.0),
        );
        
        let mut quadtree = QuadTree::new(bbox);
        
        for i in 0..5 {
            let object_bbox = BoundingBox::new(
                Point::new(i as f64 * 20.0, i as f64 * 20.0, 0.0),
                Point::new(i as f64 * 20.0 + 10.0, i as f64 * 20.0 + 10.0, 0.0),
            );
            let object = SpatialObject::new(ObjectId::new(), object_bbox);
            quadtree.insert(object);
        }
        
        let query_bbox = BoundingBox::new(
            Point::new(0.0, 0.0, 0.0),
            Point::new(50.0, 50.0, 0.0),
        );
        
        let results = quadtree.query_bbox(&query_bbox);
        
        assert!(results.len() > 0);
    }

    #[test]
    fn test_quadtree_remove() {
        let bbox = BoundingBox::new(
            Point::new(0.0, 0.0, 0.0),
            Point::new(100.0, 100.0, 0.0),
        );
        
        let mut quadtree = QuadTree::new(bbox);
        
        let object_bbox = BoundingBox::new(
            Point::new(10.0, 10.0, 0.0),
            Point::new(20.0, 20.0, 0.0),
        );
        let id = ObjectId::new();
        let object = SpatialObject::new(id.clone(), object_bbox);
        quadtree.insert(object);
        
        assert_eq!(quadtree.len(), 1);
        
        quadtree.remove(&id);
        
        assert_eq!(quadtree.len(), 0);
    }

    #[test]
    fn test_bvh_creation() {
        let objects = vec![
            SpatialObject::new(
                ObjectId::new(),
                BoundingBox::new(Point::new(0.0, 0.0, 0.0), Point::new(1.0, 1.0, 1.0)),
            ),
            SpatialObject::new(
                ObjectId::new(),
                BoundingBox::new(Point::new(2.0, 2.0, 2.0), Point::new(3.0, 3.0, 3.0)),
            ),
        ];
        
        let bvh = BVH::with_objects(objects);
        
        assert_eq!(bvh.len(), 2);
        assert!(!bvh.is_empty());
    }

    #[test]
    fn test_bvh_query() {
        let objects = vec![
            SpatialObject::new(
                ObjectId::new(),
                BoundingBox::new(Point::new(0.0, 0.0, 0.0), Point::new(1.0, 1.0, 1.0)),
            ),
            SpatialObject::new(
                ObjectId::new(),
                BoundingBox::new(Point::new(2.0, 2.0, 2.0), Point::new(3.0, 3.0, 3.0)),
            ),
        ];
        
        let bvh = BVH::with_objects(objects);
        
        let query_bbox = BoundingBox::new(
            Point::new(0.0, 0.0, 0.0),
            Point::new(2.0, 2.0, 2.0),
        );
        
        let results = bvh.query_bbox(&query_bbox);
        
        assert!(results.len() > 0);
    }

    #[test]
    fn test_spatial_index_manager() {
        let bbox = BoundingBox::new(
            Point::new(0.0, 0.0, 0.0),
            Point::new(100.0, 100.0, 100.0),
        );
        
        let mut manager = SpatialIndexManager::new(bbox);
        
        for i in 0..5 {
            let object_bbox = BoundingBox::new(
                Point::new(i as f64 * 20.0, i as f64 * 20.0, i as f64 * 20.0),
                Point::new(i as f64 * 20.0 + 10.0, i as f64 * 20.0 + 10.0, i as f64 * 20.0 + 10.0),
            );
            manager.insert(ObjectId::new(), object_bbox);
        }
        
        assert_eq!(manager.len(), 5);
        
        manager.clear();
        
        assert_eq!(manager.len(), 0);
    }

    #[test]
    fn test_spatial_index_manager_update() {
        let bbox = BoundingBox::new(
            Point::new(0.0, 0.0, 0.0),
            Point::new(100.0, 100.0, 100.0),
        );
        
        let mut manager = SpatialIndexManager::new(bbox);
        
        let id = ObjectId::new();
        let old_bbox = BoundingBox::new(
            Point::new(0.0, 0.0, 0.0),
            Point::new(10.0, 10.0, 10.0),
        );
        manager.insert(id.clone(), old_bbox);
        
        let new_bbox = BoundingBox::new(
            Point::new(50.0, 50.0, 50.0),
            Point::new(60.0, 60.0, 60.0),
        );
        manager.update(&id, new_bbox);
        
        let query_result = manager.query_point(&Point::new(55.0, 55.0, 55.0));
        assert!(query_result.contains(&id));
        
        let old_query = manager.query_point(&Point::new(5.0, 5.0, 5.0));
        assert!(!old_query.contains(&id));
    }
}
