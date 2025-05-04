use core::time;
use std::{
    fs::File,
    io::{BufRead, BufReader},
    path,
    thread::sleep,
};

const PROC_STAT: &str = "/proc/stat";

struct CpuStats {
    work: u64,
    total: u64,
}

fn calculate_cpu_usage(start: CpuStats, end: CpuStats) -> f64 {
    100f64 * (end.work - start.work) as f64 / (end.total - start.total) as f64
}

fn parse_cpu_stats(raw_stats: String) -> Result<CpuStats, String> {
    let mut stats = CpuStats { work: 0, total: 0 };
    let words: Vec<&str> = raw_stats.split_whitespace().collect();

    if words.len() != 11 {
        return Err("Unexpected format of given stat string".to_string());
    }

    for i in 2..words.len() {
        let counter: u64 = words[i]
            .parse::<u64>()
            .map_err(|e| format! {"Can't decode stats line due to conversion error: {}", e})?;

        stats.total += counter;
        if i < 4 {
            stats.work += counter;
        }
    }

    Ok(stats)
}

fn get_state_str(path: &path::Path) -> Result<String, String> {
    let stat_file = File::open(PROC_STAT)
        .map_err(|e| format!("Can't open stat file {:?} due to error: {:?}", path, e))?;

    let reader = BufReader::new(stat_file);

    reader
        .lines()
        .next()
        .ok_or(format!("file {:?} is empty", PROC_STAT))?
        .map_err(|e| format!("Can't read from {:?} file due to error: {:?}", PROC_STAT, e))
}

fn main() -> Result<(), String> {
    let path = path::Path::new(PROC_STAT);
    let start_line = get_state_str(path)?;
    let start_stats = parse_cpu_stats(start_line)?;

    sleep(time::Duration::from_secs(1));

    let end_line = get_state_str(path)?;
    let end_stats = parse_cpu_stats(end_line)?;

    println!("{:.1}%", calculate_cpu_usage(start_stats, end_stats));

    Ok(())
}
