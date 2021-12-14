use memory_alloc::algorithm::bf::calc_alloc_pos;
use memory_alloc::algorithm::typedef::{Memory, MemoryTable};

/// 1. init [100, 500, 200, 300, 600] --alloc(212)-->
/// 2.   3  [100, 500, 200, [*212, 88], 600] --alloc(417)-->
/// 3.   1 [100, [*417, 83], 200, [*212, 88], 600] --alloc(112)-->
/// 4.   2 [100, [*417, 83], [*112, 88], [*212, 88], 600] --alloc(426)-->
/// 5.   4 [100, [*417, 83], [*112, 88], [*212, 88], [*426, 174]] end
///
#[test]
fn test() {
    let mut memories: MemoryTable = MemoryTable::from([
        Memory::new(0).set_size(100),
        Memory::new(0).set_size(500),
        Memory::new(0).set_size(200),
        Memory::new(0).set_size(300),
        Memory::new(0).set_size(600),
    ]);

    let processes_and_pos: [(Memory, usize); 4] = [
        (Memory::new(1).set_size(212), 3),
        (Memory::new(1).set_size(417), 1),
        (Memory::new(1).set_size(112), 2),
        (Memory::new(1).set_size(426), 4),
    ];

    for (process, assert_pos) in processes_and_pos {
        println!("testing {:?}, assert {}", process, assert_pos);
        let res = calc_alloc_pos(memories.clone(), process);
        if res.is_ok() {
            let (pos, _) = res.unwrap();
            let old_size = memories[pos].size;
            memories[pos].set_size(old_size - process.size);
            assert_eq!(pos, assert_pos);
        } else {
            assert_eq!(res, Err(0));
        }
    }
}
