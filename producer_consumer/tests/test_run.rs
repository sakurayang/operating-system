use producer_consumer::producer_consumer::*;
use producer_consumer::typedef::*;

#[test]
fn test_cycle() {
    let p1 = PCB::new();
    let p2 = PCB::new();
    let p3 = PCB::new();
    let mut queue = ProcessQueue::from([p1, p2, p3]);
    let mut test_queue = ProcessQueue::from([p1, p2, p3]);
    cycle(&mut test_queue);
    for i in 0..test_queue.len() {
        assert_eq!(
            queue.get((i + 1) % 3).unwrap().id,
            test_queue.get(i).unwrap().id
        );
    }
}
