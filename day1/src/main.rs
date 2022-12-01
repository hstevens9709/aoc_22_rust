use std::env;
use std::fs;
use std::str::Split;

fn main() {
    let file_path = "src/input.txt";
    println!("In file {}", file_path);

    let contents = fs::read_to_string(file_path)
        .expect("Should have been able to read the file");

    let list = contents.split("\n\n");
    let mut prev_max = 0; 
    for inner_list in list {
        let mut sum = 0;
        for item in inner_list.split("\n") {
            sum += item.parse::<i32>().unwrap();
        }
        if sum >= prev_max {
            prev_max = sum;
        }
    }
    println!("{}", prev_max)
}
