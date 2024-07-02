use std::{
    io::{Read, Seek},
    ops::{Add, Div, Mul, Sub},
    path::PathBuf,
    str::FromStr,
};

mod api;

use api::{Attribute, BakeMetadata};

#[derive(Debug)]
struct Config {
    base_path: PathBuf,
    attribute_name: String
}

fn map_range<T: Copy>(value: T, from_min: T, from_max: T, to_min: T, to_max: T) -> T
where
    T: Add<T, Output = T> + Sub<T, Output = T> + Mul<T, Output = T> + Div<T, Output = T>,
{
    to_min + (value - from_min) * (to_max - to_min) / (from_max - from_min)
}

fn map_results(nums: Vec<f32>) -> String {
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

    // println!("MAx {} Min {}", max, min);

    let r: String = nums
        .iter()
        .map(|num| {
            if *num != 0.0 {
                map_range(*num, min, max, 0.0, 5.0)
            } else {
                0.0
            }
        })
        .map(|num| characters[num as usize])
        .collect();

    r
}

impl Config {
    fn load_meta(&self) {
        let mut meta_path = self.base_path.clone();
        meta_path.push("meta");

        for entry in std::fs::read_dir(meta_path).unwrap() {
            let entry = entry.unwrap();

            let metadata = entry.metadata().unwrap();

            if metadata.is_file() {
                self.read_meta(entry.path());
            }
        }
    }

    fn read_meta(&self, path: PathBuf) {
        let file = std::fs::File::open(path).unwrap();
        let bake_metadata: BakeMetadata = serde_json::from_reader(file).unwrap();

        let item = bake_metadata.items.get("0").unwrap();
        println!("{} {:?} ", item.name, item.item_type);

        for attribute in item.data.mesh.attributes.iter() {
            if attribute.name != self.attribute_name {
                continue;
            }

            println!("Blob file {}", attribute.data.name);
            println!(
                "Found {}, of domain {} and type {}",
                attribute.name, attribute.domain, attribute.attribute_type
            );

            self.read_blob(attribute);
        }
    }

    fn read_blob(&self, attribute: &Attribute) -> String {
        let blob_path;
        {
            let mut path = self.base_path.clone();
            path.push("blobs");
            path.push(&attribute.data.name);
            blob_path = path;
        }

        // dbg!(&blob_path);

        let mut file = std::fs::File::open(blob_path).unwrap();
        file.seek(std::io::SeekFrom::Start(attribute.data.start))
            .unwrap();

        let mut reader = std::io::BufReader::new(file).take(attribute.data.size);

        let mut result: Vec<f32> = Vec::with_capacity((attribute.data.size / 4) as usize);
        let mut buffer = [0u8; 4];

        while let Ok(()) = reader.read_exact(&mut buffer) {
            let num = f32::from_le_bytes(buffer);
            result.push(num);
        }

        let buffer = map_results(result);
        return buffer;
    }
}
fn main() {
    let config = Config {
        base_path: PathBuf::from_str("/tmp/91383020").unwrap(),
        attribute_name: "light".to_owned()
    };

    config.load_meta();
}
