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


    for mut i in 0..240 as i32
    {
        if (i % 40) == 0
        {
            println!("");
        }

        let mut outchar = '.';

        let pos = values[1+i as usize];

        i = i % 40;

        if (pos-i).abs() <= 1 {
            outchar = '#';
        }

        print!("{}", outchar);

    }

    //for i in 0..41 {println!("{} : {}", i, values[i]); }
}