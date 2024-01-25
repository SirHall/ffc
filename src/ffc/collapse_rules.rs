use super::grid::Grid;
use super::pos::Pos;
use std::fmt::Display;
use std::hash::Hash;

pub enum CollapseRule<T: PartialEq + Eq + Hash + Clone + Display + Sync + Send> {
    And(Vec<CollapseRule<T>>),
    Or(Vec<CollapseRule<T>>),
    Not(Box<CollapseRule<T>>),

    Is(T),
    Was(T),

    NextTo(Box<CollapseRule<T>>),
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

pub fn check_rule<T: PartialEq + Eq + Hash + Clone + Display + Sync + Send>(
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

    match rule {
        CollapseRule::And(sub_rules) => sub_rules.iter().all(move |sub_rule| {
            check_rule(
                grid,
                history_grid,
                pos,
                sub_rule,
                unset.clone(),
                outer.clone(),
                max_depth - 1,
            )
        }),
        CollapseRule::Or(sub_rules) => sub_rules.iter().any(move |sub_rule| {
            check_rule(
                grid,
                history_grid,
                pos,
                sub_rule,
                unset.clone(),
                outer.clone(),
                max_depth - 1,
            )
        }),
        CollapseRule::Not(sub_rule) => !check_rule(
            grid,
            history_grid,
            pos,
            sub_rule,
            unset.clone(),
            outer.clone(),
            max_depth - 1,
        ),
        CollapseRule::Is(tile_type) => grid.get(pos, outer.clone()) == *tile_type,
        CollapseRule::Was(tile_type) => history_grid.get(pos, outer.clone()) == *tile_type,
        CollapseRule::Left(sub_rule) => check_rule(
            grid,
            history_grid,
            &Pos { x: pos.x - 1, y: pos.y },
            sub_rule,
            unset.clone(),
            outer.clone(),
            max_depth - 1,
        ),
        CollapseRule::Right(sub_rule) => check_rule(
            grid,
            history_grid,
            &Pos { x: pos.x + 1, y: pos.y },
            sub_rule,
            unset.clone(),
            outer.clone(),
            max_depth - 1,
        ),
        CollapseRule::Up(sub_rule) => check_rule(
            grid,
            history_grid,
            &Pos { x: pos.x, y: pos.y + 1 },
            sub_rule,
            unset.clone(),
            outer.clone(),
            max_depth - 1,
        ),
        CollapseRule::Down(sub_rule) => check_rule(
            grid,
            history_grid,
            &Pos { x: pos.x, y: pos.y - 1 },
            sub_rule,
            unset.clone(),
            outer.clone(),
            max_depth - 1,
        ),
        CollapseRule::UpLeft(sub_rule) => check_rule(
            grid,
            history_grid,
            &Pos {
                x: pos.x - 1,
                y: pos.y + 1,
            },
            sub_rule,
            unset.clone(),
            outer.clone(),
            max_depth - 1,
        ),
        CollapseRule::UpRight(sub_rule) => check_rule(
            grid,
            history_grid,
            &Pos {
                x: pos.x + 1,
                y: pos.y + 1,
            },
            sub_rule,
            unset.clone(),
            outer.clone(),
            max_depth - 1,
        ),
        CollapseRule::DownLeft(sub_rule) => check_rule(
            grid,
            history_grid,
            &Pos {
                x: pos.x - 1,
                y: pos.y - 1,
            },
            sub_rule,
            unset.clone(),
            outer.clone(),
            max_depth - 1,
        ),
        CollapseRule::DownRight(sub_rule) => check_rule(
            grid,
            history_grid,
            &Pos {
                x: pos.x + 1,
                y: pos.y - 1,
            },
            sub_rule,
            unset.clone(),
            outer.clone(),
            max_depth - 1,
        ),
        CollapseRule::NextTo(sub_rule) => {
            let radius = 1;
            for dx in (-radius)..=radius {
                for dy in (-radius)..=radius {
                    if dx == 0 && dy == 0 {
                        continue;
                    }
                    let neighbour_pos = pos.rel(dx, dy);
                    let neighour_result = check_rule(
                        grid,
                        history_grid,
                        &neighbour_pos,
                        sub_rule,
                        unset.clone(),
                        outer.clone(),
                        max_depth - 1,
                    );
                    if neighour_result == false {
                        return false;
                    }
                }
            }

            true
        }
        CollapseRule::Parenthesis(sub_rule) => {
            check_rule(grid, history_grid, pos, sub_rule, unset, outer, max_depth - 1)
        }
        CollapseRule::True => true,
        CollapseRule::False => false,
        CollapseRule::InBounds => grid.is_valid(&pos),
        CollapseRule::Unset => grid.get(&pos, outer.clone()) == outer,
    }
}

// pub fn collapse_rule()
