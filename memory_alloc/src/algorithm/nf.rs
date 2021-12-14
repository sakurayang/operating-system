//! # 循环首次适应分配算法 (Next Fit)
//!
//! 与FF算法区别就是，不是每次都从首次开始，而是从上次找到的空闲分区的下一个空闲分区开始。（第一次查找的话也是从首页开始）。
//!
//! - 特点：能使内存中的空闲区分布得较均匀。
//!
//! ## 实现
//!
//! Input:
//!
//! ```c
//!     blockSize[] = {5, 10, 20};
//!     processSize[] = {10, 20, 30};
//! ```
//!
//! Output:
//!
//! | Process No. |   Process Size  |  Block no.     |
//! |-------------|-----------------|----------------|
//! |  1          |   10            |  2             |
//! |  2          |   20            |  3             |
//! |  3          |   30            |  Not Allocated |
//!
//! 1. Input the number of memory blocks and their sizes and initializes all the blocks as free.
//! 2. Input the number of processes and their sizes.
//! 3. Start by picking each process and check if it can be assigned to the current block, if yes,
//!    allocate it the required memory and check for next process but from the block where we left not from starting.
//! 4. If the current block size is smaller then keep checking the further blocks.
//!
//! ---
//!
//! 1. 输入内存块的数量及它们的大小，并初始化为空。
//! 2. 输入进程及它们的大小
//! 3. 遍历每个进程，并检查它是否可以被分配到当前块中，即检查 进程的大小 <= 块的大小。
//! 4. 如果是，则分配并从当前块开始检查下一个进程。
//! 5. 如果不是，则继续检查更多的块。
//!

use crate::algorithm::typedef::{Address, Memory, MemoryTable};

pub fn get_last_alloc(space_table: MemoryTable) -> usize {
    let mut last_index: usize = 0;
    for index in 0..space_table.len() {
        let memory = space_table[index];
        if index == 0 || memory.flag() != 1 {
            continue;
        }
        if memory.id() > space_table[last_index].id() {
            last_index = index;
        }
    }
    return last_index;
}

pub fn calc_alloc_pos(table: MemoryTable, process: Memory) -> Result<(usize, Address), i32> {
    let last_index = get_last_alloc(table.clone());
    let need_memory = process.size;
    let mut _found = false;
    let mut _pos: Option<usize> = None;
    let mut _address: Option<Address> = None;

    let len = table.len();

    for i in last_index..len {
        if _found { break; }
        let block = table[i];
        if block.size < need_memory || block.flag() == 1 { continue; } else {
            _found = true;
            _address = Some(block.address.start);
            _pos = Some(i);
        }
    }
    if !_found {
        for i in 0..last_index {
            let block = table[i];
            if block.size < need_memory || block.flag() == 1 { continue; } else {
                _found = true;
                _address = Some(block.address.start);
                _pos = Some(i);
            }
        }
    }

    return if _pos.is_some() && _address.is_some() {
        Ok((_pos.unwrap(), _address.unwrap()))
    } else {
        Err(0)
    };
}
