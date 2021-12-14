//! # 生产者与消费者问题 (Producer and Consumer problem)
//!
//! > [wiki](https://en.wikipedia.org/wiki/Producer%E2%80%93consumer_problem)
//!
//! **生产者消费者问题**（Producer-consumer problem） ，也称 **有限缓冲问题**（Bounded-buffer problem），
//! 是一个多进程同步问题的经典案例。该问题描述了共享固定大小缓冲区的两个进程
//! ——即所谓的“生产者”和“消费者”——在实际运行时会发生的问题。生产者的主要作用是生成一定量的数据放到缓冲区中，
//! 然后重复此过程。与此同时，消费者也在缓冲区消耗这些数据。该问题的关键就是要保证生产者不会在缓冲区满时加入数据，
//! 消费者也不会在缓冲区中空时消耗数据。
//!
//! ## 实验相关事项
//!
//! - 本实验将不会使用 Rust 自带的进程并发相关的函数库
//! - 本实验根据要求将使用信号量来控制
//! - 本实验的缓冲区将使用先入先出
//!
//! ## 信号量相关算法
//!
//! ```r
//! semaphore mutex = 1;
//! semaphore fillCount = 0;
//! semaphore emptyCount = BUFFER_SIZE;
//!
//! procedure producer() {
//!     while (true) {
//!         item = produceItem();
//!         down(emptyCount);
//!             down(mutex);
//!                 putItemIntoBuffer(item);
//!             up(mutex);
//!         up(fillCount);
//!     }
//! }
//! procedure consumer() {
//!     while (true) {
//!         down(fillCount);
//!             down(mutex);
//!                 item = removeItemFromBuffer();
//!             up(mutex);
//!         up(emptyCount);
//!         consumeItem(item);
//!     }
//! }
//!
//! ```
//!

use getset::{CopyGetters, Getters, MutGetters, Setters};
use ulid::Ulid;

use crate::typedef::{Buffer, ProcessQueue, Product, Semaphore};
use crate::util::print_info;

// 缓冲区大小 (buffer size)
const SIZE: usize = 5;

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum ProcessType {
    PRODUCER,
    CONSUMER,
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum ProcessStatus {
    WAIT,
    RUN,
}

// 进程控制块 (Process Control Block)
#[derive(Copy, Clone, Debug, Getters, Setters, MutGetters, CopyGetters)]
pub struct PCB {
    // 系统标号
    #[getset(get)]
    pub id: Ulid,
    // 类型标号
    #[getset(set = "pub")]
    pub process_type: ProcessType,
    // 状态
    #[getset(set = "pub")]
    pub status: ProcessStatus,
    // 产品
    #[getset(set = "pub")]
    pub product: Product,
}

impl PCB {
    pub fn new() -> PCB {
        return PCB {
            id: Ulid::new(),
            process_type: ProcessType::PRODUCER,
            status: ProcessStatus::WAIT,
            product: ' ',
        };
    }
}

pub fn cycle(queue: &mut ProcessQueue) {
    let front = queue.pop_front().unwrap();
    queue.push_back(front);
}

fn produce(process: &mut PCB, buffer: &mut Buffer) {
    process.product = rand::random::<char>();
    // if s_size is 4, that means is nothing in buf
    buffer.push_back(process.product);
}

fn consume(buffer: &mut Buffer) {
    buffer.pop_front();
}

/// return true if success
pub fn run(process: &mut ProcessQueue) -> bool {
    if process.is_empty() {
        return false;
    }

    // 缓冲区
    let mut buffer: Buffer = Buffer::new();

    // 就绪队列
    let mut ready_list: ProcessQueue = ProcessQueue::new();
    // 生产者等待队列
    let mut producer_wait: ProcessQueue = ProcessQueue::new();
    // 消费者等待队列
    let mut consumer_wait: ProcessQueue = ProcessQueue::new();

    // 信号量 (semaphore)
    // 空位计数 （生产则减）
    let mut s_empty_count: Semaphore = SIZE as i32;
    // 填充计数 （生产则加）
    let mut s_fill_count: Semaphore = 0;

    // 反正先丢一个进去
    ready_list.push_back(process.pop_front().unwrap());

    while !ready_list.is_empty() && !process.is_empty() {
        let mut p: PCB = ready_list.pop_front().unwrap().clone();
        let mut _has_run: bool = false;
        match p.process_type {
            ProcessType::PRODUCER => {
                if s_empty_count <= 0 {
                    producer_wait.push_back(p);
                    _has_run = false;
                } else {
                    s_empty_count -= 1;
                    produce(&mut p, &mut buffer);
                    _has_run = true;
                    s_fill_count += 1;
                    if !consumer_wait.is_empty() {
                        ready_list.push_back(consumer_wait.pop_front().unwrap())
                    }
                }
            }
            ProcessType::CONSUMER => {
                if s_fill_count <= 0 {
                    consumer_wait.push_back(p);
                    _has_run = false;
                } else {
                    s_fill_count -= 1;
                    consume(&mut buffer);
                    _has_run = true;
                    s_empty_count += 1;
                    if !producer_wait.is_empty() {
                        ready_list.push_back(producer_wait.pop_front().unwrap())
                    }
                }
            }
        }
        if !process.is_empty() && ready_list.is_empty() {
            ready_list.push_back(process.pop_front().unwrap());
        }
        print_info(
            p.id.to_string(),
            p.process_type,
            p.product,
            producer_wait.len(),
            consumer_wait.len(),
            buffer.len(),
            _has_run,
        );
    }
    return true;
}
