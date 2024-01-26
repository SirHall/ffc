use clap::Parser;
use ffc::{
    ffc::collapse_rules::{collapse_rule, CollapseRule},
    prelude::*,
};
use image::{DynamicImage, GenericImage, Pixel, Rgb};
use std::path::PathBuf;

/// Example application of FFC, allowing the generation of collapsed images of far greater size than before
#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    // #[clap(short, long)]
    // source: PathBuf,
    #[clap(short, long)]
    width: Option<usize>,
    #[clap(short, long)]
    height: Option<usize>,
    #[clap(short, long)]
    radius: Option<usize>,
    // #[clap(long)]
    // wrap: bool,
    #[clap(short, long)]
    seeds: Option<usize>,

    #[clap(long)]
    reroll_attempts: Option<usize>,

    // TODO: Add support back for this
    // #[clap(short, long)]
    // count : Option<usize>,
    #[clap(short, long)]
    output: Option<PathBuf>,

    #[clap(short, long)]
    unset_and_outer_are_equal: Option<bool>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Tile {
    Unset,
    Outer,
    DeepWater,
    Water,
    ShallowWater,
    Sand,
    Grass,
    Forest,
    Mountain,
}

fn main() {
    let args = Args::parse();

    let unset_and_outer_are_equal = args.unset_and_outer_are_equal.unwrap_or(false);

    let unset = Tile::Unset;
    let outer = if unset_and_outer_are_equal { unset } else { Tile::Outer };

    let seeds = args.seeds.unwrap_or(1);

    let width = args.width.unwrap_or(16);
    let height = args.height.unwrap_or(16);
    let radius = args.radius.unwrap_or(1);

    let grid = initialize(width, height, unset);
    let history_grid = initialize(width, height, unset);

    let tile_options = vec![
        Tile::Water,
        Tile::ShallowWater,
        Tile::Sand,
        Tile::Grass,
        Tile::Forest,
        Tile::Mountain,
    ];

    let unset_tile_rule = CollapseRule::False;
    let outer_tile_rule = CollapseRule::False;
    let deep_water_tile_rule = CollapseRule::NextTo(Box::new(CollapseRule::Or(vec![
        CollapseRule::Is(Tile::Outer),
        CollapseRule::Is(Tile::DeepWater),
        CollapseRule::Is(Tile::Water),
        // CollapseRule::Is(Tile::Mountain),
    ])));
    let water_tile_rule = CollapseRule::And(vec![
        CollapseRule::NextTo(Box::new(CollapseRule::Or(vec![
            CollapseRule::Is(Tile::Water),
            CollapseRule::Is(Tile::Outer),
            CollapseRule::Is(Tile::DeepWater),
            CollapseRule::Is(Tile::ShallowWater),
            // CollapseRule::Is(Tile::Forest),
        ]))),
        CollapseRule::Not(Box::new(CollapseRule::Near(
            Box::new(CollapseRule::Or(vec![
                CollapseRule::Is(Tile::Sand),
                CollapseRule::Is(Tile::Grass),
            ])),
            3,
        ))),
    ]);
    let shallow_water_tile_rule = CollapseRule::NextTo(Box::new(CollapseRule::Or(vec![
        CollapseRule::Is(Tile::ShallowWater),
        CollapseRule::Is(Tile::Water),
        CollapseRule::Is(Tile::Sand),
    ])));
    let sand_tile_rule = CollapseRule::And(vec![
        CollapseRule::NextTo(Box::new(CollapseRule::Or(vec![
            CollapseRule::Is(Tile::Sand),
            CollapseRule::Is(Tile::ShallowWater),
            CollapseRule::Is(Tile::Grass),
            // CollapseRule::Is(Tile::Mountain),
        ]))),
        CollapseRule::Not(Box::new(CollapseRule::Near(Box::new(CollapseRule::Is(Tile::Outer)), 5))),
    ]);
    let grass_tile_rule = CollapseRule::And(vec![
        CollapseRule::NextTo(Box::new(CollapseRule::Or(vec![
            CollapseRule::Is(Tile::Grass),
            CollapseRule::Is(Tile::Sand),
            CollapseRule::Is(Tile::Forest),
        ]))),
        CollapseRule::Not(Box::new(CollapseRule::Near(Box::new(CollapseRule::Is(Tile::Outer)), 6))),
    ]);

    let forest_tile_rule = CollapseRule::And(vec![
        CollapseRule::NextTo(Box::new(CollapseRule::Or(vec![
            CollapseRule::Is(Tile::Forest),
            CollapseRule::Is(Tile::Grass),
            CollapseRule::Is(Tile::Mountain),
            // CollapseRule::Is(Tile::Water),
        ]))),
        CollapseRule::Not(Box::new(CollapseRule::Near(Box::new(CollapseRule::Is(Tile::Outer)), 7))),
    ]);

    let mountain_tile_rule = CollapseRule::And(vec![
        CollapseRule::NextTo(Box::new(CollapseRule::Or(vec![
            CollapseRule::Is(Tile::Mountain),
            CollapseRule::Is(Tile::Forest),
            // CollapseRule::Is(Tile::DeepWater),
            // CollapseRule::Is(Tile::Sand),
        ]))),
        CollapseRule::Not(Box::new(CollapseRule::Near(Box::new(CollapseRule::Is(Tile::Outer)), 8))),
    ]);

    let tile_to_rule = |&tile| match tile {
        Tile::Unset => &unset_tile_rule,
        Tile::Outer => &outer_tile_rule,
        Tile::DeepWater => &deep_water_tile_rule,
        Tile::Water => &water_tile_rule,
        Tile::ShallowWater => &shallow_water_tile_rule,
        Tile::Sand => &sand_tile_rule,
        Tile::Grass => &grass_tile_rule,
        Tile::Forest => &forest_tile_rule,
        Tile::Mountain => &mountain_tile_rule,
    };

    let result = collapse_rule(
        grid,
        &history_grid,
        &tile_options[..],
        tile_to_rule,
        radius as isize,
        unset,
        outer,
        256,
        seeds,
    );

    if let Some(generated_grid) = result {
        let mut out_image = DynamicImage::new_rgba8(width as u32, height as u32);

        for i in 0..generated_grid.get_area() {
            let pos = generated_grid.i_to_pos(i);
            let tile = generated_grid.get(&pos, Tile::Outer);

            let color = match tile {
                Tile::Unset => Rgb([0, 0, 0]),
                Tile::Outer => Rgb([128, 0, 128]),
                Tile::Water => Rgb([0, 0, 255]),
                Tile::Sand => Rgb([255, 255, 0]),
                Tile::Grass => Rgb([0, 255, 0]),
                Tile::Forest => Rgb([0, 128, 0]),
                Tile::Mountain => Rgb([128, 128, 128]),
                Tile::DeepWater => Rgb([0, 0, 128]),
                Tile::ShallowWater => Rgb([0, 128, 255]),
            };

            unsafe {
                out_image.unsafe_put_pixel(pos.x as u32, pos.y as u32, color.to_rgba());
            }
        }

        out_image
            .save_with_format(
                args.output
                    .clone()
                    .unwrap_or(args.output.clone().unwrap_or_else(|| PathBuf::from("out.png"))),
                image::ImageFormat::Png,
            )
            .expect("Failed to save out.png");
    } else {
        println!("Failed to generate image");
    }
}
