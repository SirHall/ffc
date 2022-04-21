use super::{
    grid::{Grid, GridCellT},
    pos::Pos,
};
use rand::prelude::SliceRandom;
use rayon::prelude::*;
use std::collections::HashMap;

pub fn initialize<T : GridCellT>(
    pattern : Grid<T>,
    out_width : usize,
    out_height : usize,
    unset : T,
    outer : T,
) -> Grid<T>
{
    let out_grid = Grid::new(vec![unset; out_width * out_height], out_width);

    for i in 0..pattern.get_area()
    {
        let pos = pattern.i_to_pos(i);
        let tile = pattern.get(pos.clone(), outer.clone());
    }

    out_grid
}

pub fn collapse<T : GridCellT>(
    mut grid : Grid<T>,
    evaluate_order : &Vec<usize>,
    // pattern_map : &HashMap<T, Vec<Pos>>, // Most likely not needed
    pattern : &Grid<T>,
    radius : isize,
    // pattern_unset_matches_all : bool,
    reroll_attempts : usize,
    climb_amount_on_reroll : usize,
    unset : T,
    outer : T,
) -> Option<Grid<T>>
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

    // roll_count serves as a stack to count the number of times we have rolled the current evaluation index (always the
    // top of the stack/last item)
    let mut roll_counts = vec![0];

    let mut rng = rand::rngs::ThreadRng::default();

    while roll_counts.len() < evaluate_order.len()
    {
        if roll_counts.is_empty()
        {
            return None; // We've failed to generate anything
        }

        let i = roll_counts.len() - 1;

        roll_counts[i] += 1; // Are are doing the roll for this evaluation index now
        let eval_pos = grid.i_to_pos(evaluate_order[i]);

        // Iterate over all cells in the source pattern, and form a list of all local patterns that could be used at
        // this tile's location
        let valid_pattern_pos_list = (0..pattern.get_area())
            .into_par_iter()
            .map(|pattern_i| pattern.i_to_pos(pattern_i))
            .filter(|p_pos| {
                Grid::compare(
                    &grid,
                    eval_pos.clone(),
                    pattern,
                    p_pos.to_owned(),
                    radius,
                    unset.clone(),
                    outer.clone(),
                )
            })
            .collect::<Vec<_>>();

        if valid_pattern_pos_list.is_empty()
        {
            // TODO:
            // We have nothing to put here, fall back to a previous step and roll again
            for _ix in 0..climb_amount_on_reroll
            {
                roll_counts.pop();
            }
        }
        else
        {
            // We have at-least one pattern we can super impose here, choose one at random
            let selection_pos = valid_pattern_pos_list.choose(&mut rng).unwrap().to_owned();
            grid.set(eval_pos, pattern.get(selection_pos, outer.clone()));

            // Push a 0 roll count for the next evaluated position element

            roll_counts.push(0);
        }
    }

    Some(grid)
}
