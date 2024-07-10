use std::{
    fs,
    io::{self, Read, Seek},
    path::PathBuf,
    str::FromStr,
};

pub mod api;
mod error;
use api::{Attribute, AttributeData, BakeMetadata, Geometry, RawAttribute};
use error::MetaReadError;

pub struct BakeReader {
    pub base_path: PathBuf,
    pub attributes: &'static [&'static str],
}

impl BakeReader {
    pub fn new(base_path: &str, attributes: &'static [&'static str]) -> Self {
        BakeReader {
            base_path: PathBuf::from_str(base_path).unwrap(),
            attributes,
        }
    }

    pub fn load_meta(&mut self) -> Result<Vec<Geometry>, MetaReadError> {
        let mut meta_path = self.base_path.clone();
        meta_path.push("meta");

        let mut geometries = Vec::new();

        for entry in fs::read_dir(meta_path).unwrap() {
            let entry = entry.unwrap();
            let metadata = entry.metadata().unwrap();

            if metadata.is_file() {
                let read_result = self.read_meta(entry.path());
                match read_result {
                    Ok(frame) => geometries.push(frame),
                    Err(err) => return Err(err),
                }
            }
        }

        // frames.sort_by(|a, b| a.number.cmp(&b.number));
        // self.frames = frames;
        Ok(geometries)
    }

    fn read_meta(&self, path: PathBuf) -> Result<Geometry, MetaReadError> {
        let file = fs::File::open(path)?;
        let mut bake_metadata: BakeMetadata = serde_json::from_reader(file)?;

        let Some((_, item)) = bake_metadata.items.remove_entry("0") else {
            return Err(MetaReadError::ItemNotFound);
        };

        let mut geometry: Geometry = match item.item_type {
            api::ItemType::GEOMETRY => item.into(),
        };

        let attributes = std::mem::take(&mut geometry.mesh.attributes);

        for raw_attribute in attributes.into_iter() {
            dbg!(&raw_attribute.name);

            if !self.attributes.contains(&raw_attribute.name.as_str()) {
                continue;
            }

            println!("Blob file {}", raw_attribute.data.name);
            println!(
                "Found {}, of domain {} and type {}",
                raw_attribute.name, raw_attribute.domain, raw_attribute.attribute_type
            );

            let attribute = self.read_blob(raw_attribute)?;
            geometry.attributes.push(attribute);
        }

        Ok(geometry)
    }

    fn read_blob(&self, attribute: RawAttribute) -> Result<Attribute, MetaReadError> {
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

        let data = match attribute.attribute_type {
            api::AttributeType::FLOAT => {
                let mut result: Vec<f32> = Vec::with_capacity((attribute.data.size / 4) as usize);
                let mut buffer = [0u8; 4];

                while let Ok(()) = reader.read_exact(&mut buffer) {
                    let num = f32::from_le_bytes(buffer);
                    result.push(num);
                }

                AttributeData::FLOAT(result)
            }
            api::AttributeType::BOOLEAN => {
                let mut result: Vec<u8> = Vec::with_capacity((attribute.data.size) as usize);

                reader
                    .read_to_end(&mut result)
                    .expect("could not read into boolean attribute");

                let result: Vec<bool> = result
                    .into_iter()
                    .map(|b| unsafe { std::mem::transmute(b) })
                    .collect();

                AttributeData::BOOL(result)
            }
            api::AttributeType::INT => todo!(),
            api::AttributeType::FLOAT_VECTOR => todo!(),
            api::AttributeType::INT32_2D => todo!(),
        };

        let result = Attribute {
            name: attribute.name,
            domain: attribute.domain,
            attribute_type: attribute.attribute_type,
            data,
        };

        Ok(result)
    }
}
