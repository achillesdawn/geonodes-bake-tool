use std::collections::HashMap;

use crate::api::{AttributeType, Domain, MeshData};


#[derive(Debug)]
pub enum AttributeData {
    FLOAT(Vec<f32>),
    BOOL(Vec<bool>),
    INT(Vec<i32>),
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
        GeometryBuilder{
            mesh: None,
            domain: Domains::new()
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
        Geometry{
            mesh: value.mesh.unwrap(),
            domain: value.domain
        }
    }
}