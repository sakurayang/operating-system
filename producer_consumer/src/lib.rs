pub mod producer_consumer;
pub mod util;

pub mod typedef {
    use std::collections::vec_deque::VecDeque;
    use crate::producer_consumer::PCB;

    pub type Product = char;
    pub type Semaphore = i32;
    pub type Buffer = VecDeque<Product>;
    pub type ProcessQueue = VecDeque<PCB>;
}

