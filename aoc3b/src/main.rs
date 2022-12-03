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

fn main() 
{
    let mut total = 0 as u32;

    let v: Vec<String> = io::stdin().lines().map(|x| x.unwrap()).collect();

    for lines in v.chunks(3)
    {
        let s1 = lines[0].chars().collect::<HashSet<char>>();
        let s2 = lines[1].chars().collect::<HashSet<char>>();
        let s3 = lines[2].chars().collect::<HashSet<char>>();
        
        let i1: HashSet<&char> = s1.intersection(&s2).collect();
        let i2: HashSet<&char> = s3.intersection(&s2).collect();

        let mut r = i1.intersection(&i2);

        let value = r.next().unwrap();

        total += score_for_item(**value);
    } 

    println!("{}", total);
}