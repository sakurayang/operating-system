use std::time::SystemTime;

use crate::typedef::{Memory, Page, RunResult};

/// ok -> hit, err -> mis and return replace page or none if not full
pub fn put_in(memory: &mut Memory, value: &mut Page) -> RunResult {
    if memory.clone().iter().find(|&&x| x.content == value.content).is_some() {
        return Ok(());
    }
    value.in_queue = SystemTime::now();
    return if memory.len() + 1 > memory.capacity() {
        let mut oldest = 0;
        for i in 1..memory.len() {
            if memory[i].in_queue < memory[oldest].in_queue {
                oldest = i;
            }
        }
        let temp = memory.remove(oldest).unwrap();
        memory.insert(oldest, *value);
        Err(Some((oldest, temp)))
    } else {
        memory.push_back(*value);
        Err(None)
    };
}
