use std::ops::Add;

#[derive(Debug, Default, Clone)]
pub struct Vec3 {
    x: f64,
    y: f64,
    z: f64,
}

impl Vec3 {
    pub fn new(x: f64, y: f64, z: f64) -> Self {
        Vec3 { x, y, z }
    }

    pub fn x(&mut self, val: f64) {
        self.x = val;
    }

    pub fn y(&mut self, val: f64) {
        self.y = val;
    }

    pub fn z(&mut self, val: f64) {
        self.z = val;
    }

    pub fn mul(&self, val: f64) -> Vec3 {
        let x = self.x * val;
        let y = self.y * val;
        let z = self.z * val;

        Vec3 { x, y, z }
    }
}

impl Add for Vec3 {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        }
    }
}

#[derive(Debug, Default)]
pub struct Ray {
    pub origin: Vec3,
    pub direction: Vec3,
}

impl Ray {
    pub fn new(origin: Vec3, direction: Vec3) -> Self {
        Ray { origin, direction }
    }

    pub fn at(&self, t: f64) -> Vec3 {
        let multiplied = self.direction.mul(t);
        self.origin.clone() + multiplied
    }
}

#[derive(Debug, Default)]
pub struct Color {
    r: f64,
    g: f64,
    b: f64,
}

impl Color {
    pub fn new(r: f64, g: f64, b: f64) -> Self {
        Color { r, g, b }
    }

    pub fn r(&mut self, val: f64) {
        self.r = val;
    }

    pub fn g(&mut self, val: f64) {
        self.g = val;
    }

    pub fn b(&mut self, val: f64) {
        self.b = val;
    }

    pub fn write_color(&self) {
        let ir = (255.999 * self.r) as u32;
        let ig = (255.999 * self.g) as u32;
        let ib = (255.999 * self.b) as u32;
        println!("{ir} {ig} {ib}");
    }
}
