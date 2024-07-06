use std::{
    fs,
    io::{self, Read, Seek},
    path::PathBuf,
    str::FromStr,
};

use api::{Attribute, BakeMetadata, Frame};
use errors::MetaReadError;

pub mod api;
pub mod errors;
pub mod math;
pub mod tui;

pub struct App {
    base_path: PathBuf,
    attribute_name: String,
    col_size: usize,
    pub frames: Vec<Frame>,
}

impl App {
    pub fn new(base_path: &str, attribute_name: &str, col_size: usize) -> Self {
        App {
            base_path: PathBuf::from_str(base_path).unwrap(),
            attribute_name: attribute_name.to_owned(),
            frames: Vec::new(),
            col_size,
        }
    }

    pub fn load_meta(&mut self) -> Result<(), MetaReadError> {
        let mut meta_path = self.base_path.clone();
        meta_path.push("meta");

        let mut frames: Vec<Frame> = Vec::new();

        for entry in fs::read_dir(meta_path).unwrap() {
            let entry = entry.unwrap();

            let metadata = entry.metadata().unwrap();

            if metadata.is_file() {
                let read_result = self.read_meta(entry.path());
                match read_result {
                    Ok(frame) => frames.push(frame),
                    Err(err) => return Err(err),
                }
            }
        }

        frames.sort_by(|a, b| a.number.cmp(&b.number));
        self.frames = frames;
        Ok(())
    }

    fn read_meta(&self, path: PathBuf) -> Result<Frame, MetaReadError> {
        let file = fs::File::open(path)?;
        let bake_metadata: BakeMetadata = serde_json::from_reader(file)?;

        let item = bake_metadata.items.get("0");
        if item.is_none() {
            return Err(MetaReadError::ItemNotFound);
        }
        let item = item.unwrap();

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

            let frame = self.read_blob(attribute)?;
            return Ok(frame);
        }

        Err(MetaReadError::AttributeNotFound)
    }

    fn read_blob(&self, attribute: &Attribute) -> Result<Frame, MetaReadError> {
        let blob_path;
        {
            let mut path = self.base_path.clone();
            path.push("blobs");
            path.push(&attribute.data.name);
            blob_path = path;
        }

        dbg!(&blob_path);

        let mut file = fs::File::open(blob_path)?;
        file.seek(io::SeekFrom::Start(attribute.data.start))?;

        let mut reader = io::BufReader::new(file).take(attribute.data.size);

        let mut result: Vec<f32> = Vec::with_capacity((attribute.data.size / 4) as usize);
        let mut buffer = [0u8; 4];

        while let Ok(()) = reader.read_exact(&mut buffer) {
            let num = f32::from_le_bytes(buffer);
            result.push(num);
        }

        let buffer = map_results(result);

        let number = attribute
            .data
            .name
            .split("_")
            .next()
            .unwrap_or("")
            .parse::<u32>()
            .map_err(|_| MetaReadError::ParseIntError)?;

        Ok(Frame { buffer, number })
    }
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
                math::map_range(*num, min, max, 0.0, 5.0)
            } else {
                0.0
            }
        })
        .map(|num| characters[num as usize])
        .collect();

    r
}
