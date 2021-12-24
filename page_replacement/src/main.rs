use std::{env, fs};
use std::io::stdin;

use crossterm::style::Stylize;

use page_replacement::algorithm::*;
use page_replacement::typedef::{Memory, Page, Queue, ResultQueue, RunResult};

fn get_input() -> String {
    let mut input = String::new();
    stdin().read_line(&mut input).expect("Failed");
    input = input.trim().to_string();
    return input;
}

fn handle_choose_input() -> fn(&mut Memory, &mut Page) -> RunResult {
    let choice = [
        format!("{} ({} / {})", "先进先出".underlined(), "FIFO", "First In First Out".italic()).cyan(),
        format!("{} ({} / {})", "最少使用".underlined(), "LRU", "Least Recently Used".italic()).cyan()
    ];
    for i in 0..choice.len() {
        println!("[{}] {}", i.to_string().yellow().bold(), choice[i]);
    }
    let choose = get_input();
    let num: usize = choose.parse().unwrap_or_default();
    if !(0..choice.len()).contains(&num) {
        panic!("错误选项");
    }
    return match num {
        1 => lru::put_in,
        0 | _ => fifo::put_in
    };
}

fn handle_file_input() -> String {
    println!("{}", "请将文件拖拽至此处：".red());
    let input = get_input();
    return input;
}

fn run(queue: Queue, memory: &mut Memory, put_in: fn(&mut Memory, &mut Page) -> RunResult) -> (ResultQueue, u32) {
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

fn main() {
    const SIZE: usize = 3;
    let args: Vec<String> = env::args().collect();
    let file = if &args.len() < &2_usize { handle_file_input() } else { args[1].clone() };
    let raw_seq = fs::read_to_string(file).unwrap_or("CAN_NOT_READ".parse().unwrap());
    let rows: Vec<&str> = raw_seq.split("\n").collect();
    let seqs: Vec<&str> = rows[0].split(" ").collect();
    let mut queue: Queue = Queue::new();
    seqs.iter().for_each(|x| {
        let content = x.parse::<u32>().unwrap_or_default();
        queue.push(Page::new(content));
    });
    let put_in = handle_choose_input();
    let mut memory = Memory::with_capacity(SIZE);
    let (drop, count) = run(queue, &mut memory, put_in);
    println!("{}", "每次淘汰的页面号");
    drop.iter().for_each(|x| {
        if x.is_err() {
            if x.unwrap_err().is_some() {
                let (index, contain) = x.unwrap_err().unwrap();
                print!("{}:{} - ", index.to_string().yellow(), contain.to_string().dark_yellow());
            }
        }
    });
    print!("\n");
    println!("缺页总次数：{}", count.to_string().yellow().underlined());
}