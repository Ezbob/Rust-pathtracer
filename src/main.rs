use std::fs::File;
use std::io::prelude::*;

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

fn to_u8(val: f64) -> u8 { return val as u8; }