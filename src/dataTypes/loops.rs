fn main(){

    // IF STATEMENT
    let number = 3;
    if number < 5 {
        println!("Condition is true");
    } else {
        println!("Condition is false");
    }

    // FOR LOOP
     for i in 0..6 {
         println!("{}", i);
     }
 
    // WHILE LOOP
    let mut number = 0;
    while number < 4 {
        println!("Number is: {}", number);
        number += 1;
        if number == 3 {
            println!("Number is :::>>>> 3");
            continue;
        }
    }

    // MATCH
    let number = 3;
    match number {
        0 => println!{"Zero"},
        1|2 => println!("One or Two"),
        3..=5 => println!("Three to Five"),
        _ => println!("Something else"),
    }
}