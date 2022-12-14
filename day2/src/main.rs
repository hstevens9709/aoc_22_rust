use std::fs;

enum Choice {
    Rock,
    Paper,
    Scissors
}

enum Result {
    Lose,
    Draw,
    Win,
}

struct Hand {
    choice: Choice
}

struct Round {
    result: Result
}

fn makeHand(input_string: &str) -> Hand {
    if input_string == "A" || input_string == "X" {
        return Hand{choice: Choice::Rock};
    }
    else if input_string == "B" || input_string == "Y"  {
        return Hand{choice: Choice::Paper};
    }
    else if input_string == "C" || input_string == "Z"  {
        return Hand{choice: Choice::Scissors};
    }
    else {
        panic!("Uh oh!");
    }
}

fn makeRound(input_string: &str) -> Round {
    if input_string == "X" {
        return Round{result: Result::Lose};
    }
    else if input_string == "Y" {
        return Round{result: Result::Draw};
    }
    else if input_string == "Z" {
        return Round{result: Result::Win};
    }
    else {
        panic!("Uh oh!");
    }
}

fn getScore(hand1: Hand, hand2: Hand) -> i32 {
    match hand1.choice {
        Choice::Rock => match hand2.choice {
            Choice::Rock => 4,
            Choice::Paper => 8,
            Choice::Scissors => 3,
        },
        Choice::Paper => match hand2.choice {
            Choice::Rock => 1,
            Choice::Paper => 5,
            Choice::Scissors => 9,
        },
        Choice::Scissors => match hand2.choice {
            Choice::Rock => 7,
            Choice::Paper => 2,
            Choice::Scissors => 6,
        },
    }
}

fn getScorePart2(hand: Hand, round: Round) -> i32 {
    match hand.choice {
        Choice::Rock => match round.result {
            Result::Lose => getScore(hand, Hand{choice: Choice::Scissors}),
            Result::Draw => getScore(hand, Hand{choice: Choice::Rock}),
            Result::Win => getScore(hand, Hand{choice: Choice::Paper}),
        },
        Choice::Paper => match round.result {
            Result::Lose => getScore(hand, Hand{choice: Choice::Rock}),
            Result::Draw => getScore(hand, Hand{choice: Choice::Paper}),
            Result::Win => getScore(hand, Hand{choice: Choice::Scissors}),
        },
        Choice::Scissors => match round.result {
            Result::Lose => getScore(hand, Hand{choice: Choice::Paper}),
            Result::Draw => getScore(hand, Hand{choice: Choice::Scissors}),
            Result::Win => getScore(hand, Hand{choice: Choice::Rock}),
        },
    }
}

fn part1() {
    let file_path = "src/input.txt";
    println!("In file {}", file_path);

    let contents = fs::read_to_string(file_path)
        .expect("Should have been able to read the file");
    let list = contents.split("\n");
    let mut sum = 0;
    for item in list {
        let strings = item.split(" ").collect::<Vec<&str>>();
        let hand1 = makeHand(strings[0]);
        let hand2 = makeHand(strings[1]);
        sum += getScore(hand1, hand2);
    }
    println!("{}", sum)
}

fn part2() {
    // X = lose
    // Y = tie
    // Z = win
    let file_path = "src/input.txt";
    println!("In file {}", file_path);

    let contents = fs::read_to_string(file_path)
        .expect("Should have been able to read the file");
    let list = contents.split("\n");
    let mut sum = 0;
    for item in list {
        let strings = item.split(" ").collect::<Vec<&str>>();
        let hand1 = makeHand(strings[0]);
        let hand2 = makeRound(strings[1]);
        sum += getScorePart2(hand1, hand2);
    }
    println!("{}", sum)
}

fn main() {
    // A = X = Rock = 1 point
    // B = Y = Paper = 2 points
    // C = Z = Scissors = 3 points
    // Win = 6 points
    // Tie = 3 points
    // Lose = 0 points

    // e.g line C Z = Scissors vs Scissors = Tie = 3 points + 3 points = 6 points
    // e.g line A Y = Rock vs Paper = Win = 6 points + 2 points = 8 points

    // Feels like maybe the easiest way would be a switch case, e.g.
    // case A:
    //   case X: 4
    //   case Y: 8
    //   case Z: 3
    // case B:
    //    case X: 1
    // etc.

    part1();
    part2();
}


