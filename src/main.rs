use std::{error::Error, io::{self, BufRead, Read}};

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
        let binary_string = line_buffer.trim().split("").map(|x| x.parse::<i32>().unwrap()).collect::<Vec<i32>>();

        println!("{}", binary_string);
    } 
    

    Ok(())
}