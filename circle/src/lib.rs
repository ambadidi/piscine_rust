use std::f64::consts::PI;

#[derive(Debug, Clone, Copy)]
pub struct Circle {
	pub center : Point,
	pub radius : f64,
}

impl Circle {
    pub fn new(x: f64, y: f64, radius: f64) -> Circle {
        Circle { center: Point { x: x, y: y }, radius: radius }
    }
    pub fn area(&self) -> f64 {
        PI * self.radius * self.radius
    }
    pub fn diameter(&self) -> f64 {
        2.0 * self.radius
    }
    pub fn intersect(&self, circle2: &Circle) -> bool {
        let x = if self.center.x > circle2.center.x {
            self.center.x - circle2.center.x
        } else { circle2.center.x - self.center.x };
        let y = if self.center.y > circle2.center.y {
            self.center.y - circle2.center.y
        } else { circle2.center.y - self.center.y };
        if self.radius + circle2.radius >= f64::sqrt(x*x + y*y) { return true;}
        false
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Point {
    pub x : f64,
    pub y : f64,
}

impl Point {
    pub fn distance(self, other: &Point) -> f64 {
        let x = if self.x > other.x { self.x - other.x} else { other.x - self.x};
        let y = if self.y > other.y { self.y - other.y} else { other.y - self.y};
        return f64::sqrt(x*x + y*y);
    }
}