// use futures::future::join_all;
// use std::{thread::available_parallelism, time::SystemTime};
//
// #[tokio::main(flavor = "current_thread")]
// async fn main() -> Result<(), Box<dyn std::error::Error>> {
//     let now = SystemTime::now();
//
//     let cores = available_parallelism().unwrap().get();
//     let mut futes = Vec::with_capacity(cores);
//
//     for i in 0..cores {
//         let path = format!("/sys/devices/system/cpu/cpu{}/cpufreq/scaling_cur_freq", i);
//         futes.push(tokio::fs::read_to_string(path));
//     }
//     println!("{:?}", now.elapsed().unwrap());
//     let aha = join_all(futes).await;
//     println!("{:?}", now.elapsed().unwrap());
//
//     let max_freq: u64 = aha
//         .into_iter()
//         .map(|r| r.unwrap())
//         .map(|s| s.leak().trim())
//         .map(|s| s.parse().unwrap())
//         .max()
//         .unwrap();
//
//     println!("{}MHz", max_freq / 1000);
//
//     println!("{:?}", now.elapsed().unwrap());
//
//     Ok(())
// }

use std::{fs, thread::available_parallelism};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cores = available_parallelism().unwrap().get();
    let mut futes = Vec::with_capacity(cores);

    for i in 0..cores {
        let path = format!("/sys/devices/system/cpu/cpu{}/cpufreq/scaling_cur_freq", i);
        futes.push(fs::read_to_string(path));
    }

    let max_freq: u64 = futes
        .into_iter()
        .map(|r| r.unwrap())
        .map(|s| s.leak().trim())
        .map(|s| s.parse().unwrap())
        .max()
        .unwrap();

    println!("{}MHz", max_freq / 1000);

    Ok(())
}
