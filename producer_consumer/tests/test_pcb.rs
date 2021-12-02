use producer_consumer::producer_consumer::{ProcessStatus, ProcessType, PCB};
use rand::Rng;

#[test]
fn create_producer() {
    let t = if rand::thread_rng().gen_range(0..=1) == 0 {
        ProcessType::CONSUMER
    } else {
        ProcessType::PRODUCER
    };
    let s = if rand::thread_rng().gen_range(0..=1) == 0 {
        ProcessStatus::WAIT
    } else {
        ProcessStatus::RUN
    };
    let p = 'A';

    let mut pcb = PCB::new();
    pcb.set_process_type(t).set_status(s).set_product(p);

    assert_eq!(pcb.process_type, t);
    assert_eq!(pcb.status, s);
    assert_eq!(pcb.product, p);
}
