use std::fs::File;
use std::io::{self, BufRead, BufReader};

fn result_line_to_f64(res: Result<String, io::Error>) -> f64 {
    res.unwrap().split_whitespace().collect::<Vec<&str>>()[1]
        .parse()
        .unwrap()
}

fn main() {
    let file = File::open("/proc/meminfo").unwrap();
    let reader = BufReader::new(file);

    let mut mem_total: f64 = 0.0;
    let mut mem_available: f64 = 0.0;
    let mut swap_total: f64 = 0.0;
    let mut swap_available: f64 = 0.0;

    let mut counter = 0;
    reader.lines().for_each(|line| {
        match counter {
            0 => mem_total = result_line_to_f64(line),
            2 => mem_available = result_line_to_f64(line),
            14 => swap_total = result_line_to_f64(line),
            15 => swap_available = result_line_to_f64(line),
            16 => return,
            _ => {}
        }
        counter += 1
    });

    println!(
        "{:.1}GiB {:.1}GiB",
        (mem_total - mem_available) / 1024.0 / 1024.0,
        (swap_total - swap_available) / 1024.0 / 1024.0
    )
}
