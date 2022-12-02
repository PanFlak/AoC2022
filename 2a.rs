use std::io;

const SCORE_WIN :u32 = 6;
const SCORE_LOSE :u32 = 0;
const SCORE_DRAW :u32 = 3;

fn win_score(a: char, b: char) -> u32 {
    match (a, b) {
        ('A', 'X') => SCORE_DRAW,
        ('A', 'Y') => SCORE_WIN,
        ('A', 'Z') => SCORE_LOSE,

        ('B', 'X') => SCORE_LOSE,
        ('B', 'Y') => SCORE_DRAW,
        ('B', 'Z') => SCORE_WIN,

        ('C', 'X') => SCORE_WIN,
        ('C', 'Y') => SCORE_LOSE,
        ('C', 'Z') => SCORE_DRAW,
        (_, _) => panic!()
    }
}

fn shape_score(a: char) -> u32 {
    match a {
        'X' => 1,
        'Y' => 2,
        'Z' => 3,
        _ => panic!()
    }
}

fn round_score(a_str: &str, b_str: &str) -> u32 {

    match (a_str.chars().nth(0), b_str.chars().nth(0)) 
    {
        (Some(a_char), Some(b_char)) => 
        {
            let winscore = win_score(a_char, b_char);
            let shapescore = shape_score(b_char);

            winscore + shapescore
        }
        _ => panic!()
    }
}

fn main() {

    let mut total = 0;

    for line in io::stdin().lines() 
    {
        let actual_line = line.unwrap();

        let mut splits = actual_line.split_whitespace();

        total += round_score(splits.next().unwrap(),splits.next().unwrap());

    } 

    println!("{}", total);    // not 11717
}
