use std::fs::File;
use std::io::prelude::*;
use std::ops::{Add, Neg, Index};

fn to_u8(val: f64) -> u8 { return val as u8; }

fn new_vec3(e0: f64, e1: f64, e2: f64) -> Vec3 {
    return Vec3 { values: [e0, e1, e2] };
}

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