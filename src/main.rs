// use anchor_lang::prelude::*;

fn main() {
    // msg!("Hello ");
    println!("{}", is_even(2));

    // let num = 5;
    // num = 3;      // this will throw an error because num is immutable
    // you can't assign twice to an immutable variable

    // solution
    let mut num = 5;

    if is_even(2){
        num = 7;
    }

    println!("{}", num);
    
}


pub fn is_even(num: u8) -> bool{
    let result: u8 = num % 2;

    result == 0 // return bool. semicolon is not needed here for return
}