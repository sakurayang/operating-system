use std::collections::HashMap;

use bankers_algorithm::typedef::{Available, Process, Resource, ResourceMap};
use bankers_algorithm::util::display_processes_table;

#[test]
fn processes_print() {
    const PROCESS_COUNT: usize = 5;
    const RESOURCE_COUNT: usize = 3;
    let available: Available = vec![3, 3, 2];
    let mut allocation = [
        [0, 1, 0],
        [2, 0, 0],
        [3, 0, 2],
        [2, 1, 1],
        [0, 0, 2]
    ];
    let max = [
        [7, 5, 3],
        [3, 2, 2],
        [9, 0, 2],
        [2, 2, 2],
        [4, 3, 3]
    ];
    let mut processes: Vec<Process> = Vec::new();
    for i in 0..PROCESS_COUNT {
        let mut process: Process = Process::new();
        process.max = Vec::from(max[i]);
        process.allocation = Vec::from(allocation[i]);
        let mut need: Resource = vec![0; RESOURCE_COUNT];
        for j in 0..RESOURCE_COUNT {
            need[j] = max[i][j] - allocation[i][j];
        }
        process.need = need;
        processes.push(process);
    }
    let map: ResourceMap = HashMap::from([
        (0, String::from("a")),
        (1, String::from("b")),
        (2, String::from("c"))
    ]);
    display_processes_table(&processes, &map);
}