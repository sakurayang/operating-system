use prettytable::{cell, row, Table};
use strum::EnumCount;

use crate::typedef::{DeviceList, DeviceType, PCB, TypeMap};

pub fn print_type_list_without_count() {
    let mut table = Table::new();
    let header = row!["编号", "设备类型"];
    table.set_titles(header);

    for i in 0..DeviceType::COUNT as usize {
        let device_type = DeviceType::from_repr(i).unwrap_or(DeviceType::Printer);
        let row = row![
            FY -> i.to_string(),
            FC -> device_type.to_string(),
        ];
        table.add_row(row);
    }
    table.printstd();
}

pub fn print_type_list(type_map: &TypeMap) {
    let mut table = Table::new();
    let header = row!["编号", "设备类型","设备总数","空闲设备数","设备编号"];
    table.set_titles(header);

    for (device_type, type_detail) in type_map {
        let index_str: String = type_detail.device_index
            .iter()
            .map(|x| x.to_string())
            .collect::<Vec<String>>()
            .join(",");
        let row = row![
            FY -> (*device_type as usize).to_string(),
            FC -> device_type.to_string(),
            Fy -> type_detail.total().to_string(),
            FY -> type_detail.idle.to_string(),
            Fb -> index_str
        ];
        table.add_row(row);
    }
    table.printstd();
}

pub fn print_device_table(device_table: &DeviceList) {
    let mut table = Table::new();
    let header = row!["设备编号","设备名","是否已分配","占用进程"];
    table.set_titles(header);

    for i in 0..device_table.len() {
        let device = &device_table[i];
        let using_str: String = match device.using_process() {
            None => String::from(" "),
            Some(a) => a.id().to_string()
        };
        let row = row![
            Fy -> i.to_string(),
            FB -> device.name(),
            FR -> device.using().to_string(),
            FC -> using_str
        ];
        table.add_row(row);
    }
    table.printstd();
}

pub fn print_processes(processes: Vec<PCB>) {
    let mut table = Table::new();
    let header = row!["进程编号", "进程ID", "需求类型", "已分配设备"];
    table.set_titles(header);

    for i in 0..processes.len() {
        let process = &processes[i];
        let device_str: String = match process.alloc {
            None => String::from(" "),
            Some(i) => i.to_string()
        };
        let row = row![
            Fy -> i.to_string(),
            FC -> process.id().to_string(),
            Fb -> process.need,
            Fr -> device_str
        ];
        table.add_row(row);
    }
    table.printstd();
}
