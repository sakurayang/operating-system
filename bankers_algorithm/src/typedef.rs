use std::collections::HashMap;

use ulid::Ulid;

type Entity = Vec<i32>;
pub type Resource = Entity;
pub type Sequence = Vec<usize>;
pub type Available = Resource;

pub type ResourceMap = HashMap<usize, String>;

#[derive(Clone, Debug)]
pub struct Process {
    id: Ulid,
    pub max: Resource,
    pub allocation: Resource,
    pub need: Resource,
}

impl Process {
    pub fn new() -> Process {
        Process {
            id: Ulid::new(),
            max: vec![],
            allocation: vec![],
            need: vec![],
        }
    }
    pub fn id(self) -> Ulid {
        self.id
    }
}
