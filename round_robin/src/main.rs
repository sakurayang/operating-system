use std::io::stdin;

use crossterm::style::Stylize;

use round_robin::round_robin::run;
use round_robin::typedef::PCB;
use round_robin::util::{display_process, display_result};

fn get_input() -> String {
    let mut input = String::new();
    stdin().read_line(&mut input).expect("Failed");
    input = input.trim().to_string();
    return input;
}

fn processes_input() -> Vec<PCB> {
    let mut processes: Vec<PCB> = Vec::new();
    let text = ["作业到达时间", "作业需求时间"].join("、").blue().bold();

    loop {
        println!("请按顺序输入{}，空格分隔，按 o 结束", text);
        let input = get_input();
        if input == "o" {
            return processes;
        }
        let params: Vec<&str> = input.split(' ').collect();
        let arrive_time: u32 = params[0].parse::<u32>().unwrap_or(0).to_owned();
        let need_time: u32 = params[1].parse::<u32>().unwrap_or(0).to_owned();
        let mut process = PCB::new(need_time);
        process.arrive = arrive_time;
        display_process(&process.clone());
        processes.push(process);
    }
}

fn main() {
    let mut processes = processes_input();
    let sequence = run(&mut processes);
    display_result(&processes, &sequence);
}
