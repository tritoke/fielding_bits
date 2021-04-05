use rgb::ComponentBytes;
use std::{
    fs::File,
    io::{BufWriter, Write},
    path::Path,
};

use anyhow::Result;
use rayon::prelude::*;

type Pixel = rgb::RGB<u8>;
type ComponentFunc = dyn Fn(u32, u32) -> bool;
type Row = Box<[Pixel]>;

const WIDTH: u32 = 256;

fn main() -> Result<()> {
    // setup the file for buffered writing
    let path = Path::new("render.png");
    let file = File::create(path)?;
    let w = &mut BufWriter::new(file);

    // setup the PNG encoder / stram
    let mut encoder = png::Encoder::new(w, WIDTH, WIDTH);
    encoder.set_color(png::ColorType::RGB);
    encoder.set_depth(png::BitDepth::Eight);
    let mut writer = encoder.write_header()?;
    let mut stream = writer.stream_writer();

    // generate the pixels
    let fr = |x, y| (x * y) & 8 == 0;
    let fg = |x, y| (x * y) & 8 == 0;
    let fb = |x, y| (x * y) & 8 == 0;
    let rows: Vec<_> = (0..WIDTH).into_par_iter().map(|y| gen_row(y, (&fr, &fg, &fb))).collect();

    // write the rows out to disk
    for row in rows {
        stream.write(row.as_bytes())?;
    }

    // finalise the PNG
    stream.finish()?;

    Ok(())
}

fn gen_row(y: u32, funcs: (&ComponentFunc, &ComponentFunc, &ComponentFunc)) -> Row {
    let (fr, fg, fb) = funcs;

    (0..WIDTH)
        .map(|x| {
            Pixel {
                r: if fr(x, y) { 0xFF } else { 0x00 },
                g: if fg(x, y) { 0xFF } else { 0x00 },
                b: if fb(x, y) { 0xFF } else { 0x00 },
            }
        })
        .collect::<Vec<_>>()
        .into_boxed_slice()
}
