use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Wire {
    pub id: String,
    pub points: Vec<Point>,
    pub net_id: Option<String>,
    pub width: f64,
    pub color: String,
    pub selected: bool,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Point {
    pub x: f64,
    pub y: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Junction {
    pub id: String,
    pub x: f64,
    pub y: f64,
    pub connected_wires: Vec<String>,
}

impl Wire {
    pub fn new(start: Point, end: Point) -> Self {
        Self {
            id: Uuid::new_v4().to_string(),
            points: vec![start, end],
            net_id: None,
            width: 1.0,
            color: "#000000".to_string(),
            selected: false,
        }
    }

    pub fn add_point(&mut self, point: Point) {
        self.points.push(point);
    }

    pub fn insert_point(&mut self, index: usize, point: Point) {
        if index <= self.points.len() {
            self.points.insert(index, point);
        }
    }

    pub fn remove_point(&mut self, index: usize) -> Option<Point> {
        if index < self.points.len() && self.points.len() > 2 {
            Some(self.points.remove(index))
        } else {
            None
        }
    }

    pub fn get_segments(&self) -> Vec<(Point, Point)> {
        let mut segments = Vec::new();
        for i in 0..self.points.len() - 1 {
            segments.push((self.points[i], self.points[i + 1]));
        }
        segments
    }

    pub fn get_bounding_box(&self) -> (f64, f64, f64, f64) {
        let mut min_x = f64::MAX;
        let mut min_y = f64::MAX;
        let mut max_x = f64::MIN;
        let mut max_y = f64::MIN;

        for point in &self.points {
            min_x = min_x.min(point.x);
            min_y = min_y.min(point.y);
            max_x = max_x.max(point.x);
            max_y = max_y.max(point.y);
        }

        (min_x, min_y, max_x, max_y)
    }

    pub fn hit_test(&self, x: f64, y: f64, tolerance: f64) -> bool {
        for (start, end) in self.get_segments() {
            if point_to_segment_distance(Point { x, y }, start, end) <= tolerance {
                return true;
            }
        }
        false
    }

    pub fn split_at_point(&self, point: Point, tolerance: f64) -> Option<(Wire, Wire)> {
        for (i, (start, end)) in self.get_segments().iter().enumerate() {
            if point_to_segment_distance(point, *start, *end) <= tolerance {
                let mut wire1 = self.clone();
                let mut wire2 = self.clone();
                
                wire1.id = Uuid::new_v4().to_string();
                wire2.id = Uuid::new_v4().to_string();
                
                wire1.points = self.points[0..=i].to_vec();
                wire1.points.push(point);
                
                wire2.points = vec![point];
                wire2.points.extend_from_slice(&self.points[i + 1..]);
                
                return Some((wire1, wire2));
            }
        }
        None
    }
}

impl Point {
    pub fn new(x: f64, y: f64) -> Self {
        Self { x, y }
    }

    pub fn distance_to(&self, other: &Point) -> f64 {
        ((self.x - other.x).powi(2) + (self.y - other.y).powi(2)).sqrt()
    }

    pub fn manhattan_distance_to(&self, other: &Point) -> f64 {
        (self.x - other.x).abs() + (self.y - other.y).abs()
    }
}

// Helper function to calculate distance from point to line segment
fn point_to_segment_distance(point: Point, start: Point, end: Point) -> f64 {
    let dx = end.x - start.x;
    let dy = end.y - start.y;
    
    if dx == 0.0 && dy == 0.0 {
        return point.distance_to(&start);
    }
    
    let t = ((point.x - start.x) * dx + (point.y - start.y) * dy) / (dx * dx + dy * dy);
    let t = t.clamp(0.0, 1.0);
    
    let closest = Point {
        x: start.x + t * dx,
        y: start.y + t * dy,
    };
    
    point.distance_to(&closest)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_wire_creation() {
        let start = Point::new(0.0, 0.0);
        let end = Point::new(100.0, 100.0);
        let wire = Wire::new(start, end);
        
        assert_eq!(wire.points.len(), 2);
        assert_eq!(wire.points[0].x, 0.0);
        assert_eq!(wire.points[1].x, 100.0);
    }

    #[test]
    fn test_wire_segments() {
        let mut wire = Wire::new(Point::new(0.0, 0.0), Point::new(100.0, 0.0));
        wire.add_point(Point::new(100.0, 100.0));
        
        let segments = wire.get_segments();
        assert_eq!(segments.len(), 2);
    }

    #[test]
    fn test_point_distance() {
        let p1 = Point::new(0.0, 0.0);
        let p2 = Point::new(3.0, 4.0);
        assert_eq!(p1.distance_to(&p2), 5.0);
        assert_eq!(p1.manhattan_distance_to(&p2), 7.0);
    }

    #[test]
    fn test_hit_test() {
        let wire = Wire::new(Point::new(0.0, 0.0), Point::new(100.0, 0.0));
        assert!(wire.hit_test(50.0, 0.0, 1.0));
        assert!(wire.hit_test(50.0, 1.0, 2.0));
        assert!(!wire.hit_test(50.0, 10.0, 1.0));
    }
}