use glam::Vec2;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Rect {
    pub min: Vec2,
    pub max: Vec2,
}

impl Rect {
    pub fn new(min: Vec2, max: Vec2) -> Self {
        Self { min, max }
    }

    pub fn from_center_size(center: Vec2, size: Vec2) -> Self {
        let half = size * 0.5;
        Self {
            min: center - half,
            max: center + half,
        }
    }

    pub fn width(&self) -> f32 {
        self.max.x - self.min.x
    }

    pub fn height(&self) -> f32 {
        self.max.y - self.min.y
    }

    pub fn size(&self) -> Vec2 {
        Vec2::new(self.width(), self.height())
    }

    pub fn center(&self) -> Vec2 {
        (self.min + self.max) * 0.5
    }

    pub fn intersects(&self, other: &Rect) -> bool {
        !(self.max.x <= other.min.x
            || self.min.x >= other.max.x
            || self.max.y <= other.min.y
            || self.min.y >= other.max.y)
    }

    pub fn contains_point(&self, point: Vec2) -> bool {
        point.x >= self.min.x
            && point.x <= self.max.x
            && point.y >= self.min.y
            && point.y <= self.max.y
    }

    pub fn contains_rect(&self, other: &Rect) -> bool {
        other.min.x >= self.min.x
            && other.max.x <= self.max.x
            && other.min.y >= self.min.y
            && other.max.y <= self.max.y
    }

    pub fn intersects_circle(&self, center: Vec2, radius: f32) -> bool {
        let closest = Vec2::new(
            self.min.x.max(center.x.min(self.max.x)),
            self.min.y.max(center.y.min(self.max.y)),
        );
        let distance = (center - closest).length();
        distance < radius
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rect_creation() {
        let rect = Rect::new(Vec2::new(0.0, 0.0), Vec2::new(10.0, 10.0));
        assert_eq!(rect.width(), 10.0);
        assert_eq!(rect.height(), 10.0);
        assert_eq!(rect.center(), Vec2::new(5.0, 5.0));
    }

    #[test]
    fn test_rect_from_center_size() {
        let rect = Rect::from_center_size(Vec2::new(5.0, 5.0), Vec2::new(10.0, 10.0));
        assert_eq!(rect.min, Vec2::new(0.0, 0.0));
        assert_eq!(rect.max, Vec2::new(10.0, 10.0));
    }

    #[test]
    fn test_intersects() {
        let r1 = Rect::new(Vec2::new(0.0, 0.0), Vec2::new(10.0, 10.0));
        let r2 = Rect::new(Vec2::new(5.0, 5.0), Vec2::new(15.0, 15.0));
        let r3 = Rect::new(Vec2::new(20.0, 20.0), Vec2::new(30.0, 30.0));

        assert!(r1.intersects(&r2));
        assert!(r2.intersects(&r1));
        assert!(!r1.intersects(&r3));
    }

    #[test]
    fn test_contains_point() {
        let rect = Rect::new(Vec2::new(0.0, 0.0), Vec2::new(10.0, 10.0));
        assert!(rect.contains_point(Vec2::new(5.0, 5.0)));
        assert!(rect.contains_point(Vec2::new(0.0, 0.0)));
        assert!(rect.contains_point(Vec2::new(10.0, 10.0)));
        assert!(!rect.contains_point(Vec2::new(-1.0, 5.0)));
        assert!(!rect.contains_point(Vec2::new(5.0, 11.0)));
    }

    #[test]
    fn test_contains_rect() {
        let outer = Rect::new(Vec2::new(0.0, 0.0), Vec2::new(10.0, 10.0));
        let inner = Rect::new(Vec2::new(2.0, 2.0), Vec2::new(8.0, 8.0));
        let overlapping = Rect::new(Vec2::new(5.0, 5.0), Vec2::new(15.0, 15.0));

        assert!(outer.contains_rect(&inner));
        assert!(!outer.contains_rect(&overlapping));
    }

    #[test]
    fn test_intersects_circle() {
        let rect = Rect::new(Vec2::new(0.0, 0.0), Vec2::new(10.0, 10.0));

        assert!(rect.intersects_circle(Vec2::new(5.0, 5.0), 1.0));
        assert!(rect.intersects_circle(Vec2::new(0.0, 0.0), 1.0));
        assert!(rect.intersects_circle(Vec2::new(5.0, 5.0), 10.0));
        assert!(!rect.intersects_circle(Vec2::new(-5.0, -5.0), 1.0));
    }
}
