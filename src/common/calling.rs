// use anchor_lang::prelude::*;

fn main() {
    // msg!("Hello ");
    println!("{}", is_even(2));
    callings();
}


pub fn is_even(num: u8) -> bool{
    let result: u8 = num % 2;

    result == 0 // return bool. semicolon is not needed here for return
}

// test cases
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_even() {
        assert_eq!(is_even(2), true);
        assert_eq!(is_even(3), false);
    }
}

// cargo test -- --nocapture
// cargo test -- --nocapture --test-threads=1
// cargo test -- --nocapture --test-threads=1 --ignored
// cargo test -- --nocapture --test-threads=1 --ignored test_is_even
// cargo test -- --nocapture --test-threads=1 --ignored test_is_even -- --ignored
// cargo test -- --nocapture --test-threads=1 --ignored test_is_even -- --ignored --nocapture

pub fn callings(){
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