use crossterm::style::Stylize;
use producer_consumer::producer_consumer::{run, ProcessType, PCB};
use producer_consumer::typedef::ProcessQueue;
use producer_consumer::util::print_queue;
use rand::Rng;
use std::env;
use std::io::stdin;

fn generate_random_process(num: u32) -> ProcessQueue {
    let mut list: ProcessQueue = ProcessQueue::new();
    // 因为有两个用来保证至少每种一个
    if num > 2 {
        for _ in 0..num - 2 {
            let mut process = PCB::new();
            process.set_process_type(if rand::thread_rng().gen_range(0..=1) == 0 {
                ProcessType::CONSUMER
            } else {
                ProcessType::PRODUCER
            });
            list.push_back(process);
        }
    }
    // keep at least one process each type
    list.push_back(*PCB::new().set_process_type(ProcessType::PRODUCER));
    list.push_back(*PCB::new().set_process_type(ProcessType::CONSUMER));
    return list;
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let num: u32 = if args.len() < 2 {
        println!("{}", "请输入需要运行的进程数量".red());
        let mut input: String = String::new();
        stdin().read_line(&mut input).expect("Failed");
        input = input.trim().to_string();
        input.parse::<u32>().unwrap()
    } else {
        args[1].parse::<u32>().unwrap()
    };
    let mut list = generate_random_process(num);

    println!("生成随机属性的进程");
    print_queue(list.to_owned());
    println!("开始运行\n");
    let result = run(&mut list);
    if result {
        println!("成功")
    } else {
        println!("失败")
    }
}
