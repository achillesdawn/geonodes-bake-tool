use std::{fs, path::PathBuf, str::FromStr};

mod error;
pub mod api;
use api::BakeMetadata;
use error::MetaReadError;

struct BakeReader {
    base_path: PathBuf,
    attributes: &'static[&'static str],
}

impl BakeReader {
    pub fn new(base_path: &str) -> Self {
        BakeReader {
            base_path: PathBuf::from_str(base_path).unwrap(),
            attributes: &["light", "hit"]
        }
    }

    pub fn load_meta(&mut self) -> Result<(), MetaReadError> {
        let mut meta_path = self.base_path.clone();
        meta_path.push("meta");

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

        let Some(item) = bake_metadata.items.get("0") else {
            return Err(MetaReadError::ItemNotFound);
        };

        println!("{} {:?} ", item.name, item.item_type);

        for attribute in item.data.mesh.attributes.iter() {
            dbg!(&attribute.name);

            if !self.attributes.contains(&attribute.name.as_str()) {
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

        let buffer = map_results(result, self.col_size);

        let frame_number = attribute
            .data
            .name
            .split("_")
            .next()
            .unwrap_or("")
            .parse::<u32>()
            .map_err(|_| MetaReadError::ParseIntError)?;

        Ok(Frame {
            buffer,
            number: frame_number,
        })
    }
}
