#![allow(unused)]
use serde::Deserialize;
use std::{
    collections::HashMap,
    fmt::{write, Display},
};

#[derive(Debug, Deserialize)]
pub enum ItemType {
    GEOMETRY,
}

#[derive(Debug, Deserialize)]
pub enum Domain {
    POINT,
    EDGE,
    FACE,
    CORNER,
}

impl Display for Domain {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Domain::POINT => write!(f, "{}", "POINT"),
            Domain::EDGE => write!(f, "{}", "EDGE"),
            Domain::FACE => write!(f, "{}", "FACE"),
            Domain::CORNER => write!(f, "{}", "CORNER"),
        }
    }
}

#[derive(Debug, Deserialize)]
#[allow(non_camel_case_types)]
pub enum AttributeType {
    FLOAT,
    BOOLEAN,
    INT,
    FLOAT_VECTOR,
    INT32_2D,
}

impl Display for AttributeType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            AttributeType::FLOAT => write!(f, "{}", "FLOAT"),
            AttributeType::BOOLEAN => write!(f, "{}", "BOOLEAN"),
            AttributeType::INT => write!(f, "{}", "INT"),
            AttributeType::FLOAT_VECTOR => write!(f, "{}", "FLOAT_VECTOR"),
            AttributeType::INT32_2D => write!(f, "{}", "INT32_2d"),
        }
    }
}

pub enum AttributeData {
    FLOAT(Vec<f32>),
    BOOL(Vec<bool>),
    INT(Vec<i32>),
}

#[derive(Debug, Deserialize)]
pub struct BlobData {
    pub name: String,
    pub start: u64,
    pub size: u64,
}
#[derive(Debug, Deserialize)]
pub struct RawAttribute {
    pub name: String,
    pub domain: Domain,
    #[serde(rename = "type")]
    pub attribute_type: AttributeType,
    pub data: BlobData,
}
#[derive(Debug, Deserialize)]
pub struct MeshData {
    pub num_vertices: u64,
    pub num_edges: u64,
    pub num_polygons: u64,
    pub num_corners: u64,
    poly_offsets: BlobData,
    pub attributes: Vec<RawAttribute>,
}

#[derive(Debug, Deserialize)]
pub struct ItemData {
    pub mesh: MeshData,
}

#[derive(Debug, Deserialize)]
pub struct Item {
    pub name: String,
    #[serde(rename = "type")]
    pub item_type: ItemType,
    pub data: ItemData,
}

#[derive(Debug, Deserialize)]
pub struct BakeMetadata {
    version: u8,
    pub items: HashMap<String, Item>,
}

pub struct Attribute {
    pub name: String,
    pub domain: Domain,

    pub attribute_type: AttributeType,
    pub data: AttributeData,
}

pub struct Domains {
    pub point: HashMap<String, Attribute>,
    pub edge: HashMap<String, Attribute>,
    pub face: HashMap<String, Attribute>,
    pub corner: HashMap<String, Attribute>,
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

pub struct GeometryFrame {
    pub mesh: MeshData,
    pub domain: Domains,
    pub frame: u32,
}

impl From<Item> for GeometryFrame {
    fn from(value: Item) -> Self {
        GeometryFrame {
            mesh: value.data.mesh,
            domain: Domains::new(),
            frame: 0u32,
        }
    }
}
