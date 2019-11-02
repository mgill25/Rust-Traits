// Traits basic (from RustByExample)
struct Sheep {
    naked: bool,
    name: &'static str
}

// This looks quite like an interface kinda like Java. What is the difference? TODO
trait Animal {
    // static method
    fn new(name: &'static str) -> Self;     // Self refers to the "implementor" type
    
    // instance methods
    fn name(&self) -> &'static str;
    
    fn noise(&self) -> &'static str;
    
    // default method
    fn talk(&self) {
        println!("{} says {}", self.name(), self.noise())
    }
}

impl Sheep {
    fn is_naked(&self) -> bool {
        self.naked
    }
    
    fn shear(&mut self) {
        if self.is_naked() {
            // implementor methods canc use the implementor's trait methods
            println!("{} is already naked...", self.name())
        } else {
            println!("{} gets a haircut!", self.name);
            self.naked = true;
        }
    }
}

// Implement the Animal trait for Sheep!
impl Animal for Sheep {
    // Self becomes the implementor type Sheep
    fn new(name: &'static str) -> Sheep {
        Sheep { name: name, naked: false }
    }
    
    fn name(&self) -> &'static str {
        self.name
    }
    
    fn noise(&self) -> &'static str {
        if self.is_naked() {
            "baaaaaah?"
        } else {
            "baaaaaah!"
        }
    }
    
    // We can override the default trait methods as well
    fn talk(&self) {
        println!("{} pauses briefly...{}", self.name, self.noise())
    }
}

fn main() {
    let mut dolly: Sheep = Animal::new("Dolly");
    
    dolly.talk();
    dolly.shear();
    dolly.talk();
}

