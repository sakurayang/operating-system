use memory_alloc::algorithm::nf::get_last_alloc;
use memory_alloc::algorithm::typedef::{Memory, MemoryTable};

#[test]
fn test_get_last() {
    let memories = [
        Memory::new(1).set_size(100),
        Memory::new(1).set_size(200),
        Memory::new(0).set_size(300),
        Memory::new(1).set_size(400),
        Memory::new(0).set_size(500),
    ];
    let table = MemoryTable::from(memories);
    assert_eq!(get_last_alloc(table), 3);
}
