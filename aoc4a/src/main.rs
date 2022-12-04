use std::io;

fn line_to_ranges(line: &str) -> (&str, &str)
{
    line.split_once(",").unwrap()
}

fn range_to_numpair(r: &str) -> (u32, u32)
{
    let (l, r) = r.split_once("-").unwrap();
    
    (l.parse::<u32>().unwrap(), r.parse::<u32>().unwrap())
}

fn containment(a_str: &str, b_str: &str) -> bool
{
    let (a1, a2) = range_to_numpair(a_str);
    let (b1, b2) = range_to_numpair(b_str);

    (a1 >= b1 && a2 <= b2)
    ||
    (a1 <= b1 && a2 >= b2)
}

fn main() 
{
    let mut total: u32 = 0;

    for line in io::stdin().lines().map(|x| x.unwrap())
    {
        let (range_a, range_b) = line_to_ranges(&line);

        total += containment(range_a, range_b) as u32;
    } 

    println!("{}", total);
}