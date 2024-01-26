use super::grid::Grid;
use super::pos::Pos;
use priority_queue::PriorityQueue;
use rand::Rng;
use std::cmp::Reverse;
use std::hash::Hash;

pub enum CollapseRule<T: PartialEq + Eq + Hash + Clone + Sync + Send> {
    And(Vec<CollapseRule<T>>),
    Or(Vec<CollapseRule<T>>),
    Not(Box<CollapseRule<T>>),

    Is(T),
    Was(T),

    Near(Box<CollapseRule<T>>, isize),
    NextTo(Box<CollapseRule<T>>),
    NextTo1(Box<CollapseRule<T>>),
    Left(Box<CollapseRule<T>>),
    Right(Box<CollapseRule<T>>),
    Up(Box<CollapseRule<T>>),
    Down(Box<CollapseRule<T>>),
    UpLeft(Box<CollapseRule<T>>),
    UpRight(Box<CollapseRule<T>>),
    DownLeft(Box<CollapseRule<T>>),
    DownRight(Box<CollapseRule<T>>),

    Parenthesis(Box<CollapseRule<T>>),

    True,
    False,
    InBounds, // Can be used to check if this tile falls outside of the world bounds
    Unset,    // Can be used to check if this tile has not been set yet
}

pub fn check_rule<T: PartialEq + Eq + Hash + Clone + Sync + Send>(
    grid: &Grid<T>,
    history_grid: &Grid<T>,
    pos: &Pos,
    rule: &CollapseRule<T>,
    unset: T,
    outer: T,
    max_depth: usize,
) -> bool {
    if max_depth == 0 {
        return false;
    }

    macro_rules! sub_check_rule {
        ($sub_pos:expr, $sub_rule:expr) => {
            check_rule(
                grid,
                history_grid,
                $sub_pos,
                $sub_rule,
                unset.clone(),
                outer.clone(),
                max_depth - 1,
            )
        };
    }

    match rule {
        CollapseRule::And(sub_rules) => sub_rules.iter().all(move |sub_rule| sub_check_rule!(pos, sub_rule)),
        CollapseRule::Or(sub_rules) => sub_rules.iter().any(move |sub_rule| sub_check_rule!(pos, sub_rule)),
        CollapseRule::Not(sub_rule) => !sub_check_rule!(pos, sub_rule),
        CollapseRule::Is(tile_type) => {
            let tile = grid.get(pos, outer.clone());
            tile == unset || tile == *tile_type
        }
        CollapseRule::Was(tile_type) => {
            let tile: T = history_grid.get(pos, outer.clone());
            tile == unset || tile == *tile_type
        }
        CollapseRule::Left(sub_rule) => sub_check_rule!(&Pos { x: pos.x - 1, y: pos.y }, sub_rule),
        CollapseRule::Right(sub_rule) => sub_check_rule!(&Pos { x: pos.x + 1, y: pos.y }, sub_rule),
        CollapseRule::Up(sub_rule) => sub_check_rule!(&Pos { x: pos.x, y: pos.y + 1 }, sub_rule),
        CollapseRule::Down(sub_rule) => sub_check_rule!(&Pos { x: pos.x, y: pos.y - 1 }, sub_rule),
        CollapseRule::UpLeft(sub_rule) => sub_check_rule!(
            &Pos {
                x: pos.x - 1,
                y: pos.y + 1,
            },
            sub_rule
        ),
        CollapseRule::UpRight(sub_rule) => sub_check_rule!(
            &Pos {
                x: pos.x + 1,
                y: pos.y + 1,
            },
            sub_rule
        ),
        CollapseRule::DownLeft(sub_rule) => sub_check_rule!(
            &Pos {
                x: pos.x - 1,
                y: pos.y - 1,
            },
            sub_rule
        ),
        CollapseRule::DownRight(sub_rule) => sub_check_rule!(
            &Pos {
                x: pos.x + 1,
                y: pos.y - 1,
            },
            sub_rule
        ),
        CollapseRule::Near(sub_rule, radius) => {
            for dx in (-radius)..=*radius {
                for dy in (-radius)..=*radius {
                    if dx == 0 && dy == 0 {
                        continue;
                    }
                    let neighbour_pos = pos.rel(dx, dy);
                    let neighour_result = sub_check_rule!(&neighbour_pos, sub_rule);
                    if neighour_result == false {
                        return false;
                    }
                }
            }

            true
        }
        CollapseRule::NextTo(sub_rule) => {
            sub_check_rule!(&Pos { x: pos.x, y: pos.y + 1 }, sub_rule)
                && sub_check_rule!(&Pos { x: pos.x, y: pos.y - 1 }, sub_rule)
                && sub_check_rule!(&Pos { x: pos.x + 1, y: pos.y }, sub_rule)
                && sub_check_rule!(&Pos { x: pos.x - 1, y: pos.y }, sub_rule)
        }
        CollapseRule::NextTo1(sub_rule) => {
            sub_check_rule!(&Pos { x: pos.x, y: pos.y + 1 }, sub_rule)
                || sub_check_rule!(&Pos { x: pos.x, y: pos.y - 1 }, sub_rule)
                || sub_check_rule!(&Pos { x: pos.x + 1, y: pos.y }, sub_rule)
                || sub_check_rule!(&Pos { x: pos.x - 1, y: pos.y }, sub_rule)
        }
        CollapseRule::Parenthesis(sub_rule) => {
            sub_check_rule!(pos, sub_rule)
        }
        CollapseRule::True => true,
        CollapseRule::False => false,
        CollapseRule::InBounds => grid.is_valid(&pos),
        CollapseRule::Unset => grid.get(&pos, outer.clone()) == outer,
    }
}

pub fn collapse_rule<'a, T: PartialEq + Eq + Hash + Clone + Sync + Send + 'a>(
    mut grid: Grid<T>,
    history_grid: &Grid<T>,
    tile_options: &'a [T],
    tile_to_rule: impl Fn(&'a T) -> &'a CollapseRule<T>,
    re_check_radius: isize,
    unset: T,
    outer: T,
    max_depth: usize,
    seeds: usize,
) -> Option<Grid<T>> {
    let mut placed_stack: Vec<(Pos, T, usize)> = vec![];
    let mut front = PriorityQueue::new();

    let mut rng = rand::rngs::ThreadRng::default();

    for _ in 0..seeds {
        let starting_point = rng.gen_range(0..grid.get_area());
        front.push(starting_point, Reverse(0));
    }

    let steps_to_print = grid.get_area() / 105;
    let mut clock_to_print = steps_to_print;

    macro_rules! find_valid_options {
        ($find_pos: expr) => {
            tile_options
                .iter()
                .filter(|tile_option| {
                    check_rule(
                        &grid,
                        history_grid,
                        $find_pos,
                        tile_to_rule(tile_option),
                        unset.clone(),
                        outer.clone(),
                        max_depth,
                    )
                })
                .collect::<Vec<_>>()
        };
    }

    while !front.is_empty()
    /* && placed_stack.len() < grid.get_area() */
    {
        if clock_to_print <= 0 {
            print!(
                "Progress: {: >3}%\r",
                ((100.0 / grid.get_area() as f32) * placed_stack.len() as f32) as u32
            );
            clock_to_print = steps_to_print;
        }
        clock_to_print -= 1;

        let (i, Reverse(_current_priority)) = front.pop().expect("Front is empty but we just checked it");

        let pos = grid.i_to_pos(i);

        let current_tile = grid.get(&pos, outer.clone());
        if current_tile != unset || current_tile == outer {
            continue;
        }

        let valid_options = find_valid_options!(&pos);

        // We've re-calculated this tile's valid options, and only want to evaluate the lowest-"entropy" option
        if !front.is_empty() && valid_options.len() > front.peek().unwrap().1 .0 {
            front.push(i, Reverse(valid_options.len()));
            continue;
        }

        if valid_options.is_empty() {
            front.push(i, Reverse(0));

            let mut first_backtrack = true;

            loop {
                if placed_stack.is_empty() {
                    return None; // We failed to generated anything
                }

                let (last_placed_pos, _, last_placed_attempts_remaining) =
                    placed_stack.last().expect("Stack is empty but we just checked it");

                if first_backtrack || *last_placed_attempts_remaining <= 0 {
                    first_backtrack = false;
                    grid.set(&last_placed_pos, unset.clone());

                    // let last_placed_options: Vec<&T> = find_valid_options!(&last_placed_pos);
                    // front.push(grid.pos_to_i(&last_placed_pos), last_placed_options.len());
                    front.push(grid.pos_to_i(&last_placed_pos), Reverse(0));

                    placed_stack.pop();
                } else {
                    placed_stack.last_mut().unwrap().2 = last_placed_attempts_remaining - 1;
                    break;
                }
            }
            continue;
        }

        let chosen_option = valid_options[rng.gen_range(0..valid_options.len())];

        grid.set(&pos, chosen_option.clone());
        placed_stack.push((pos.clone(), chosen_option.clone(), 15));

        // Now, for each unset neighbour within the re-check radius, we need to recheck it
        for dx in (-re_check_radius)..=re_check_radius {
            for dy in (-re_check_radius)..=re_check_radius {
                if dx == 0 && dy == 0 {
                    continue;
                }
                let neighbour_pos = pos.rel(dx, dy);
                if !grid.is_valid(&neighbour_pos) {
                    continue;
                }
                let neighbour_tile = grid.get(&neighbour_pos, outer.clone());
                if neighbour_tile != unset || neighbour_tile == outer {
                    continue;
                }
                // TODO: We should probably cache this somewhere and re-use it if no tiles within re_check_radius have changed (using a counter comparison?)
                let neighbour_valid_options = find_valid_options!(&neighbour_pos);
                front.push(grid.pos_to_i(&neighbour_pos), Reverse(neighbour_valid_options.len()));
                // front.push(grid.pos_to_i(&neighbour_pos), Reverse(0));
            }
        }
    }

    Some(grid)
}
