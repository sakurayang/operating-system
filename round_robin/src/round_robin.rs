use crate::typedef::{PCB, Sequence, Time};

fn get_longest_process(processes: Vec<PCB>) -> usize {
    let mut index: usize = 0;
    let len = &processes.len();
    for i in (0..*len).rev() {
        if &processes[index].need() < &processes[i].need() {
            index = i;
        }
    }
    return index;
}

pub fn run(processes: &mut Vec<PCB>) -> Sequence {
    let mut sequence: Sequence = Sequence::new();
    let longest_process = processes[get_longest_process(processes.clone())];
    let mut current: Time = 0;

    for _ in 0..longest_process.need() {
        for i in 0..processes.len() {
            let mut process = processes[i];
            let can_run = process.run(current);
            if can_run {
                sequence.push(i);
                current += 1;
                processes[i] = process;
            }
        }
    }
    return sequence;
}
