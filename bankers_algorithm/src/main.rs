use std::collections::HashMap;
use std::io::stdin;

use crossterm::style::Stylize;

use bankers_algorithm::bankers_algorithm::alloc_calc;
use bankers_algorithm::typedef::{Process, Resource, ResourceMap};
use bankers_algorithm::util::{display_process, display_processes_table, display_resource_table, display_sequence};

fn get_input() -> String {
    let mut input = String::new();
    stdin().read_line(&mut input).expect("Failed");
    input = input.trim().to_string();
    return input;
}

fn handle_resource_input() -> (Resource, ResourceMap) {
    println!("请输入{}与{}，{}分割，输入 {} 结束",
             "资源名称".blue().underlined(),
             "数量".blue().underlined(),
             "空格".yellow().underlined(),
             "o".red().underlined()
    );
    let mut resource: Resource = Resource::new();
    let mut resource_name_map: ResourceMap = HashMap::new();

    let mut index: usize = 0;
    loop {
        let input = get_input();
        if input.to_lowercase() == "o" { break; }
        let split: Vec<&str> = input.split_whitespace().collect();
        let name = String::from(split[0]);
        let num: i32 = split[1].parse::<i32>().unwrap_or_default();
        resource.push(num);
        resource_name_map.insert(index, name);
        index += 1;
    }
    return (resource.clone(), resource_name_map.clone())
}

fn handle_process_resource_input(len: usize) -> Option<Resource> {
    let mut resource: Resource = Resource::new();
    let input = get_input();
    if input.to_lowercase() == "o" { return None; }
    let split: Vec<&str> = input.split(" ").collect();
    for i in 0..len { resource.push(split[i].parse::<i32>().unwrap_or_default()); }
    return Some(resource);
}

fn handle_process_input(len: usize, map: &ResourceMap) -> Vec<Process> {
    let mut processes: Vec<Process> = Vec::new();
    let print = |str: &str| {
        let string = String::from(str);
        println!("请输入进程{}, {}分割，输入 {} 结束",
                 string.blue().underlined(),
                 "空格".yellow().underlined(),
                 "o".red().underlined()
        );
    };
    loop {
        print("最大需求");
        let max = handle_process_resource_input(len);
        if max.is_none() { break; }
        let _max = max.unwrap();

        print("已分配资源");
        let allocation = handle_process_resource_input(len);
        if allocation.is_none() { break; }
        let _allocation = allocation.unwrap();

        let mut need: Resource = vec![0; len];
        for i in 0..len {
            need[i] = _max[i] - _allocation[i];
        }

        let mut process = Process::new();
        process.allocation = _allocation;
        process.max = _max;
        process.need = need;
        processes.push(process.clone());
        display_process(&process, &map.clone());
    }
    return processes;
}

fn main() {
    let (available, available_map) = handle_resource_input();
    display_resource_table(&available, &available_map.clone());
    let processes = handle_process_input(available.len(), &available_map);
    display_processes_table(&processes, &available_map.clone());
    let sequence = alloc_calc(available.clone(), processes.clone());
    if sequence.is_err() {
        eprintln!("{}", "无法找到安全序列".red());
    } else {
        display_sequence(&sequence.unwrap());
    }
}
