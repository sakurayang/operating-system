use bankers_algorithm::bankers_algorithm::{alloc_calc, is_safe};
use bankers_algorithm::typedef::{Available, Process, Resource};

/// 示例：
///
/// 考虑有进程表如下
///
/// ``` text
/// | Process | Allocation |    Max     | Available  |
/// |---------|------------|------------|------------|
/// |         |  A   B   C |  A   B   C |  A   B   C |
/// |---------|------------|------------|------------|
/// |    P0   |  0   1   0 |  7   5   3 |  3   3   2 |
/// |    P1   |  2   0   0 |  3   2   2 |            |
/// |    P2   |  3   0   2 |  9   0   2 |            |
/// |    P3   |  2   1   1 |  2   2   2 |            |
/// |    P4   |  0   0   2 |  4   3   3 |            |
/// ```
///
/// 可求出需求表如下
///
/// ```text
/// | Process |    Need   |
/// |---------|-----------|
/// |         | A | B | C |
/// |---------|---|---|---|
/// |    P0   | 7 | 4 | 3 |
/// |    P1   | 1 | 2 | 2 |
/// |    P2   | 6 | 0 | 0 |
/// |    P3   | 0 | 1 | 1 |
/// |    P4   | 4 | 3 | 1 |
/// ```
///
/// 可得出安全序列为 `P1 -> P3 -> P4 -> P0 -> P2`
/// (其实 `3 -> 1 -> 4 -> 0 -> 2` 也是可以的)
///
#[test]
fn test_unsafe() {
    const PROCESS_COUNT: usize = 3;
    const RESOURCE_COUNT: usize = 3;
    let available: Available = vec![0, 0, 0];
    let allocation = [
        [0, 1, 0],
        [2, 0, 0],
        [3, 0, 2]
    ];
    let max = [
        [7, 5, 3],
        [3, 2, 2],
        [9, 0, 2]
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
    let safe = is_safe(available, processes);
    assert_eq!(safe, false);
}

#[test]
fn test_safe() {
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
    let safe = is_safe(available, processes);
    assert_eq!(safe, true);
}

#[test]
fn test_alloc() {
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

    let sequence = vec![1, 3, 4, 0, 2];
    assert_eq!(alloc_calc(available, processes), Ok(sequence));
}