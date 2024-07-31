fn main(){
    println!("Hello, world!");

    let x = 5;
    let y = {
        let x = 3;
        x + 1
    };

    println!("The value of x is: {}", x);
    println!("The value of y is: {}", y);

    // SCALAR TYPES

    //unsigned integer (non negative integers)
    // u8 u16 u32 u64 u128 usize
    // u8: 0 to 255
    // u16: 0 to 65535
    // u32: 0 to 4294967295
    // u64: 0 to 18446744073709551615
    // u128: 0 to 340282366920938463463374607431768211455
    // usize: depends on the computer architecture 64 bit or 32 bit
    let unsigned: u8 = 10;

    println!("The value of unsigned is: {}", unsigned);

    //signed integer (positive and negative integers)
    // i8 i16 i32 i64 i128 isize  
    // i8: -128 to 127
    // i16: -32768 to 32767
    // i32: -2147483648 to 2147483647
    // i64: -9223372036854775808 to 9223372036854775807
    // i128: -170141183460469231731687303715884105728 to 170141183460469231731687303715884105727
    // isize: depends on the computer architecture 64 bit or 32 bit
    let signed: i8 = -10;
    
    println!("The value of signed is: {}", signed);

    let  float: f32 = 3.14;
    let  double: f64 = 3.141592653589793238462643383279502884197169399375105820974944592307816406286208998628034825342117067982148086513282306647093844609550582231725359408128481117450284102701938521105559644622948954930381964428810975665933446128475648233785662134967341853;

    println!("The value of float is: {}", float);
    println!("The value of double is: {}", double);

    let boolean: bool = true;
    println!("The value of boolean is: {}", boolean);

    let character: char = 'a';
    println!("The value of character is: {}", character);
}