use std::io;

fn main() {

    let mut sum = 0;
    let mut max = 0;

    for line in io::stdin().lines() 
    {
        
        match line.expect("io ok").parse::<u64>() {
            Ok(n) => sum += n,
            Err(_) => {
                if sum > max {
                    max = sum;
                }
                    
                sum = 0;
            }
        }
    }

    println!("{}", max);    
}
