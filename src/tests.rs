use crate::prelude::*;

#[test]
fn test_compare() -> Result<(), String>
{
    let a = Grid::new(
        vec![
            0, 0, 0, 0, //
            0, 1, 0, 0, //
            0, 0, 2, 0, //
            0, 0, 0, 0, //
        ],
        4,
    );

    let b = Grid::new(
        vec![
            0, 0, 0, 0, //
            0, 0, 0, 0, //
            0, 0, 1, 0, //
            0, 0, 0, 2, //
        ],
        4,
    );

    if Grid::compare(&a, Pos::new(1, 1), &b, Pos::new(2, 2), 1, 127, 128)
    {
        Ok(())
    }
    else
    {
        Err(String::from("Compare failed"))
    }
}
