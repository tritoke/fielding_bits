use rgb::ComponentBytes;
use std::{
    fs::{self,File},
    io::BufWriter,
    path::PathBuf,
};

use anyhow::{Result,Context};
use rayon::prelude::*;

type Pixel = rgb::RGB<u8>;

const WIDTH: i32 = 256;

fn main() -> Result<()> {
    let frame_path = PathBuf::from("./frames/");
    fs::create_dir(&frame_path).context("failed to crate frames directory.")?;

    for (i, z) in (0..=WIDTH).enumerate() { //((-WIDTH / 2)..=(WIDTH / 2)).enumerate() {
        // setup the file for buffered writing
        let mut frame = frame_path.clone();
        frame.push(format!("{:05}.png", i));
        let file = File::create(frame)?;
        let w = &mut BufWriter::new(file);

        // setup the PNG encoder / stram
        let mut encoder = png::Encoder::new(w, WIDTH as u32, WIDTH as u32);
        encoder.set_color(png::ColorType::RGB);
        encoder.set_depth(png::BitDepth::Eight);
        let mut writer = encoder.write_header()?;

        // generate the pixels
        /*
        let fr = |x, y| if (x * y) & 64 != 0 { 0xFF } else { 0x00 };
        let fg = |x, y| if (x | y) % 17 == 0 { 0xFF } else { 0x00 };
        let fb = |x, y| if (x ^ y) & 32 == 0 { 0xFF } else { 0x00 };
        */

        /*
        let ff = |x, y, z| {
            let modulus = (x - 128);
            let left = (y - 128) * z;
            if modulus != 0 && (left % modulus) == 0 { 0xFF } else { 0x00 }
        };
        */

        let ff = |x: i32, y: i32, z: i32| if (x * y) & ((x ^ y) < z) as u8 as i32 == 0 { 0xFF } else { 0x00 };

        let pixels: Vec<_> = (0..WIDTH*WIDTH)
            .into_par_iter()
            .map(|i| {
                let x = i / WIDTH;
                let y = i % WIDTH;

                let v = ff(x, y, z);
                Pixel {
                    r: v, //fr(x, y),
                    g: v, //fg(x, y),
                    b: v, //fb(x, y),
                }
            })
        .collect();

        // write the rows out to disk
        writer.write_image_data(pixels.as_bytes())?;
    }

    Ok(())
}
