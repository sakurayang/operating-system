use crate::algorithm::typedef::{Address, Memory, MemoryTable};

/// return index number of next number
pub fn alloc(memory: &mut MemoryTable, process: &mut Memory, address: Address, index: usize) -> usize {
    let end = address + process.size;
    process.address.start = address;
    process.address.end = end.clone();

    let mut after_block = memory.remove(index).unwrap_or_default();
    let size = after_block.size;
    after_block.address.start += process.size;
    after_block.set_size(size - process.size);

    memory.insert(index, *process);
    if after_block.size > 0 { memory.insert(index + 1, after_block); }
    return if memory.len() - 1 > index { index + 1 } else { index };
}

fn merge(memory: &mut Memory, other: &Memory) {
    if memory.address.start > other.address.start {
        memory.address.start = other.address.start;
    }
    memory.set_size(memory.size + other.size);
}

pub fn free(memory: &mut MemoryTable, index: usize) -> Result<(), isize> {
    let process = memory[index];
    let front = if index == 0 { process.clone().set_size(0) } else { memory[index - 1] };
    let back = if memory.len() - 1 < index + 1 {
        let mut clone = process.clone();
        let end = clone.address.end;
        clone.address.start = end;
        clone.set_size(0)
    } else { memory[index + 1] };

    let mut space = Memory::new(0).set_size(process.size);
    let mut insert_pos = index;
    if process.flag() == 0 { return Err(-1); }

    if front.flag() == 1 && back.flag() == 1 {
        memory.remove(index);
        space.address.start = process.address.start;
        space.set_size(process.size);
    } else if front.flag() == 1 && back.flag() == 0 {
        memory.drain(index..=index + 1);
        merge(&mut space, &back);
    } else if front.flag() == 0 && back.flag() == 1 {
        insert_pos = index - 1;
        memory.drain(index - 1..=index);
        merge(&mut space, &front)
    } else if front.flag() == 0 && back.flag() == 0 {
        insert_pos = index - 1;
        memory.drain(index - 1..=index + 1);
        space = front.clone();
        merge(&mut space, &process);
        merge(&mut space, &back);
    } else { return Err(-2); }

    memory.insert(insert_pos, space);
    return Ok(());
}