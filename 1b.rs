use std::io;

fn main() {

    let mut sum = 0;
    let mut vec = Vec::new();

    for line in io::stdin().lines() 
    {
        
        match line.expect("io ok").parse::<u64>() {
            Ok(n) => sum += n,
            Err(_) => {
                vec.push(sum);  
                sum = 0;
            }
        }
    }

    vec.sort();
    vec.reverse();

    println!("{}", vec[0]+vec[1]+vec[2]);    
}
