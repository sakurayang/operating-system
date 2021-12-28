use crate::typedef::{DeviceList, PCB, TypeMap};

/// | result | stand |
/// |--------|-------|
/// | Err(0) | device type not found |
/// | Err(1) | all devices is busy |
/// | Err(255) | unknown err |
pub fn malloc(device_list: &mut DeviceList, type_map: &mut TypeMap, process: &mut PCB) -> Result<(), u8> {
    let need_type = process.need;
    let type_detail = type_map.get(&need_type);
    if type_detail.is_none() { return Err(0); }
    if type_detail.unwrap().idle <= 0 { return Err(1); }

    let device_index = &type_detail.clone().unwrap().device_index;
    let index = device_index.iter().find(|&&x| !device_list[x].using());
    if index.is_none() { return Err(1); }

    let i = *index.unwrap();
    let device = &mut device_list[i];
    device.take(process.clone());
    process.alloc = Some(i);

    let mut replace = type_detail.unwrap().clone();
    replace.idle -= 1;
    type_map.insert(need_type, replace);

    return Ok(());
}

/// | result | stand |
/// |--------|-------|
/// | Err(0) | process has not alloc device |
/// | Err(1) | all devices is busy |
/// | Err(255) | unknown err |
pub fn free(device_list: &mut DeviceList, type_map: &mut TypeMap, process: &mut PCB) -> Result<(), u8> {
    if process.alloc.is_none() { return Err(0); }
    let alloc_type = process.need;
    let device_index = process.alloc;
    if device_index.is_none() { return Err(0); }

    let device = &mut device_list[device_index.unwrap()];
    device.release();
    process.alloc = None;

    let mut replace = type_map.get(&alloc_type).unwrap().clone();
    replace.idle += 1;
    type_map.insert(alloc_type, replace);

    return Ok(());
}