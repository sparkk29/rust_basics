fn main(){
    // COMPOUND TYPES

    // Tuple
    let tuple: (i32, f64, u8) = (500, 6.4, 1);
    let (a, _b, _c) = tuple; // destructuring tuple
    println!("The value of a is: {}", a);
    //accessing tuple elements
    println!("The value of tuple.0 is: {}", tuple.0);
    println!("The value of tuple full is: {:?}", tuple);

    let arr: [u8;3] = [1, 2, 3];
    let other_arr: [u8;5] = [100; 5];

    // print structure of arrays and other structures
    println!("The value of arr is: {:?}", arr);
    println!("The value of arr.0 is: {}. length of array is {}", arr[0], arr.len());
    println!("The value of other_arr is: {:?}", other_arr);
}