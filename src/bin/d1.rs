use std::fs::File;
use std::io::Read;

fn main() {
    let mut data_file = File::open("d1p1.txt").unwrap();
    let mut file_data = String::new();
    data_file.read_to_string(&mut file_data).unwrap();

    let mut left_list = Vec::new();
    let mut right_list = Vec::new();

    let lines = file_data.trim().split("\n");
    for line in lines {
        let numbers: Vec<i32> = line
            .split_whitespace()
            .map(|x| x.parse().unwrap())
            .collect();

        if numbers.len() == 2 {
            left_list.push(numbers[0]);
            right_list.push(numbers[1]);
        }
    }

    left_list.sort_unstable();
    right_list.sort_unstable();

    //println!("left list: {:?}", left_list);
    //println!("right list: {:?}", right_list);
    let mut total_distance = 0;

    for i in (0..left_list.len()) {
        let difference = (right_list[i] - left_list[i]).abs();
        total_distance += difference;
    }
    let mut similarity_score = 0;

    for &left_num in &left_list {
        let mut count = 0;
        for &right_num in &right_list {
            if left_num == right_num {
                count += 1;
            }
        }

        let present_score = left_num * count;
        similarity_score += present_score;
    }

    println!("{}", total_distance);
    println!("{}", similarity_score);
}
