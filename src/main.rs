use std::{
    io::{Read, Seek},
    path::PathBuf,
    str::FromStr,
};

mod api;

use api::{Attribute, BakeMetadata};

#[derive(Debug)]
struct Config {
    base_path: PathBuf,
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

            break;
        }
    }

    fn read_meta(&self, path: PathBuf) {
        let file = std::fs::File::open(path).unwrap();
        let bake_metadata: BakeMetadata = serde_json::from_reader(file).unwrap();

        dbg!(&bake_metadata);
        let item = bake_metadata.items.get("0").unwrap();
        println!("{} {:?} ", item.name, item.item_type);

        for attribute in item.data.mesh.attributes.iter() {
            if attribute.name != "light" {
                continue;
            }

            println!(
                "Found {}, of domain {} and type {}",
                attribute.name, attribute.domain, attribute.attribute_type
            );
            println!("Blob file {}", attribute.data.name);

            self.read_blob(attribute);
            break;
        }
    }

    fn read_blob(&self, attribute: &Attribute) {
        let blob_path;
        {
            let mut path = self.base_path.clone();
            path.push("blobs");
            path.push(&attribute.data.name);
            blob_path = path;
        }

        dbg!(&blob_path);

        let mut file = std::fs::File::open(blob_path).unwrap();
        file.seek(std::io::SeekFrom::Start(attribute.data.start))
            .unwrap();

        let mut reader = std::io::BufReader::new(file).take(attribute.data.size);

        let mut result: Vec<f32> = Vec::with_capacity((attribute.data.size / 4) as usize);
        let mut convert_buf = [0u8; 4];

        while let Ok(()) = reader.read_exact(&mut convert_buf) {
            let num = f32::from_le_bytes(convert_buf);
            result.push(num);
        }

        let mut max = f32::NEG_INFINITY;
        let mut min = f32::INFINITY;

        for num in result.iter() {
            if *num > max {
                max = *num;
            } else if *num < min {
                min = *num;
            }
        }

        dbg!(max, min);
    }
}
fn main() {
    let config = Config {
        base_path: PathBuf::from_str("/tmp/91383020").unwrap(),
    };

    config.load_meta();
}
