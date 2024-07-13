use std::{borrow::Borrow, collections::HashMap, iter};

use crate::api::{AttributeType, Domain, MeshData};

#[derive(Debug)]
pub enum AttributeData {
    FLOAT(Vec<f32>),
    BOOL(Vec<bool>),
    INT(Vec<i32>),
}

#[derive(Debug)]
pub enum AttributeDataType {
    FLOAT(f32),
    BOOL(bool),
    INT(i32),
}

trait WrapInDataType {
    fn wrap_into_attribute_data_type(self) -> AttributeDataType;
}

impl WrapInDataType for f32 {
    fn wrap_into_attribute_data_type(self) -> AttributeDataType {
        AttributeDataType::FLOAT(self)
    }
}

impl WrapInDataType for bool {
    fn wrap_into_attribute_data_type(self) -> AttributeDataType {
        AttributeDataType::BOOL(self)
    }
}

impl WrapInDataType for i32 {
    fn wrap_into_attribute_data_type(self) -> AttributeDataType {
        AttributeDataType::INT(self)
    }
}
pub struct Attribute {
    pub name: String,
    pub domain: Domain,
    pub frame: u32,
    pub attribute_type: AttributeType,
    pub data: AttributeData,
}

pub struct Domains {
    pub point: HashMap<String, Vec<Attribute>>,
    pub edge: HashMap<String, Vec<Attribute>>,
    pub face: HashMap<String, Vec<Attribute>>,
    pub corner: HashMap<String, Vec<Attribute>>,
}

impl Domains {
    pub fn new() -> Self {
        Domains {
            point: HashMap::new(),
            edge: HashMap::new(),
            face: HashMap::new(),
            corner: HashMap::new(),
        }
    }
}

pub struct GeometryBuilder {
    pub mesh: Option<MeshData>,
    pub domain: Domains,
}

impl GeometryBuilder {
    pub fn new() -> Self {
        GeometryBuilder {
            mesh: None,
            domain: Domains::new(),
        }
    }

    pub fn sort_frames(&mut self) {
        for (_, value) in self.domain.point.iter_mut() {
            value.sort_by(|a, b| a.frame.cmp(&b.frame));
        }

        for (_, value) in self.domain.edge.iter_mut() {
            value.sort_by(|a, b| a.frame.cmp(&b.frame));
        }

        for (_, value) in self.domain.face.iter_mut() {
            value.sort_by(|a, b| a.frame.cmp(&b.frame));
        }

        for (_, value) in self.domain.corner.iter_mut() {
            value.sort_by(|a, b| a.frame.cmp(&b.frame));
        }
    }

    pub fn build(self) -> Geometry {
        self.into()
    }
}

pub struct Geometry {
    pub mesh: MeshData,
    pub domain: Domains,
}

impl From<GeometryBuilder> for Geometry {
    fn from(value: GeometryBuilder) -> Self {
        Geometry {
            mesh: value.mesh.unwrap(),
            domain: value.domain,
        }
    }
}

fn insert_data<'a, 'b, T>(
    data: &Vec<T>,
    points: &'b mut HashMap<usize, Point<'a>>,
    frame: usize,
    key: &'a str,
) where
    T: WrapInDataType + Copy,
{
    for (idx, value) in data.iter().enumerate() {
        let entry = points.entry(idx).or_insert(Point {
            frame,
            index: idx,
            data: HashMap::new(),
        });

        entry
            .data
            .insert(key, value.wrap_into_attribute_data_type());
    }
}

impl Geometry {
    pub fn points(&self, frame: usize) -> Vec<Point> {
        let mut points: HashMap<usize, Point> =
            HashMap::with_capacity(self.mesh.num_vertices as usize);

        for (key, value) in self.domain.point.iter() {
            let attribute = value.get(frame).unwrap();

            match &attribute.data {
                AttributeData::FLOAT(data) => {
                    insert_data(data, &mut points, frame, key);
                }
                AttributeData::BOOL(data) => {
                    insert_data(data, &mut points, frame, key);
                }
                AttributeData::INT(data) => {
                    insert_data(data, &mut points, frame, key);
                }
            }
        }
        let mut points: Vec<Point> = points.into_values().collect();
        points.sort_by(|a, b| a.index.cmp(&b.index));
        points
    }
}

#[derive(Debug)]
pub struct Point<'a> {
    frame: usize,
    index: usize,
    data: HashMap<&'a str, AttributeDataType>,
}
