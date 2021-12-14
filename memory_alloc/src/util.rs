use crossterm::style::Stylize;
use prettytable::{Attr, Cell, color, row, Row, Table};
use ulid::Ulid;

use crate::algorithm::typedef::{Memory, MemoryTable, Size};

pub fn show_memory_block(block: Memory, show_address: Option<bool>) {
    let id: Ulid = block.id();
    let id_text = id.to_string().green().underlined();
    let size_text = block.size.to_string().yellow();
    let mut text = format!("进程id: {}, 占用大小: {}", id_text, size_text);

    let address_text = format!(", 地址: {} ~ {}",
                               block.address.start.to_string().yellow().bold(),
                               block.address.end.to_string().yellow().bold()
    );
    if show_address.unwrap_or(false) { text += &*address_text; }
    println!("{}", text);
}

fn get_header_row<T: ToString>(header: Vec<T>) -> Row {
    let mut header_row = row![];
    for i in header {
        let text = i.to_string();
        header_row.add_cell(
            Cell::new(&*text)
                .with_style(Attr::Bold)
                .with_style(Attr::ForegroundColor(color::BRIGHT_WHITE)),
        )
    }
    return header_row;
}

fn get_cell<T: ToString>(contain: T, styles: Vec<Attr>) -> Cell {
    let text = contain.to_string();
    let mut cell = Cell::new(&*text);
    for style in styles {
        cell = cell.with_style(style);
    }
    return cell;
}

pub fn display_memory_table(memory_table: MemoryTable, total: Size) {
    let mut table = Table::new();

    let header = get_header_row(vec!["编号", "块状态", "起始地址", "结束地址", "块大小", "占用比例", "块Id"]);
    table.add_row(header);

    for i in 0..memory_table.len() {
        let memory = memory_table[i];
        let mut row = row![];
        let status = match memory.flag() {
            1 => "被占用",
            0 | _ => "空"
        };
        let status_color = if memory.flag() == 0 { color::BRIGHT_WHITE } else { color::BRIGHT_BLUE };

        let radio = format!("{:.2}%", (memory.size as f64 / total as f64) * 100_f64);

        row.add_cell(get_cell(i.to_string(), vec![Attr::ForegroundColor(color::MAGENTA)]));
        row.add_cell(get_cell(status, vec![Attr::ForegroundColor(status_color)]));
        row.add_cell(get_cell(memory.address.start, vec![Attr::ForegroundColor(color::BRIGHT_YELLOW)]));
        row.add_cell(get_cell(memory.address.end, vec![Attr::ForegroundColor(color::BRIGHT_YELLOW)]));
        row.add_cell(get_cell(memory.size, vec![Attr::ForegroundColor(color::YELLOW)]));
        row.add_cell(get_cell(radio, vec![Attr::ForegroundColor(color::BLUE)]));
        row.add_cell(get_cell(memory.id(), vec![Attr::ForegroundColor(color::MAGENTA)]));

        table.add_row(row);
    }
    table.printstd();
}
