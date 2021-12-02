#[path =  "utility.rs"] 
pub mod utility;


fn part_1 (input : &String) -> u32
{
    let mut output = 0;
    let array_in_integers = utility::convert_to_uintegers_32(input);

    let mut prev = array_in_integers[0];
    for i in 1..array_in_integers.len()
    {
        if prev < array_in_integers[i]
        {
            output+=1;
        }
        prev = array_in_integers[i];
    }

    return output;
}
fn part_2(input : &String) ->u32
{
    let mut output = 0;
    let array_in_integers = utility::convert_to_uintegers_32(input);

    let mut prev = array_in_integers[0]+array_in_integers[1]+array_in_integers[2];
    for i in 1..array_in_integers.len()-2
    {
        let triplet = array_in_integers[i]+array_in_integers[i+1]+array_in_integers[i+2];
        if prev < triplet
        {
            output+=1;
        }
        prev = triplet;
    }

    return output;
}

pub fn day1()
{
    println!("Day 1!");
    let test_input = utility::get_input_from_filename(&String::from("input/day_1_test_.txt"));
    let input = utility::get_input_from_filename(&String::from("input/day_1.txt"));
   // print!("{}",&test_input);
    assert_eq!(part_1(&test_input),7);
    assert_eq!(part_2(&test_input),5);
    let result_1 = part_1(&input);
    let result_2 = part_2(&input);
    println!(" Andwer of day 1 part 1 = {} part 2 = {}" , result_1 , result_2);

}