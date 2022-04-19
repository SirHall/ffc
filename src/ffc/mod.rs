pub mod gen_source;
pub mod grid;
pub mod pos;
#[cfg(test)]
pub mod test;

use serde::{Deserialize, Serialize};

// #[derive(Debug, Serialize, Deserialize)]
// pub struct FFCMap
// {
//     map : Array2<u32>,
// }

// impl FFCMap
// {
//     pub fn new(width : usize, height : usize) -> Self
//     {
//         Self {
//             map : Array2::zeros((width, height)),
//         }
//     }
// }
