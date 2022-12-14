use std::io;


fn main() 
{
    let mut values = Vec::new();
    let mut reg = 1;

    // values.push(reg);
    // values.push(reg);
    // values.push(reg);
    values.push(reg);




    for line_str in io::stdin().lines().map(|x| x.unwrap())
    {
        let mut line = line_str.split(" ");

        values.push(reg);

        match line.next().unwrap()
        {
            "noop" => {
            }
            "addx" => {
                values.push(reg);
                reg += line.next().unwrap().parse::<i32>().unwrap();
            }
            _ => panic!()
        }
    } 

    let poi_vec: Vec<i32> = vec![20, 60, 100, 140, 180, 220];

    let mut sum = 0;
    for i in poi_vec {
        let val = i * values[i as usize];
        println!("val at index {} is {}", i, val);
        sum += val;
    }

    println!("{}", sum);

    //for i in 218..222 {println!("{} : {}", i, values[i]); }
}