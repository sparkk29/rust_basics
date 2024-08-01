fn main(){
    let str: &str = "Hello Rust"; // `&str` because it is a string slice. it holds a space in memory

    let mut string: String = String::from("Hello Rust"); // type String with cap S. Used `String` class struct to reference a method `from` and type the string value.
    // This creates a sting object in the heap memory, that has a text "Hello Rust"
    // this is equivalent to a vector of u8 values. It is a growable, heap-allocated data structure. Kind of like a vector of u8 values. (dynamic size array)

    let slice = &string[0..5]; // slice of 

    println!("{}", str);
    println!("{}", string);
    println!("{}", slice);

    string.push_str(" and WebAssembly");
    println!("{}", string);

    string = string.replace("Hello", "Welcome to");

    println!("{}", string); 
}