fn main(){

    let bird = Bird{
        name: String::from("Eagle"),
        attack: 5,
    };

    bird.print_name();
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