use std::{fs, str::Lines};
use itertools::{Chunk, Itertools};

fn read_file() -> String {
    let file_path = "src/input.txt";
    println!("In file {}", file_path);

    let contents = fs::read_to_string(file_path)
        .expect("Should have been able to read the file");

    return contents;
}

fn find_common_char(str1: &str, str2: &str) -> char{
    // println!("{}, {}", str1, str2);
    for c in str1.chars() {
        if str2.contains(c) { return c; }
    }
    return ' ';
}

fn find_common_char_from_3_strings(str1: &str, str2: &str, str3: &str) -> char {
    for c in str1.chars() {
        if str2.contains(c) && str3.contains(c) { return c; }
    }
    return ' ';
}

fn convert_char_to_num(c: char) -> u32 {
    if c.is_uppercase() {
        return (c as u32) - ('A' as u32) + 27;
    } else {
        return (c as u32) - ('a' as u32) + 1;
    }
}

fn get_badge_priority(group: Chunk<Lines>) -> u32 {
    let group_vec = group.collect::<Vec<&str>>();
    let badge = find_common_char_from_3_strings(group_vec[0], group_vec[1], group_vec[2]);
    return convert_char_to_num(badge);
}

fn part1() {
    let input = read_file();
    let rucksacks = input.split("\n").collect::<Vec<&str>>();
    let mut sum = 0;
    for rucksack in rucksacks {
        // split in half
        let length = rucksack.len();
        let head = &rucksack[0..length/2];
        let tail = &rucksack[(length/2)..length];
        // run character search algorithm over them
        let common_char = find_common_char(head, tail);
        // convert to number and add to sum
        sum += convert_char_to_num(common_char);
    }
    println!("{}", sum);
}

fn part2() {
    let input: String = read_file();
    let result = input.lines().chunks(3).into_iter().map(get_badge_priority).sum::<u32>();
    println!("{}", result);
}

fn main() {
    part1();
    part2();
}
