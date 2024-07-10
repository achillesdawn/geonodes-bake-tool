use std::{
    collections::HashMap,
    fs,
    io::{self, Read, Seek},
    path::PathBuf,
    str::FromStr,
};

use bake_reader::api::{RawAttribute, AttributeType, BakeMetadata, Frame};

pub mod math;
pub mod tui;

mod bake_reader;

pub use bake_reader::api;

pub struct App {
    base_path: PathBuf,
    attribute_names: Vec<&'static str>,
    col_size: usize,
    pub frames: Vec<Frame>,
}

fn map_results(nums: Vec<f32>, col_size: usize) -> String {
    let characters = [' ', '·', '-', '+', 'r', '@'];

    let mut max = f32::NEG_INFINITY;
    let mut min = f32::INFINITY;

    for item in nums.iter() {
        if *item > max {
            max = *item;
        } else if *item < min {
            min = *item;
        }
    }

    nums.chunks_exact(col_size)
        .map(|row| row.iter().map(|num| {}));

    // println!("MAx {} Min {}", max, min);

    let r: String = nums
        .iter()
        .map(|num| {
            if *num != 0.0 {
                math::map_range(*num, min, max, 0.0, 5.0)
            } else {
                0.0
            }
        })
        .map(|num| characters[num as usize])
        .collect();

    r
}
