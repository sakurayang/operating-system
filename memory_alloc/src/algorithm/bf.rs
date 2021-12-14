//! # 最佳适配法(Best Fit)
//!
//! 将所有空闲分区按照空闲分区容量大小从小到大的顺序连接起来，形成一个空闲分区链。
//!
//! 即，每次都是找空间容量不但可以满足要求的空闲区，而且该空闲分区的容量还要最接近要求的容量大小。
//!
//! - 优点：每次分配给文件的都是最合适该文件大小的分区。
//! - 缺点：内存中留下许多难以利用的小的空闲区（外碎片）。
//!
//! ## 实现
//!
//! > [geeks for geeks](https://www.geeksforgeeks.org/program-best-fit-algorithm-memory-management/)
//!
//! Input:
//! ```c
//!    blockSize[]   = {100, 500, 200, 300, 600};
//!    processSize[] = {212, 417, 112, 426};
//! ```
//! Output:
//!
//! | Process No. | Process Size | Block no. |
//! |---|-----|---|
//! | 0 | 212 | 3 |
//! | 1 | 417 | 1 |
//! | 2 | 112 | 2 |
//! | 3 | 426 | 4 |
//!
//! 1. Input memory blocks and processes with sizes.
//! 2. Initialize all memory blocks as free.
//! 3. Start by picking each process and find the minimum block size that can be assigned to
//!    current process i.e., find
//!    `min(bockSize[1],blockSize[2],.....blockSize[n]) > processSize[current]`.
//! 4. If found then assign it to the current process.
//! 5. If not then leave that process and keep checking the further processes.
//!
//! ---
//!
//! 1. 输入内存块和进程及它们的大小。
//! 2. 初始化所有内存块为空。
//! 3. 遍历每个进程，找到可分配给当前进程的最小块大小，即找到
//!    `min(bockSize[1], blockSize[2],.....blockSize[n]) > processSize[current]`，
//! 4. 如果找到，则把它分配给当前进程。
//! 5. 如果没找到，就放置当前进程，转入下一个。
//!

use crate::algorithm::typedef::{Address, Memory, MemoryTable};

pub fn calc_alloc_pos(memory: MemoryTable, process: Memory) -> Result<(usize, Address), i32> {
    let need_memory = process.size;
    let mut min_index: usize = 0;

    for index in 0..memory.len() {
        if memory[min_index].size < need_memory { min_index = index; }
        let block = memory[index];
        if block.flag() == 1 { continue; }
        if block.size >= need_memory && block.flag() == 0 {
            if memory[min_index].flag() == 1 || block.size <= memory[min_index].size {
                min_index = index;
            }
        }
    }

    return Ok((min_index, memory[min_index].address.start));
}
