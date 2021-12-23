use round_robin::round_robin::run;
use round_robin::typedef::{PCB, Sequence};

/// input:
///
/// | 进程  | 持续时间 | 顺序  | 到达时间 |
/// |-----|------|-----|------|
/// | P1  | 3    | 1   | 0    |
/// | P2  | 4    | 2   | 0    |
/// | P3  | 3    | 3   | 0    |
///
/// output: `1, 2, 3, 1, 2, 3, 1, 2, 3, 2`
///
#[test]
fn test_run() {
    let mut processes = vec![
        PCB::new(3),
        PCB::new(4),
        PCB::new(3),
    ];
    let seq: Sequence = vec![0, 1, 2, 0, 1, 2, 0, 1, 2, 1];
    let res = run(&mut processes);
    assert_eq!(res, seq);
}