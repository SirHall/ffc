use super::{
    grid::{Grid, GridCellT},
    pos::Pos,
};
use rayon::prelude::*;
use std::collections::{hash_map, HashMap};

pub fn initialize<T : GridCellT>(
    pattern : Grid<T>,
    out_width : usize,
    out_height : usize,
    unset : T,
    outer : T,
) -> Grid<T>
{
    let out_grid = Grid::new(vec![unset; out_width * out_height], out_width);

    // Maps an input center cell value to a list of positions in the pattern map with that center value

    let mut pattern_map = HashMap::<T, Vec<Pos>>::new();

    for i in 0..pattern.get_area()
    {
        let pos = pattern.i_to_pos(i);
        let tile = pattern.get(pos.clone(), outer.clone());

        pattern_map.entry(tile).or_insert_with(|| vec![]).push(pos);
    }

    out_grid
}

pub fn collapse<T : GridCellT>(
    mut grid : Grid<T>,
    evaluate_order : &Vec<usize>,
    pattern_map : &HashMap<T, Vec<Pos>>, // Most likely not needed
    pattern : &Grid<T>,
    pattern_unset_matches_all : bool,
    reroll_attempts : usize,
    climb_amount_on_reroll : usize,
    unset : T,
    outer : T,
) -> Grid<T>
{
    // | evaluate_order
    // + evaluate_order is a list of indices where each index in the output grid exists once and only once.
    // + The solver will visit these indices in the order they exist in evaluate_order to resolve the cells.
    // + This then makes it very easy to walk backwards any time a cell needs to be rerolled.

    // | pattern_unset_matches_all
    // + Any 'unset' cell in the source pattern will match against all center-tests when
    // + collapsing

    // | reroll_attempts
    // + The maximum number of times a particular cell can be rerolled before we fall back to rerolling its parent.
    // + It is advised to keep this quite low

    // | climb_amount_on_reroll
    // + When we reroll we jump backward through the evaluate_order list by this number, generally advised to keep
    // + this at 1.

    let mut i = 0;

    let roll_count = vec![0usize; 0];

    // let empty = vec![];

    loop
    {
        let eval_pos = grid.i_to_pos(evaluate_order[i]);
        let eval_tile = grid.get(eval_pos, outer.clone());

        // let set_options;
        // {
        //     let current_pattern_it = pattern_map.get(&eval_tile).unwrap_or_else(|| &empty).par_iter();
        //     let unset_pattern_it;
        //     if pattern_unset_matches_all
        //     {
        //         unset_pattern_it = pattern_map.get(&unset).unwrap_or_else(|| &empty).par_iter();
        //     }
        //     else
        //     {
        //         unset_pattern_it = [].par_iter();
        //     };

        //     // Iterates over all patterns that have the same center as our current tile
        //     let all_pattern_it = current_pattern_it.interleave(unset_pattern_it);

        //     // Now iterate over all of these patterns and find which *could* fit into our current location
        // }

        // grid.set(eval_pos, val);

        i += 1;
    }
}
