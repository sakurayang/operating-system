use memory_alloc::algorithm::nf::calc_alloc_pos;
use memory_alloc::algorithm::typedef::{Memory, MemoryTable};

/// 1. init [5, 10, 20, 5] --alloc(10)-->
/// 2.   1  [5, *10, 20, 5] --alloc(5)-->
/// 3.   2  [5, *10, *5, 15, 5] --alloc(15)-->
/// 4.   3  [5, *10, *5, *15, 5] --alloc(5)-->
/// 5.   4  [5, *10, *5, *15, *5] --alloc(20)--> err
///
#[test]
fn test() {
    let mut memories: MemoryTable = MemoryTable::from([
        Memory::new(0).set_size(5),
        Memory::new(0).set_size(10),
        Memory::new(0).set_size(20),
        Memory::new(0).set_size(5)
    ]);

    let processes_and_pos: [(Memory, usize); 5] = [
        (Memory::new(1).set_size(10), 1),
        (Memory::new(1).set_size(5), 2),
        (Memory::new(1).set_size(15), 3),
        (Memory::new(1).set_size(5), 4),
        (Memory::new(1).set_size(20), 0),
    ];

    for (process, assert_pos) in processes_and_pos {
        println!("testing {:?}, assert {}", process, assert_pos);
        let res = calc_alloc_pos(memories.clone(), process);
        if res.is_ok() {
            let (pos, _) = res.unwrap();
            let old_size = memories[pos].size;
            if old_size == process.size { memories.remove(pos); } else { memories[pos].set_size(old_size - process.size); }
            memories.insert(pos, Memory::new(1).set_size(process.size));
            assert_eq!(pos, assert_pos);
        } else {
            assert_eq!(res, Err(0));
        }
    }
}
