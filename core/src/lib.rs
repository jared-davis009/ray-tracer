use std::ops::{Add, Mul, Sub};

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

    pub fn div(&self, val: f64) -> Vec3 {
        let x = self.x / val;
        let y = self.y / val;
        let z = self.z / val;

        Vec3 { x, y, z }
    }

    pub fn unit_vector(&self) -> Vec3 {
        self.div(self.length())
    }

    pub fn length(&self) -> f64 {
        self.length_squared().sqrt()
    }

    pub fn length_squared(&self) -> f64 {
        self.x * self.x + self.y * self.y + self.z * self.z
    }
}

impl From<Point3> for Vec3 {
    fn from(point3: Point3) -> Self {
        Vec3::new(point3.x, point3.y, point3.z)
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
    pub origin: Point3,
    pub direction: Vec3,
}

impl Ray {
    pub fn new(origin: Point3, direction: Vec3) -> Self {
        Ray { origin, direction }
    }

    pub fn at(&self, t: f64) -> Point3 {
        let multiplied = self.direction.mul(t);
        self.origin.clone() + multiplied.into()
    }

    // See linear blend/interpolation
    pub fn ray_color(&self) -> Color {
        let unit_direction = self.direction.unit_vector();
        let a = 0.5 * (unit_direction.y + 1.0);
        Color::new(1.0, 1.0, 1.0).mul(1.0 - a) + Color::new(0.5, 0.7, 1.0).mul(a)
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

    pub fn mul(&self, val: f64) -> Color {
        let r = self.r * val;
        let g = self.g * val;
        let b = self.b * val;

        Color { r, g, b }
    }
}

impl Add for Color {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self {
            r: self.r + other.r,
            g: self.g + other.g,
            b: self.b + other.b,
        }
    }
}

#[derive(Debug, Default, Clone)]
pub struct Point3 {
    x: f64,
    y: f64,
    z: f64,
}

impl Point3 {
    pub fn new(x: f64, y: f64, z: f64) -> Self {
        Point3 { x, y, z }
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

    pub fn mul(&self, val: f64) -> Point3 {
        let x = self.x * val;
        let y = self.y * val;
        let z = self.z * val;

        Point3 { x, y, z }
    }

    pub fn add(&self, val: f64) -> Point3 {
        let x = self.x + val;
        let y = self.y + val;
        let z = self.z + val;

        Point3 { x, y, z }
    }
}

impl From<Vec3> for Point3 {
    fn from(vec3: Vec3) -> Self {
        Point3::new(vec3.x, vec3.y, vec3.z)
    }
}

impl Add for Point3 {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        }
    }
}

impl Sub for Point3 {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        Self {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
        }
    }
}

impl Mul for Point3 {
    type Output = Self;

    fn mul(self, other: Self) -> Self {
        Self {
            x: self.x * other.x,
            y: self.y * other.y,
            z: self.z * other.z,
        }
    }
}
