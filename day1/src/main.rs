use std::env;
use std::fs;
use std::str::Split;

fn main() {
    part1();
    part2();
}

fn part1() {
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

fn part2() {
    // I could use the same approach as part1 and just keep track of the 3 highest but that seems inelegant. 
    // Better to sort the list of sums and take the top 3
    let file_path = "src/input.txt";
    println!("In file {}", file_path);

    let contents = fs::read_to_string(file_path)
        .expect("Should have been able to read the file");

    let list = contents.split("\n\n");
    let mut sums = Vec::<i32>::new();
    for inner_list in list {
        let mut sum = 0;
        for item in inner_list.split("\n") {
            sum += item.parse::<i32>().unwrap();
        }
        sums.push(sum);
    }
    sums.sort();
    println!("{}", sums[sums.len()-1] + sums[sums.len()-2] + sums[sums.len()-3]);
}