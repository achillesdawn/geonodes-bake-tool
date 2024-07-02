use std::{
    fs,
    io::{self, Read, Seek},
    ops::{Add, Div, Mul, Sub},
    path::PathBuf,
    str::FromStr,
};

use thiserror::Error;

mod api;

use api::{Attribute, BakeMetadata, Frame};

#[derive(Debug)]
struct Config {
    base_path: PathBuf,
    attribute_name: String,
}


#[derive(Error, Debug)]
enum MetaReadError {
    #[error("File read error")]
    Io {
        #[from]
        source: io::Error
    },
    #[error("Deserializing error")]
    Deserialize {
        #[from]
        source: serde_json::Error
    },
    #[error("Item not found")]
    ItemNotFound
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

        let frames: Vec<Frame> = Vec::new();

        for entry in fs::read_dir(meta_path).unwrap() {
            let entry = entry.unwrap();

            let metadata = entry.metadata().unwrap();

            if metadata.is_file() {
                let read_result = self.read_meta(entry.path());
                match read_result{
                    Ok(_) => todo!(),
                    Err(err) => match err. {
                        
                    },
                }
            }
        }
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

        Err(Box::new(io::Error::new(
            io::ErrorKind::NotFound,
            "Attribute not found",
        )))
    }

    fn read_blob(&self, attribute: &Attribute) -> Result<Frame, Box<dyn Error>> {
        let blob_path;
        {
            let mut path = self.base_path.clone();
            path.push("blobs");
            path.push(&attribute.data.name);
            blob_path = path;
        }

        // dbg!(&blob_path);

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
            .expect("Could not parse frame from blob filename");

        return Ok(Frame { buffer, number });
    }
}

fn main() {
    let config = Config {
        base_path: PathBuf::from_str("/tmp/91383020").unwrap(),
        attribute_name: "light".to_owned(),
    };

    config.load_meta();
}
