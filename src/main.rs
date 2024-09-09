#[derive(Debug, Clone, Copy)]
struct Vector3D {
    x: f64,
    y: f64,
    z: f64,
}

impl Vector3D {
    fn new(x: f64, y: f64, z: f64) -> Vector3D {
        Vector3D { x, y, z}
    }

    fn add(self, vector: Vector3D) -> Vector3D {
        Vector3D {
            x: self.x + vector.x,
            y: self.y + vector.y,
            z: self.z + vector.z,
        }
    }

    fn subtract(self, vector: Vector3D) -> Vector3D {
        Vector3D {
            x: self.x - vector.x,
            y: self.y - vector.y,
            z: self.z - vector.z,
        }
    }

    fn scale(self, scalar: f64) -> Vector3D {
        Vector3D {
            x: self.x * scalar,
            y: self.y * scalar,
            z: self.z * scalar,
        }
    }

    fn norm(&self) -> f64 {
        (self.x * self.x + self.y * self.y + self.z * self.z).sqrt()
    }

    fn normalize(self) -> Vector3D {
        let norm = self.norm();
        Vector3D {
            x: self.x / norm,
            y: self.y / norm,
            z: self.z / norm,
        }
    }
}

fn main() {
    println!("Hello, world!");
}
