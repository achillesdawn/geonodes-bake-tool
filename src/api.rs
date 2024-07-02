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
pub(crate) struct BlobData {
    pub name: String,
    pub start: u64,
    pub size: u64,
}
#[derive(Debug, Deserialize)]
pub(crate) struct Attribute {
    pub name: String,
    pub domain: Domain,
    #[serde(rename = "type")]
    pub attribute_type: String,
    pub data: BlobData,
}
#[derive(Debug, Deserialize)]
pub(crate) struct MeshData {
    num_vertices: u64,
    num_edges: u64,
    num_polygons: u64,
    num_corners: u64,
    poly_offsets: BlobData,
    pub attributes: Vec<Attribute>,
}
#[derive(Debug, Deserialize)]
pub(crate) struct ItemData {
    pub mesh: MeshData,
}
#[derive(Debug, Deserialize)]
pub(crate) struct Item {
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

pub struct Frame {
    pub number: u32,
    pub buffer: String
}
