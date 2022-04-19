// use ndarray::prelude::*;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

// #[derive(Debug, Default, Serialize, Deserialize)]
// pub struct GenSource
// {
//     // Maps a tile-type to a vector of possible fit templates
//     center_pieces : HashMap<u32, Vec<Array2<u32>>>,
// }

// impl GenSource
// {
//     pub fn generate(radius : usize, src : &Array2<u32>) -> Self
//     {
//         let mut pieces = HashMap::<u32, Vec<Array2<u32>>>::default();

//         let mut neighbours = src.view();

//         Self {
//             center_pieces : pieces
//         }
//     }
// }
