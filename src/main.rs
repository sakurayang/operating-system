extern crate operating_system;

use std::io::{stdin};
use crossterm::{
    style::{Color, Attribute, Stylize}
};
use prettytable::{Attr, Cell, color, row, Table};
use operating_system::algorithmic::hrrn::{Job, run};

const ALGORITHMIC: [&str; 1] = ["高响应比优先 (HRRN)"];

fn algorithmic_chooser() -> String {
    let mut input = String::new();

    println!("{}", "请选择算法，按 q 退出".with(Color::Yellow).attribute(Attribute::Bold));
    for i in 0..ALGORITHMIC.len() {
        println!("[{}] {}", i.to_string().green(), ALGORITHMIC[i]);
    }

    stdin().read_line(&mut input).expect("Failed");
    return input.replace("\n", "");
}

fn job_input() -> Vec<Job> {
    let mut jobs :Vec<Job> = Vec::new();
    let text :String = ["作业到达时间", "作业需求时间"].join("、");
    println!("请按顺序输入{}，空格分隔，按 o 确定，按 c 取消", text.with(Color::Blue).attribute(Attribute::Bold));
    loop {
        let mut input= String::new();
        stdin().read_line(&mut input).expect("Failed");
        input = input.replace("\n", "");
        if input == "o" { return jobs; }
        else if input == "c" { return Vec::new(); }
        let params :Vec<&str> = input.split(' ').collect();
        let arrive_time :f64 = params[0].parse::<f64>().unwrap_or(0.0).to_owned();
        let need_time :f64 = params[1].parse::<f64>().unwrap_or(0.0).to_owned();
        jobs.push(Job::new(arrive_time, need_time));
    }
}

fn main() {
    let algorithmic = algorithmic_chooser();

    let mut jobs :Vec<Job> = Vec::new();

    'start: loop {
        match algorithmic.as_str() {
            // hrrn
            "0" => {
                jobs = job_input();
                break;
            }
            "q" => { return; }
            _ => break 'start
        }
    }

    // println!("{:?}", jobs);
    let run_job = run(jobs);
    // println!("{}", text);

    let mut table = Table::new();

    let header = vec!["到达时间","需求时间","开始时间","周转时间","带权周转时间","作业ID"];
    let mut header_row = row![];

    for i in header {
        header_row.add_cell(Cell::new(i)
            .with_style(Attr::Bold)
            .with_style(Attr::ForegroundColor(color::BRIGHT_WHITE))
        )
    }
    table.add_row(header_row);

    let jobs_count = run_job.len() as f64;
    let mut total_turnaround_time :f64 = 0.0;
    let mut total_weighted_turnaround :f64 = 0.0;
    for job in run_job {
        let values = [job.time.arrive, job.resource.time, job.time.begin, job.time.turnaround, job.time.weighted_turnaround];
        total_turnaround_time += job.time.turnaround;
        total_weighted_turnaround += job.time.weighted_turnaround;
        let mut value_row = row![];
        for value in values {
            value_row.add_cell(Cell::new(&*value.to_string())
                .with_style(Attr::ForegroundColor(color::WHITE))
            )
        }
        value_row.add_cell(Cell::new(&*job.id.to_string()).with_style(Attr::ForegroundColor(color::WHITE)));
        table.add_row(value_row);
    }

    let text = "运行结果：".bold().cyan();
    println!("{}", text);
    table.printstd();

    let avg_turnaround = total_turnaround_time / jobs_count;
    let avg_weighted_turnaround = total_weighted_turnaround / jobs_count;
    let count_text = format!("{}{}", "作业总数：".green(), jobs_count).underlined();
    let avg_turnaround_text = format!("{}{:.3}", "平均周转时间为：".green(), avg_turnaround).underlined();
    let avg_weighted_text = format!("{}{:.3}", "平均带权周转时间为：".green(), avg_weighted_turnaround).underlined();
    println!("{}\n{}\n{}", count_text, avg_turnaround_text, avg_weighted_text);
}
