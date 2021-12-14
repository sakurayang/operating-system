use std::collections::vec_deque::VecDeque;

use getset::{CopyGetters, Setters};
use ulid::Ulid;

pub type Address = u32;
pub type Size = u32;

pub enum Algorithm {
    FirstFit,
    BestFit,
    NextFit,
}

#[derive(Copy, Clone, Debug, Setters, Default)]
pub struct AddressSection {
    #[getset(set = "pub")]
    pub start: Address,
    #[getset(set = "pub")]
    pub end: Address,
}

#[derive(Copy, Clone, Debug, CopyGetters, Setters, Default)]
pub struct Memory {
    #[getset(get_copy = "pub")]
    id: Ulid,
    /// memory type, 0 is space, 1 is process
    #[getset(get_copy = "pub")]
    flag: u8,
    pub address: AddressSection,
    pub size: Size,
}

impl Memory {
    pub fn new(flag: u8) -> Memory {
        Memory {
            id: Ulid::new(),
            flag,
            address: AddressSection { start: 0, end: 0 },
            size: 0,
        }
    }
    pub fn actual_size(&self) -> u32 {
        self.address.end - self.address.start
    }
    fn set_address(&mut self, start: Address, end: Address) -> Self {
        self.address = AddressSection { start, end };
        return *self;
    }
    pub fn set_size(&mut self, size: Size) -> Self {
        self.size = size;
        let start = self.address.start;
        let end = start + size;
        return self.set_address(start, end);
    }
}

pub type MemoryTable = VecDeque<Memory>;
