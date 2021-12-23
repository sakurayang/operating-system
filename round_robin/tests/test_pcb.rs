use ulid::Ulid;

use round_robin::typedef::{PCB, Time, TimeVec};

#[test]
fn pcb_new() {
    let need: Time = 10;
    let p = PCB::new(need);
    assert_eq!(p.need(), need);
}

#[test]
fn pcb_time() {
    let arrive_times: TimeVec = vec![0, 0, 0, 1, 2];
    let need_times: TimeVec = vec![3, 2, 1, 4, 8];
    let end_times: TimeVec = vec![0, 0, 0, 0, 0];
    let begin_times: TimeVec = vec![0];
}
