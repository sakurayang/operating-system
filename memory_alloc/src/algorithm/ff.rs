//! # 适应分配算法 (First fit)
//!
//! 要求，空闲分区链以地址递增的顺序链接。每次从链首开始，直到找到第一个能满足要求的空闲分区为止。
//! 简单来说，就是，每次都从第一个开始顺序查找，找到一块区域可以满足要求的。
//!
//! - 优点：优先利用内存中低址部分的空闲分区，从而保留了高址部分的大空闲区，这为以后到达的大作业分配大的内存空间创造了条件。
//! - 缺点：低址部分不断被划分，会留下许多难以利用的，很小的空闲分区，称为碎片。而每次查找又都是从低址部分开始的，这无疑又会增加查找可用空闲分区时的开销。
//!
//! ## 实现
//!
//! Input:
//!
//! ```c
//!     blockSize[]   = {100, 500, 200, 300, 600};
//!     processSize[] = {212, 417, 112, 426};
//!  ```
//! Output:
//!
//! | Process No. | Process Size |  Block no.    |
//! |-------------|--------------|---------------|
//! |      0      |     212      |     1         |
//! |      1      |     417      |     4         |
//! |      2      |     112      |     1         |
//! |      3      |     426      | Not Allocated |
//!
//! 1. Input memory blocks with size and processes with size.
//! 2. Initialize all memory blocks as free.
//! 3. Start by picking each process and check if it can be assigned to current block.
//! 4. If size-of-process <= size-of-block if yes then assign and check for next process.
//! 5. If not then keep checking the further blocks.
//!
//! ---
//!
//! 1. 输入内存块和进程及它们的大小。
//! 2. 初始化所有内存块为空。
//! 3. 遍历每个进程，并检查它是否可以被分配到当前块中，即检查 进程的大小 <= 块的大小。
//! 4. 如果是，则分配，并从头开始检查下一个进程。
//! 5. 如果不是，则继续检查更多的块。
//!

use crate::algorithm::typedef::{Address, Memory, MemoryTable};

// impl CalcFn
pub fn calc_alloc_pos(space_table: MemoryTable, memory: Memory) -> Result<(usize, Address), i32> {
    let need_memory = memory.size;
    for i in 0..space_table.len() {
        let block = space_table[i];
        if block.size < need_memory || block.flag() == 1 {
            continue;
        } else {
            return Ok((i, block.address.start));
        }
    }
    return Err(0);
}
