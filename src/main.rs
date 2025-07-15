use core::num;
use std::{error::Error, io::{self, BufRead}, num::ParseIntError};

fn main() -> Result<(), Box<dyn Error>> {
    
    let stdin = io::stdin();

    let mut handle= stdin.lock();

    
    let mut line_buffer= String::new();
    
    handle.read_line(&mut line_buffer)?;
    
    let test_cases = line_buffer.trim().parse::<i32>()?;
    
    for i in 0..test_cases {
        line_buffer.clear();
        handle.read_line(&mut line_buffer)?;

        let mut nk_string = line_buffer.trim().split_whitespace();
        
        let n: i32 = nk_string.next().ok_or("missing N")?.parse::<i32>()?;
        let k: i32 = nk_string.next().ok_or("missing K")?.parse::<i32>()?;

        line_buffer.clear();
        handle.read_line(&mut line_buffer)?;

        let mut result_vector = line_buffer.trim().chars().map(|x| x.to_digit(10).ok_or("not a digit")).collect::<Result<Vec<_>,_>>()?;
        println!("result_vector: {:?}", result_vector);


        let mut highest_count = 0;
        let mut which_rotation = 0;
        
        for j in 1..n+1 {

            result_vector.rotate_left(1);
            println!("After rotation {}: {:?}", j, result_vector);

            let mut total_base10 = 0;

            for l in 0..n {
                total_base10 += result_vector[l as usize] * 2_u32.pow((n - 1 - l) as u32);
                
            }

            println!("{:?}", total_base10);

            if total_base10 > highest_count {
                highest_count = total_base10;
                which_rotation = j;
            }

        }



        println!("Highest count: {:?}, which_rotation: {:?}", highest_count, which_rotation);
        
        let mut number_of_cyclic_shift_ops = which_rotation;
        for _ in 1..k {
            number_of_cyclic_shift_ops += n;
        }

        println!("Number of cyclic shift operations: {}", number_of_cyclic_shift_ops);


        println!();
    } 

    

    Ok(())
}