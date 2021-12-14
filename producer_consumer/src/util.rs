use crossterm::style::Stylize;
use prettytable::{Attr, Cell, color, row, Row, Table};

use crate::producer_consumer::ProcessType;
use crate::typedef::{ProcessQueue, Product};

pub fn print_info(
    id: String,
    p_type: ProcessType,
    product: Product,
    producer_wait_len: usize,
    consumer_wait_len: usize,
    buffer_len: usize,
    has_run: bool,
) {
    let id_text = id.to_string().underlined().dark_green();
    let p_type_text = match p_type {
        ProcessType::PRODUCER => "生产者",
        ProcessType::CONSUMER => "消费者",
    }
    .bold()
    .blue();
    let product_text = format!("{:?}", product).yellow();
    let process_text = format!("运行进程[{}]{}：产物{}", p_type_text, id_text, product_text);
    let run_stat_text = if !has_run {
        "进程阻塞，转入等待队列".red()
    } else {
        "进程已运行".green()
    };
    let wait_text = format!(
        "消费者等待队列长度: {}, 生产者等待队列长度: {}, 缓冲区使用: {}",
        producer_wait_len.to_string().underlined().yellow(),
        consumer_wait_len.to_string().underlined().yellow(),
        buffer_len.to_string().underlined().yellow()
    );
    println!("{}\n{}\n{}\n", process_text, run_stat_text, wait_text);
}

pub fn print_queue(queue: ProcessQueue) {
    let mut table = Table::new();
    let header = vec!["类型", "产物", "ID"];
    let mut header_row = row![];
    for x in header {
        header_row.add_cell(
            Cell::new(x)
                .with_style(Attr::Bold)
                .with_style(Attr::ForegroundColor(color::BRIGHT_WHITE)),
        )
    }
    table.add_row(header_row);

    for x in &queue {
        let p_type_text = Cell::new(match x.process_type {
            ProcessType::PRODUCER => "生产者",
            ProcessType::CONSUMER => "消费者",
        })
        .with_style(Attr::ForegroundColor(color::BLUE));
        let product_text = Cell::new(&*format!("{:?}", x.product))
            .with_style(Attr::ForegroundColor(color::YELLOW));
        let id_text = Cell::new(&*x.id.to_string()).with_style(Attr::ForegroundColor(color::GREEN));
        table.add_row(Row::from(vec![p_type_text, product_text, id_text]));
    }

    let process_count = queue.len() as i32;
    table.printstd();
    println!(
        "总计：{}\n",
        format!("{}", process_count).underlined().yellow()
    );
}
