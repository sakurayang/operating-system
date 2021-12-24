use page_replacement::algorithm::fifo::put_in;
use page_replacement::typedef::{Memory, Page, Queue, ResultQueue};

fn run(queue: Queue, memory: &mut Memory) -> (ResultQueue, u32) {
    let mut drop_queue: ResultQueue = ResultQueue::new();
    let mut failed_count: u32 = 0;
    for i in 0..queue.len() {
        let mut page = queue[i].clone();
        let result = put_in(memory, &mut page);
        if result.is_err() {
            failed_count += 1;
        }
        drop_queue.push(result);
    }
    return (drop_queue, failed_count);
}

#[test]
fn run_test() {
    const SIZE: usize = 4;
    let queue: Queue = vec![
        Page::new(7), // Mis
        Page::new(0), // Mis
        Page::new(1), // Mis
        Page::new(2), // Mis
        Page::new(0), // Hit
        Page::new(3), // Mis
        Page::new(0), // Hit
        Page::new(4), // Mis
        Page::new(2), // Hit
        Page::new(3), // Hit
        Page::new(0), // Hit
        Page::new(3), // Hit
        Page::new(2), // Hit
        Page::new(1),  // Mis
    ];
    let assume: ResultQueue = vec![
        Err(None),
        Err(None),
        Err(None),
        Err(None),
        Ok(()),
        Err(Some((0, queue[0]))),
        Ok(()),
        Err(Some((2, queue[2]))),
        Ok(()),
        Ok(()),
        Ok(()),
        Ok(()),
        Ok(()),
        Err(Some((2, queue[7]))),
    ];
    let assume_count = 7_u32;
    let mut memory: Memory = Memory::with_capacity(SIZE);
    let (result, result_count) = run(queue, &mut memory);
    assert_eq!(result.len(), assume.len());
    for i in 0..result.len() {
        let res = result[i];
        let ass = assume[i];
        assert_eq!(res.is_ok(), ass.is_ok());
        if res.is_err() {
            assert_eq!(res.unwrap_err().is_none(), ass.unwrap_err().is_none());
            if res.unwrap_err().is_some() {
                assert_eq!(res.unwrap_err().unwrap().0, ass.unwrap_err().unwrap().0);
                assert_eq!(res.unwrap_err().unwrap().1.content, ass.unwrap_err().unwrap().1.content);
            }
        }
    }
    assert_eq!(assume_count, result_count);
}