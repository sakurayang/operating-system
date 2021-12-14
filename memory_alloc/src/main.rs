use std::io::stdin;

use crossterm::style::Stylize;

use memory_alloc::algorithm::*;
use memory_alloc::algorithm::typedef::{Algorithm, Memory, MemoryTable, Size};
use memory_alloc::memory_alloc::{alloc, free};
use memory_alloc::util::{display_memory_table, show_memory_block};

const SIZE: u32 = 0xffffffff;

fn get_input() -> String {
    let mut input = String::new();
    stdin().read_line(&mut input).expect("Failed");
    input = input.trim().to_string();
    return input;
}

fn handle_choose() -> crossterm::Result<Algorithm> {
    println!("{}", String::from("请选择算法").red().bold());

    let map = [
        ["首次适应分配算法", "First Fit"],
        ["循环适应分配算法", "Next Fit"],
        ["最佳适应分配算法", "Best Fit"],
    ];
    for i in 0..map.len() {
        let order = i.to_string().bold().white();
        let name = map[i][0];
        let translate = map[i][1].italic().grey();
        print!("{}{}{}", "[".yellow(), order, "]".yellow());
        print!("{} ({})\n", name, translate);
    }

    let input = get_input();
    let num = input.parse::<usize>().unwrap_or_default();

    match num {
        1 => Ok(Algorithm::NextFit),
        2 => Ok(Algorithm::BestFit),
        0 | _ => Ok(Algorithm::FirstFit),
    }
}

fn handle_process_input() -> Result<Memory, u8> {
    println!(
        "请输入{}，输入 r 释放内存",
        String::from("进程大小").blue().bold()
    );

    let input = get_input();
    // 因为没有丢表进来，又不想丢，干脆返回一个特殊的数字，让后面来判断
    if input == "r" {
        return Err(8);
    }

    let size = input.parse::<Size>().unwrap_or(0);
    if size > SIZE {
        return Err(1);
    }
    if size <= 0 {
        return Err(2);
    }
    let mut process = Memory::new(1);
    process.set_size(size);
    return Ok(process);
}

fn handle_free_choose() -> Result<(), ()> {
    println!("是否需要释放内存? [Y]/N");
    let input = get_input();
    return if input.is_empty() || input.to_lowercase() == "y" {
        Ok(())
    } else {
        Err(())
    };
}

fn handle_free(memory: &mut MemoryTable) {
    return loop {
        display_memory_table(memory.clone(), SIZE);
        println!(
            "请选择{}，输入 q 退出",
            String::from("需要释放进程的编号").blue().bold()
        );

        let input = get_input();
        if input == "q" {
            break;
        }
        let index = input.parse::<usize>().unwrap_or_default();

        let res = free(memory, index);
        if res.is_err() {
            println!("{}", "错误编号，请重新输入".red());
            continue;
        }
        println!("{}", "成功释放".green());
    };
}

fn main() {
    let mut table: MemoryTable = MemoryTable::new();
    // init
    table.push_back(Memory::new(0).set_size(SIZE));

    let calc_fn = match handle_choose().unwrap_or(Algorithm::FirstFit) {
        Algorithm::FirstFit => ff::calc_alloc_pos,
        Algorithm::NextFit => nf::calc_alloc_pos,
        Algorithm::BestFit => bf::calc_alloc_pos,
    };

    let mut _start_index: usize = 0;

    loop {
        let process_res = handle_process_input();
        if process_res.is_err() {
            match process_res.unwrap_err() {
                0 => {
                    eprintln!("{}", "失败，空间不足".red().bold());
                    if handle_free_choose().is_ok() {
                        handle_free(&mut table);
                        continue;
                    } else {
                        break;
                    }
                }
                1 => eprintln!("{}", "进程占用大于总内存".red().bold()),
                2 => eprintln!("{}", "进程大小错误".red().bold()),
                8 => {
                    handle_free(&mut table);
                    continue;
                }
                _ => continue,
            }
            continue;
        }

        let mut process = process_res.unwrap();

        let result = calc_fn(table.to_owned(), process);

        if result.is_err() {
            eprintln!("{}", "失败，空间不足".red().bold());
            show_memory_block(process, Some(false));
            if handle_free_choose().is_ok() {
                handle_free(&mut table);
                continue;
            } else {
                break;
            }
        }

        let (index, address) = result.unwrap();
        println!("{}", "成功".green());

        show_memory_block(process, Some(true));

        _start_index = alloc(&mut table, &mut process, address, index);

        display_memory_table(table.clone(), SIZE)
    }
}
