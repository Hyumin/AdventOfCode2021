#[path =  "utility.rs"] 
pub mod utility;

pub fn day1()
{
    println!("Day 1!");
    let test_input = utility::get_input_from_filename(&String::from("input/test.txt"));
    print!("{}",&test_input);
}