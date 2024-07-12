use std::{
    fs::{self, File},
    io::{self, BufReader, Read, Seek, Take},
    path::PathBuf,
    str::FromStr,
};

pub mod api;
mod error;
use api::{Attribute, AttributeData, BakeMetadata, Geometry, GeometryFrame, RawAttribute};
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

    pub fn load_meta(&mut self) -> Result<Geometry, MetaReadError> {
        let mut meta_path = self.base_path.clone();
        meta_path.push("meta");

        let mut geometries = Vec::new();

        for entry in fs::read_dir(meta_path).unwrap() {
            let entry = entry.unwrap();
            let metadata = entry.metadata().unwrap();

            if metadata.is_file() {
                let read_result = self.read_meta(entry.path());
                match read_result {
                    Ok(geometry) => {
                        geometries.push(geometry);
                    }
                    Err(err) => return Err(err),
                }
            }
        }

        geometries.sort_by(|a, b| a.frame.cmp(&b.frame));

        let geometry: Geometry = geometries.into();
        // frames.sort_by(|a, b| a.number.cmp(&b.number));
        // self.frames = frames;
        Ok(geometry)
    }

    fn read_meta(&self, path: PathBuf) -> Result<GeometryFrame, MetaReadError> {
        let frame_number = get_frame_number(&path)?;

        let file = fs::File::open(path)?;
        let mut bake_metadata: BakeMetadata = serde_json::from_reader(file)?;

        let Some((_, item)) = bake_metadata.items.remove_entry("0") else {
            return Err(MetaReadError::ItemNotFound);
        };

        let mut frame: GeometryFrame = match item.item_type {
            api::ItemType::GEOMETRY => item.into(),
        };

        frame.frame = frame_number;

        let attributes = std::mem::take(&mut frame.mesh.attributes);

        for mut raw_attribute in attributes.into_iter() {
            dbg!(&raw_attribute.name);

            if !self.attributes.contains(&raw_attribute.name.as_str()) {
                continue;
            }

            println!("Blob file {}", raw_attribute.data.name);
            println!(
                "Found {}, of domain {} and type {}",
                raw_attribute.name, raw_attribute.domain, raw_attribute.attribute_type
            );

            let entry = match &raw_attribute.domain {
                api::Domain::POINT => frame
                    .domain
                    .point
                    .entry(std::mem::take(&mut raw_attribute.name)),
                api::Domain::EDGE => frame
                    .domain
                    .edge
                    .entry(std::mem::take(&mut raw_attribute.name)),
                api::Domain::FACE => frame
                    .domain
                    .face
                    .entry(std::mem::take(&mut raw_attribute.name)),
                api::Domain::CORNER => frame
                    .domain
                    .corner
                    .entry(std::mem::take(&mut raw_attribute.name)),
            };

            let attribute = self.read_blob(raw_attribute)?;
            entry.or_insert_with(|| attribute);
        }

        Ok(frame)
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

        let reader = io::BufReader::new(file).take(attribute.data.size);

        let data = match attribute.attribute_type {
            api::AttributeType::FLOAT => read_float(reader, attribute.data.size),
            api::AttributeType::BOOLEAN => read_bool(reader, attribute.data.size),
            api::AttributeType::INT => {
                // similar to float, (4 byte) ints together
                todo!()
            }
            api::AttributeType::FLOAT_VECTOR => {
                // similar to float, 3 (4 byte) floats together
                todo!()
            }
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

fn get_frame_number(path: &PathBuf) -> Result<u32, MetaReadError> {
    let frame_number = path
        .file_name()
        .unwrap_or_default()
        .to_str()
        .unwrap()
        .split("_")
        .next()
        .unwrap_or("")
        .parse::<u32>()
        .map_err(|_| MetaReadError::ParseIntError)?;

    Ok(frame_number)
}

fn read_float(mut reader: Take<BufReader<File>>, size: u64) -> AttributeData {
    let mut result: Vec<f32> = Vec::with_capacity((size / 4) as usize);
    let mut buffer = [0u8; 4];

    while let Ok(()) = reader.read_exact(&mut buffer) {
        let num = f32::from_le_bytes(buffer);
        result.push(num);
    }

    AttributeData::FLOAT(result)
}

fn read_bool(mut reader: Take<BufReader<File>>, size: u64) -> AttributeData {
    let mut result: Vec<u8> = Vec::with_capacity(size as usize);

    reader
        .read_to_end(&mut result)
        .expect("could not read into boolean attribute");

    let result: Vec<bool> = result
        .into_iter()
        .map(|b| unsafe { std::mem::transmute(b) })
        .collect();

    AttributeData::BOOL(result)
}
