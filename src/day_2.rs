#[path =  "utility.rs"] 
pub mod utility;

fn part_1(input : &String)->i32
{
let result = 0;

let mut z = 0;
let mut y = 0;
for lines in input.lines()
{
    let mut iter = lines.split_whitespace();
    let word = iter.next();

    let mut direction = "";
    if word != Some("bags")
    {
        match word
        {
            Some(value) =>
            { 
                direction= value;
            },
            None => println!("yoink"),
        }
    }
    let mut tru_dist = 0;
    let  distance  = iter.next();
    match distance
    {
        Some(value)=>
        {
            //Is value a
                match  value.parse::<i32>()
            {
                Ok(n)=> 
                {
                    tru_dist = n;
                },
                Err(_) => print!(""),
            };
        }
        None => print!(""),
    }
    println!("word: {}",direction);
    if direction == String::from("forward")
    {
        z += tru_dist;
    }
    if direction == String::from("up")
    {
        y -= tru_dist;
    }
    if direction == String::from("down")
    {
        y+= tru_dist;
    }

}
println!("{}",z);
return z*y;
}


fn part_2(input : &String)->i32
{
    let mut result = 0;

    let mut z = 0;
    let mut y = 0;
    let mut x =0;
    for lines in input.lines()
    {
        let mut iter = lines.split_whitespace();
        let word = iter.next();
    
        let mut direction = "";
        if word != Some("bags")
        {
            match word
            {
                Some(value) =>
                { 
                    direction= value;
                },
                None => println!("yoink"),
            }
        }
        let mut tru_dist = 0;
        let  distance  = iter.next();
        match distance
        {
            Some(value)=>
            {
                //Is value a
                    match  value.parse::<i32>()
                {
                    Ok(n)=> 
                    {
                        tru_dist = n;
                    },
                    Err(_) => print!(""),
                };
            }
            None => print!(""),
        }
        println!("word: {}",direction);
        if direction == String::from("forward")
        {
            z += tru_dist;
            x += tru_dist*y;
        }
        if direction == String::from("up")
        {
            y -= tru_dist;
        }
        if direction == String::from("down")
        {
            y+= tru_dist;
        }
    
    }
    println!("{}",z);
    result = x*z;
    return result;
}

pub fn day_2()
{
    println!("Day 2!");
    let test_input = utility::get_input_from_filename(&String::from("input/day_2_test.txt"));
    let input = utility::get_input_from_filename(&String::from("input/day_2.txt"));
   // print!("{}",&test_input);
    assert_eq!(part_1(&test_input),150);
    assert_eq!(part_2(&test_input),900);
    let result_1 = part_1(&input);
    let result_2 = part_2(&input);
    println!(" Andwer of day 2 part 1 = {} part 2 = {}" , result_1 , result_2);

}