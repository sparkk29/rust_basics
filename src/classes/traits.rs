
// Rust does not support inheritance. It uses traits to achieve polymorphism.
// Traits are similar to interfaces in other languages. They define a set of methods that a type must implement.
// A type can implement a trait by providing the implementation for the methods defined in the trait.
// A type can implement multiple traits.
// A trait can have default implementations for some or all of its methods.
// A trait can be used to define a generic type.

fn main(){

    let bird = Bird{
        name: String::from("Eagle"),
        attack: 5,
    };

    bird.print_name();
    println!("Can the bird fly? {} and is it an animal? {} ", bird.can_fly(), bird.is_animal());
}

struct Bird{
    name: String, //
    attack: u64,
}

impl Bird{
    fn print_name(&self){
        println!("The name of the bird is: {}", self.name);
    }
}

trait Animal{
    fn can_fly(&self) -> bool;
    fn is_animal(&self) -> bool{
        true
    }
}

impl Animal for Bird{
    fn can_fly(&self) -> bool{
        true
    }
}