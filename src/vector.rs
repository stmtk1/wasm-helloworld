use serde::{ Serialize, Deserialize };

#[derive(Serialize, Deserialize, Clone)]
pub struct Vector {
    pub x: f64,
    pub y: f64,
}

impl Vector {
    pub fn new(x: f64, y: f64) -> Vector {
        Vector { x, y }
    }

    pub fn add<'a>(&'a mut self, other: &Vector) -> &'a Vector {
        self.x += other.x;
        self.y += other.y;
        self
    }

    pub fn sub<'a>(&'a mut self, other: &Vector) -> &'a Vector {
        self.x -= other.x;
        self.y -= other.y;
        self
    }

    pub fn mul<'a>(&'a mut self, scalar: f64) -> &'a Vector {
        self.x *= scalar;
        self.y *= scalar;
        self
    }

    pub fn div<'a>(&'a mut self, scalar: f64) -> &'a Vector {
        self.x /= scalar;
        self.y /= scalar;
        self
    }

    pub fn size(&self) -> f64 {
        (self.x * self.x + self.y * self.y).sqrt()
    }

    pub fn normalize<'a>(&'a mut self) -> &'a Vector {
        let size = self.size();
        if size > 0.0 {
            self.div(size);
        }
        self
    }

    pub fn distance(&self, other: &Vector) -> f64 {
        let x = self.x - other.x;
        let y = self.y - other.y;
        (x * x + y * y).sqrt()
    }
}
