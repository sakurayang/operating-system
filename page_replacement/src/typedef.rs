use std::collections::vec_deque::VecDeque;
use std::time::SystemTime;

#[derive(Copy, Clone, Debug)]
pub struct Page {
    pub content: u32,
    pub last_use: SystemTime,
    pub in_queue: SystemTime,
}

impl Page {
    pub fn new(content: u32) -> Page {
        Page {
            content,
            last_use: SystemTime::now(),
            in_queue: SystemTime::now(),
        }
    }
    pub fn to_string(&self) -> String {
        self.content.to_string()
    }
}

pub type Queue = Vec<Page>;
pub type RunResult = Result<(), Option<(usize, Page)>>;
pub type ResultQueue = Vec<RunResult>;
pub type Memory = VecDeque<Page>;
