use std::time::SystemTime;

use crate::typedef::{Memory, Page, RunResult};

pub fn put_in(memory: &mut Memory, value: &mut Page) -> RunResult {
    if memory.clone().iter().find(|&&x| x.content == value.content).is_some() {
        value.last_use = SystemTime::now();
        return Ok(());
    }
    value.in_queue = SystemTime::now();
    return if memory.len() + 1 > memory.capacity() {
        let mut least = 0;
        for i in 1..memory.len() {
            if memory[i].last_use < memory[least].last_use {
                least = i;
            }
        }
        let temp = memory.remove(least).unwrap();
        memory.insert(least, *value);
        Err(Some((least, temp)))
    } else {
        memory.push_back(*value);
        Err(None)
    };
}