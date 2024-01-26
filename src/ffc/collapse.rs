use super::grid::Grid;
use rand::prelude::SliceRandom;
use std::hash::Hash;

pub fn initialize<T: PartialEq + Eq + Hash + Clone + Sync + Send>(
    out_width: usize,
    out_height: usize,
    unset: T,
) -> Grid<T> {
    Grid::new(vec![unset; out_width * out_height], out_width)
}

pub fn collapse<T: PartialEq + Eq + Hash + Clone + Sync + Send>(
    mut grid: Grid<T>,
    evaluate_order: &Vec<usize>,
    pattern: &Grid<T>,
    radius: isize,
    reroll_attempts: usize,
    climb_amount_on_reroll: usize,
    unset: T,
    outer: T,
) -> Option<Grid<T>> {
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

    let pattern_points = (0..pattern.get_area())
        .into_iter()
        .map(|pattern_i| pattern.i_to_pos(pattern_i))
        .collect::<Vec<_>>();

    // roll_count serves as a stack to count the number of times we have rolled the current evaluation index (always the
    // top of the stack/last item)
    let mut roll_counts = vec![0];

    macro_rules! fallback {
        () => {
            // println!("fallback");
            for _ix in 0..climb_amount_on_reroll {
                if !roll_counts.is_empty() {
                    roll_counts.pop();
                    // let i = roll_counts.len() - 1;
                    // let eval_pos = evaluate_order[i];
                    // grid.set(grid.i_to_pos(eval_pos), unset.clone());
                }
            }
        };
    }

    let mut rng = rand::rngs::ThreadRng::default();

    while roll_counts.len() < evaluate_order.len() {
        if roll_counts.is_empty() {
            return None; // We've failed to generate anything
        }

        let i = roll_counts.len() - 1;
        // println!("i: {} {}", i, roll_counts[i]);

        roll_counts[i] += 1; // We are doing the roll for this evaluation index now

        if roll_counts[i] > reroll_attempts {
            fallback!();
            continue;
        }

        let eval_pos = grid.i_to_pos(evaluate_order[i]);
        grid.set(&eval_pos, unset.clone());

        // println!("{i}");

        // Iterate over all cells in the source pattern, and form a list of all local patterns that could be used at
        // this tile's location
        let valid_pattern_pos_list = pattern_points
            .iter()
            .filter(|p_pos| Grid::compare(pattern, &p_pos, &grid, &eval_pos, radius, unset.clone(), outer.clone()))
            .collect::<Vec<_>>();

        if valid_pattern_pos_list.is_empty() {
            // We have nothing to put here, fall back to a previous step and roll again
            // fallback!();
        } else {
            // We have at-least one pattern we can super impose here, choose one at random
            let selection_pos = valid_pattern_pos_list.choose(&mut rng).unwrap().to_owned();
            // println!("Pos: {} {}", selection_pos.x, selection_pos.y);
            grid.set(&eval_pos, pattern.get(selection_pos, outer.clone()));

            // Push a 0 roll count for the next evaluated position element

            roll_counts.push(0);
        }
    }

    Some(grid)
}
