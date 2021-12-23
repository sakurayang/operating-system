use crossterm::style::Stylize;
use prettytable::{Cell, cell, Row, row, Table};

use crate::typedef::{PCB, Sequence};

pub fn display_process(process: &PCB) {
    println!("进程 {}", process.id().to_string().cyan().underlined());
    print!("到达时间 {}，", process.arrive.to_string().yellow());
    print!("所需时间 {}\n", process.need().to_string().yellow());
}

fn display_sequence(sequence: &Sequence) {
    let (width, _) = crossterm::terminal::size().unwrap_or_default();
    let mut char_count = 0;
    for i in 0..sequence.len() {
        let index = sequence[i];
        let name = format!("{}{}", "P".cyan(), index.to_string().yellow());
        print!("{}", name);
        char_count += 2;
        if char_count > width {
            print!("\n");
            char_count = 0;
        }
        if i + 1 < sequence.len() {
            print!(" -> ");
            char_count += 4;
            if char_count > width {
                print!("\n");
                char_count = 0;
            }
        }
        print!("\n")
    }
}

fn get_table(processes: &Vec<PCB>) -> Table {
    let mut table = Table::new();
    let title = Vec::from([
        "进程编号", "到达时间", "需求时间",
        "开始时间", "结束时间", "等待时间",
        "周转时间", "带权周转时间"
    ].map(|x| Cell::new(x)));

    table.set_titles(Row::new(title));

    for i in 0..processes.len() {
        let pcb = processes[i];
        let row = row![
            FC -> i.to_string(), pcb.arrive.to_string(), pcb.need().to_string(),
            pcb.begin.to_string(), pcb.end.to_string(), pcb.wait_time().to_string(),
            pcb.turnaround().to_string(), pcb.weighted_turnaround().to_string()
        ];
        table.add_row(row);
    }

    return table;
}

pub fn display_result(processes: &Vec<PCB>, sequence: &Sequence) {
    get_table(&processes).printstd();
    display_sequence(&sequence);

    let process_count = processes.len();
    let mut turnaround_total: u32 = 0;
    let mut weighted_turnaround_total: f64 = 0_f64;
    for pcb in processes {
        turnaround_total += pcb.turnaround();
        weighted_turnaround_total += pcb.weighted_turnaround();
    }
    let avg_total = turnaround_total as f64 / process_count as f64;
    let avg_weighted = weighted_turnaround_total / process_count as f64;
    println!("平均周转时间：{}", avg_total.to_string().green().underlined());
    println!("平均带权周转时间：{}", avg_weighted.to_string().green().underlined());
}
