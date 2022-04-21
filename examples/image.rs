use anyhow::Result;
use clap::Parser;
use ffc::ffc::collapse;
use ffc::prelude::*;
use image::io::Reader as ImageReader;
use image::{DynamicImage, GenericImage, GenericImageView, Pixel, Rgba};
use std::io::Cursor;
use std::{
    collections::HashMap,
    path::{Path, PathBuf},
};

/// Example application of FFC, allowing the generation of collapsed images of far greater size than before
#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args
{
    #[clap(short, long)]
    source : PathBuf,

    #[clap(short, long)]
    width :  usize,
    #[clap(short, long)]
    height : usize,
    #[clap(short, long)]
    radius : usize,
    #[clap(short, long)]
    wrap :   bool,

    #[clap(long)]
    reroll_attempts : Option<usize>,

    // TODO: Add support back for this
    // #[clap(short, long)]
    // count : Option<usize>,
    #[clap(short, long)]
    output : Option<PathBuf>,
}

fn main() -> Result<()>
{
    let args = Args::parse();

    // 0 - Reserved for unset
    // 1 - Reserved for outer
    let mut unique_count : usize = 2;

    let img = ImageReader::open(args.source)?.decode()?;

    let mut pattern = Grid::new(vec![0; (img.width() * img.height()) as usize], img.width() as usize);

    // Map between pixels and their integer value
    let mut pixel_to_int = HashMap::<u32, usize>::new();
    let mut int_to_pixel = vec![0u32; (img.width() * img.height()) as usize];

    for (pixel_idx, pixel) in img.pixels().enumerate()
    {
        let pixel_rgba : u32 = u32::from_ne_bytes(pixel.2.to_rgba().0);
        // println!("{pixel_rgba}");
        // The unique id for this color
        let pixel_color_id : usize;

        if pixel_to_int.contains_key(&pixel_rgba)
        {
            pixel_color_id = pixel_to_int.get(&pixel_rgba).unwrap().to_owned(); // Inefficient double lookup
        }
        else
        {
            pixel_color_id = unique_count;
            pixel_to_int.insert(pixel_rgba, pixel_color_id);
            int_to_pixel[pixel_color_id] = pixel_rgba;
            unique_count += 1;
        }

        pattern.set(pattern.i_to_pos(pixel_idx), pixel_color_id);
    }

    // ---
    // Generate the output images
    // ---

    // TODO: Use a smarter evaluation order
    let evaluate_order = (0..pattern.get_area()).collect::<Vec<_>>();

    for gen_num in 1..=1
    //(args.count.unwrap_or(1))
    {
        let mut grid = initialize::<usize>(&pattern, args.width, args.height, 0, 1);

        match collapse(
            grid,
            &evaluate_order,
            &pattern,
            args.radius as isize,
            args.reroll_attempts.unwrap_or(3),
            1, // TODO:
            0, // TODO:
            1,
        )
        {
            Some(generated_grid) =>
            {
                // We successfully generated this grid
                println!("Finished generating grid {gen_num}");
                let mut out_image = DynamicImage::new_rgba8(args.width as u32, args.height as u32);

                for i in 0..generated_grid.get_area()
                {
                    let pos = generated_grid.i_to_pos(i);
                    let pixel_rgba_u32 = int_to_pixel[generated_grid.get(pos.clone(), 1)];
                    println!("{pixel_rgba_u32}",);
                    let pixel_rgba_4u8 = pixel_rgba_u32.to_ne_bytes();
                    let pixel_rgba = Rgba::from_slice(&pixel_rgba_4u8);
                    unsafe {
                        out_image.unsafe_put_pixel(pos.x as u32, pos.y as u32, pixel_rgba.to_rgba());
                    }
                }

                // println!("Saving grid {gen_num}");
                out_image.save_with_format(
                    args.output
                        .clone()
                        .unwrap_or(args.output.clone().unwrap_or_else(|| PathBuf::from("out.png"))),
                    image::ImageFormat::Png,
                );
            },
            None =>
            {
                println!("Failed to generate grid {gen_num}");
            },
        }
    }

    Ok(())
}
