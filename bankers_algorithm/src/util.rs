use crossterm::style::Stylize;
use prettytable::{Cell, cell, format, Row, row, Table, table};

use crate::typedef::{Process, Resource, ResourceMap, Sequence};

fn get_resource_table(resource: &Resource, map: &ResourceMap) -> Table {
    let mut table = Table::new();
    table.set_format(*format::consts::FORMAT_NO_BORDER_LINE_SEPARATOR);
    let mut header_row = row![];
    let mut value_row = row![];
    for i in 0..resource.len() {
        let res = resource[i];
        let name = map.get(&i).unwrap();
        header_row.add_cell(Cell::new(name));
        value_row.add_cell(Cell::new(&*res.to_string()).style_spec("Fy"));
    }
    table.add_row(header_row);
    table.add_row(value_row);
    return table;
}

pub fn display_process(process: &Process, map: &ResourceMap) {
    println!("进程");
    let max = get_resource_table(&process.clone().max, &map.clone());
    let allocation = get_resource_table(&process.clone().allocation, &map.clone());
    let need = get_resource_table(&process.clone().need, &map.clone());
    let mut table = table!(
        ["最大需求\nmax", max],
        ["已分配\nallocation", allocation],
        ["需求\nneed", need]
    );
    table.set_format(*format::consts::FORMAT_NO_BORDER);
    table.set_titles(row![
        H2 -> (&*process.clone().id().to_string())
    ]);
    table.printstd();
}

pub fn display_processes_table(processes: &Vec<Process>, map: &ResourceMap) {
    let mut table = Table::new();
    let resource_type_count = map.values().len();
    let style_spec = format!("H{}", resource_type_count);

    table.set_titles(Row::new(vec![
        Cell::new(" "),
        Cell::new("最大").style_spec(&*style_spec),
        Cell::new("已分配").style_spec(&*style_spec),
        Cell::new("需求").style_spec(&*style_spec),
    ]));
    let mut resource_row = row![Cell::new("进程id")];
    for _ in 0..3 {
        for i in 0..resource_type_count {
            resource_row.add_cell(Cell::new(map.get(&i).unwrap()))
        }
    }
    table.add_row(resource_row);

    for i in 0..processes.len() {
        let mut row = row![];
        let process = &processes[i];
        row.add_cell(Cell::new(&i.to_string()));
        for r in [&process.max, &process.allocation, &process.need] {
            for num in r {
                row.add_cell(Cell::new(&*num.to_string()));
            }
        }
        table.add_row(row);
    }
    table.printstd();
}

pub fn display_resource_table(resource: &Resource, map: &ResourceMap) {
    let table = get_resource_table(resource, map);
    table.printstd();
}

pub fn display_sequence(sequence: &Sequence) {
    println!("{}", "进程执行顺序如下：".underlined());
    for i in 0..sequence.len() {
        let pid = &sequence[i];
        if i != 0 { print!(" {} ", "->".grey()) }
        print!("P{}", pid.to_string().blue());
    }
    print!("\n");
}
