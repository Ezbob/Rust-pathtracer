use std::fs::File;
use std::io::prelude::*;
use std::ops::{Add, Neg, Index};

fn to_u8(val: f64) -> u8 { return val as u8; }

struct Vec3 {
    values: [f64; 3]
}

impl Index<usize> for Vec3 {
    type Output = f64;

    fn index(&self, index: usize) -> &f64 {
        return &self.values[index];
    }
}

impl Add for Vec3 {
    type Output = Vec3;

    fn add(self, other: Vec3) -> Vec3 {
        return Vec3 {
            values: [
                self.values[0] + other.values[0],
                self.values[1] + other.values[1],
                self.values[2] + other.values[2]
            ]
        };
    }
}


impl Neg for Vec3 {
    type Output = Vec3;

    fn neg(self) -> Vec3 {
        return Vec3 {
            values: [self.values[0], self.values[1], self.values[2]]
        };
    }
}

impl Vec3 {
    fn new(e0: f64, e1: f64, e2: f64) -> Vec3 {
        return Vec3 { values: [e0, e1, e2] };
    }

    fn x(&self) -> f64 { return self.values[0]; }
    fn y(&self) -> f64 { return self.values[1]; }
    fn z(&self) -> f64 { return self.values[2]; }
    fn r(&self) -> f64 { return self.values[0]; }
    fn g(&self) -> f64 { return self.values[1]; }
    fn b(&self) -> f64 { return self.values[2]; }
    
    fn length(&self) -> f64 {
        return (self.values[0].powi(2) +
                self.values[1].powi(2) +
                self.values[2].powi(2)).sqrt()
    }

    fn squared_length(&self) -> f64 {
        return self.values[0].powi(2) +
               self.values[1].powi(2) +
               self.values[2].powi(2)
    }
}


#[cfg(test)]
mod vec_tests {

    use ::Vec3;

    #[test]
    fn can_create_vec3_with_new() {
        let v: Vec3 = Vec3::new(1., 1., 1.);
        assert_eq!(v.values[0], 1.);
        assert!(v.values[1] == v.values[2]);
        assert!(v.values[0] == v.values[1]);
    }

    #[test]
    fn squared_length_gives_expected_results() {
        let x: f64 = 1.;
        let y: f64 = 2.;
        let z: f64 = 3.;
        let res: f64 = 14.;

        let v1: Vec3 = Vec3::new(x,y,z);

        assert_eq!(v1.squared_length(), res);
    }

    #[test]
    fn length_gives_expected_results() {
        let x: f64 = 1.;
        let y: f64 = 2.;
        let z: f64 = 3.;
        let res: f64 = 3.74165738677394138;
        let epsilon: f64 = 0.00000001;

        let v1: Vec3 = Vec3::new(x,y,z);

        assert!((v1.length() - res).abs() < epsilon);
    }
}

// Render gradient in .ppm file:
fn main() -> std::io::Result<()> {
    let mut file = File::create("out.ppm")?;
    let nx: u8 = 200;
    let ny: u8 = 100;
    file.write(format!("P3\n{} {}\n255\n", nx, ny).as_bytes())?;
    for j in (0..ny).rev() {
        for i in 0..nx {
            let r: f64 = i as f64 / nx as f64;
            let g: f64 = j as f64 / ny as f64;
            let b: f64 = 0.2;
            let ir: u8 = to_u8(255.99 * r);
            let ig: u8 = to_u8(255.99 * g);
            let ib: u8 = to_u8(255.99 * b);
            file.write(format!("{} {} {}\n", ir, ig, ib).as_bytes())?;
        }
    }
    Ok(())
}
