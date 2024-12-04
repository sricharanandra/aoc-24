use std::fs::File;
use std::io::Read;

fn is_report_safe(reports: &Vec<i32>) -> bool {
    let increasing = reports[1] > reports[0];
    for i in 0..reports.len() - 1 {
        let diff = reports[i + 1] - reports[i];

        if diff.abs() < 1 || diff.abs() > 3 {
            return false;
        }
        if increasing && diff < 0 || !increasing && diff > 0 {
            return false;
        }
    }
    true
}

fn main() {
    let mut data_file = File::open("../d2.txt").unwrap();
    let mut file_data = String::new();
    data_file.read_to_string(&mut file_data).unwrap();

    let mut safe_count = 0;

    for line in file_data.trim().lines() {
        let reports: Vec<i32> = line
            .split_whitespace()
            .map(|x| x.parse().unwrap())
            .collect();

        if is_report_safe(&reports) {
            safe_count += 1;
        }
    }
    println!("{}", safe_count);
}
