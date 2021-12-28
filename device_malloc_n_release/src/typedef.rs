use std::collections::HashMap;
use std::ops::Deref;
use std::string::ToString;

use strum::{Display, EnumCount, EnumIter, FromRepr};
use strum::IntoEnumIterator;
use ulid::Ulid;

#[derive(Debug, Copy, Clone, Hash, Eq, PartialEq, EnumCount, EnumIter, FromRepr, Display)]
pub enum DeviceType {
    Printer,
    DiskReader,
    Input,
    Output,
}

#[derive(Clone, Debug, Hash)]
pub struct Device {
    name: String,
    physical_address: u32,
    using: bool,
    using_process: Option<PCB>,
    device_type: DeviceType,
}

#[derive(Clone, Debug)]
pub struct TypeDetail {
    total: u32,
    pub idle: u32,
    pub device_index: Vec<usize>,
}

#[derive(Clone, Debug, Hash)]
pub struct PCB {
    id: Ulid,
    pub need: DeviceType,
    pub alloc: Option<usize>,
}

pub type TypeMap = HashMap<DeviceType, TypeDetail>;
pub type DeviceList = Vec<Device>;

impl Device {
    pub fn new(name: String, device_type: DeviceType) -> Device {
        Device {
            name,
            physical_address: rand::random::<u32>(),
            using: false,
            using_process: None,
            device_type,
        }
    }

    pub fn name(&self) -> String {
        self.name.clone()
    }

    pub fn physical_address(&self) -> u32 {
        self.physical_address
    }

    pub fn using(&self) -> bool {
        self.using
    }

    pub fn using_process(&self) -> Option<PCB> {
        self.clone().using_process
    }

    pub fn device_type(&self) -> DeviceType {
        self.device_type
    }

    pub fn take(&mut self, process: PCB) -> Self {
        self.using = true;
        self.using_process = Some(process);
        self.deref().clone()
    }

    pub fn release(&mut self) -> Self {
        self.using = false;
        self.using_process = None;
        self.deref().clone()
    }
}

impl PCB {
    pub fn new(need: DeviceType) -> PCB {
        PCB {
            id: Ulid::new(),
            need,
            alloc: None,
        }
    }
    pub fn id(&self) -> Ulid {
        self.id
    }
}

impl TypeDetail {
    pub fn new(total: u32) -> TypeDetail {
        TypeDetail {
            total,
            idle: total,
            device_index: vec![],
        }
    }
    pub fn total(&self) -> u32 {
        self.total
    }
}