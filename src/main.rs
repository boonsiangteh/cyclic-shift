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
        // bug here. need to 

        // let result_vector = line_buffer.trim().split("").filter_map(|x| x.parse::<i32>().ok()).collect::<Vec<i32>>();

        let result_vector = line_buffer.trim().chars().map(|x| x.to_digit(10).ok_or("not a digit")).collect::<Result<Vec<_>,_>>();
        println!("result_vector: {:?}", result_vector);

        // for r in result_vector {
        //     match r {
        //         Ok(digit) => print!("{} ", digit),
        //         Err(err) => print!("{} ", err),
        //     }
        // }

        // for r in result_vector {
        //     println!("{:?}", r);
        // }**

        match result_vector {
            Ok(r) => println!("{:?}", r),
            Err(e) => println!("{:?}", e),
        }
        println!();
        println!();
    } 

    

    Ok(())
}