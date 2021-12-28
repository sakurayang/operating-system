use std::collections::HashMap;

use device_malloc_n_release::typedef::{Device, TypeDetail, TypeMap};
use device_malloc_n_release::typedef::DeviceType::*;
use device_malloc_n_release::util::{print_device_table, print_type_list};

#[test]
fn type_table() {
    let mut counter = HashMap::from([
        (Printer, 0),
        (DiskReader, 0),
        (Input, 0),
        (Output, 0)
    ]);

    let mut type_map: TypeMap = HashMap::from([
        (Printer, TypeDetail::new(0)),
        (DiskReader, TypeDetail::new(0)),
        (Input, TypeDetail::new(0)),
        (Output, TypeDetail::new(0)),
    ]);

    let devices_list = [
        ("print0", Printer),
        ("print1", Printer),
        ("print2", Printer),
        ("print3", Printer),
        ("disk_read0", DiskReader),
        ("disk_read1", DiskReader),
        ("disk_read2", DiskReader),
        ("input0", Input),
        ("input1", Input),
        ("output0", Output)
    ].map(|(name, device_type)| {
        let c = counter.get(&device_type).unwrap() + 1;
        counter.insert(device_type, c);
        return Device::new(String::from(name), device_type);
    }).to_vec();

    for i in 0..devices_list.clone().len() {
        let t = devices_list[i].device_type();
        let total = type_map.get(&t).unwrap().total() + 1;
        let arr = &type_map.get(&t).unwrap().device_index;
        let mut replace = TypeDetail::new(total);
        let mut c = arr.clone();
        c.push(i);
        replace.device_index = c;
        type_map.insert(t, replace);
    }

    print_type_list(&type_map);
}

#[test]
fn device_table() {
    let devices_list = [
        ("print0", Printer),
        ("print1", Printer),
        ("print2", Printer),
        ("print3", Printer),
        ("disk_read0", DiskReader),
        ("disk_read1", DiskReader),
        ("disk_read2", DiskReader),
        ("input0", Input),
        ("input1", Input),
        ("output0", Output)
    ].map(|(name, device_type)| {
        return Device::new(String::from(name), device_type);
    }).to_vec();
    print_device_table(&devices_list);
}