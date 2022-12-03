use std::io;
use std::collections::HashSet;


fn score_for_item(c: char) -> u32 {

    if c.is_ascii_lowercase()
    {
        (c as u32) - ('a' as u32) + 1
    }
    else if c.is_ascii_uppercase() 
    {
        (c as u32) - ('A' as u32) + 27
    }
    else 
    {
        panic!()
    }
}

fn get_shared_item(s :String) -> char 
{
    let halves = s.split_at(s.len()/2);

    let s1 = halves.0.chars().collect::<HashSet<char>>();
    let s2 = halves.1.chars().collect::<HashSet<char>>();

    *s1.intersection(&s2).next().unwrap()
}

fn main() 
{
    let mut total = 0 as u32;

    for line in io::stdin().lines() 
    {
        let actual_line = line.unwrap();

        total += score_for_item(get_shared_item(actual_line));
    } 

    println!("{}", total);
}