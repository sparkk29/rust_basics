fn main(){
    let a: MyEnum = MyEnum::A;
    let b: MyEnum = MyEnum::B(10);
    let c: MyEnum = MyEnum::C{a: 10, b: 20};

    println!("{:?}", a);
    println!("{:?}", b);
    println!("{:?}", c);
}


#[derive(Debug)]
enum MyEnum{
    A,
    B(i32),
    C{a: i32, b: i32},
}