use std::collections::HashMap;
use std::io::stdin;

use crossterm::style::Stylize;
use strum::IntoEnumIterator;

use device_malloc_n_release::device::{free, malloc};
use device_malloc_n_release::typedef::{Device, DeviceList, DeviceType, PCB, TypeDetail, TypeMap};
use device_malloc_n_release::util::{print_device_table, print_processes, print_type_list, print_type_list_without_count};

fn get_input() -> String {
    let mut input = String::new();
    stdin().read_line(&mut input).expect("Failed");
    input = input.trim().to_string();
    return input;
}

fn handle_device_input() -> DeviceList {
    let mut device_list: DeviceList = DeviceList::new();
    print_type_list_without_count();
    let format_text = format!("{}与{}", "设备类型编号".cyan().underlined(), "设备名".cyan().underlined());
    println!("请输入{}，空格分隔，按 o 结束", format_text);
    loop {
        let input = get_input();
        if input == "o" {
            return device_list;
        }
        let params: Vec<&str> = input.split(' ').collect();
        let type_id: usize = params[0].parse::<usize>().unwrap_or(0).to_owned();
        let name = params[1].to_owned();
        let t = DeviceType::from_repr(type_id).unwrap_or(DeviceType::Printer);
        let device = Device::new(name, t);
        device_list.push(device);
    }
}

fn handle_process_input(type_map: &TypeMap) -> Vec<PCB> {
    let mut processes: Vec<PCB> = Vec::new();
    print_type_list(type_map);
    println!("请输入进程所需的{}，按 o 结束", "设备类型".cyan().underlined());
    loop {
        let input = get_input();
        if input == "o" {
            return processes;
        }
        let params: Vec<&str> = input.split(' ').collect();
        let type_id: usize = params[0].parse::<usize>().unwrap_or(0).to_owned();
        let t = DeviceType::from_repr(type_id).unwrap_or(DeviceType::Printer);
        let process = PCB::new(t);
        processes.push(process);
    }
}

fn main() {
    let mut type_map: TypeMap = HashMap::new();
    for t in DeviceType::iter() {
        type_map.insert(t, TypeDetail::new(0));
    }

    let mut devices_list: DeviceList = handle_device_input();
    print_device_table(&devices_list);

    for i in 0..devices_list.clone().len() {
        let t = devices_list[i].device_type();

        // 总数计数
        let total = type_map.get(&t).unwrap().total() + 1;
        let arr = &type_map.get(&t).unwrap().device_index;
        let mut replace = TypeDetail::new(total);
        let mut c = arr.clone();
        c.push(i);
        replace.device_index = c;
        type_map.insert(t, replace);
    }


    let mut processes = handle_process_input(&type_map);
    print_processes(processes.clone());

    let mut wait_list: Vec<PCB> = vec![];

    for i in 0..processes.clone().len() {
        let mut process = processes[i].clone();
        let res = malloc(&mut devices_list, &mut type_map, &mut process);
        if res.is_err() {
            match res.unwrap_err() {
                0 => eprint!("进程 {} 所需的设备种类无法找到", &process.id().to_string()),
                1 => {
                    wait_list.push(process.clone());
                    eprint!("进程 {} 所需的设备无空闲，已加入等待队列", &process.id().to_string())
                },
                255 | _ => eprint!("unknown err")
            }
        }
        processes.remove(i);
        processes.insert(i, process);
    }
    println!("运行完毕，分配表如下：");
    print_processes(processes.clone());
    println!("等待队列如下：");
    print_processes(wait_list.clone());
    println!("是否释放设备并恢复等待队列中的进程运行?  {} ", "[Y]/n".blue());
    if get_input().to_lowercase() == "n" { return; }

    // release all
    for i in 0..processes.clone().len() {
        let mut process = processes[i].clone();
        free(&mut devices_list, &mut type_map, &mut process);
    }
    for i in 0..wait_list.clone().len() {
        let mut process = wait_list[i].clone();
        let res = malloc(&mut devices_list, &mut type_map, &mut process);
        if res.is_err() {
            match res.unwrap_err() {
                0 => eprintln!("进程 {} 所需的设备种类无法找到", &process.id().to_string()),
                1 => eprintln!("进程 {} 所需的设备无空闲", &process.id().to_string()),
                255 | _ => eprintln!("unknown err")
            }
        }
        processes.remove(i);
        processes.insert(i, process);
    }
    println!("运行完毕，分配表如下：");
    print_processes(wait_list.clone());
}
