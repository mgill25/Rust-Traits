trait Hash {
    fn hash(&self) -> u64;
}

impl Hash for bool {
    fn hash(&self) -> u64 {
        if *self { 0 } else { 1 }
    }
}

impl Hash for i64 {
    fn hash(&self) -> u64 {
        *self as u64
    }
}

trait Eq {
    fn eq(&self, other: &Self) -> bool;
    // Self here will resolve to whatever type we implement the trait for
    // for example, `impl Eq for bool` will result in `Self` being equal to `bool`
}

// We can then define a Hash Map that is generic over a type T implementing both
// Hash and Eq.
#[derive(Debug)]
struct HashMap<Key: Hash + Eq, Value> {
    key: Key,
    val: Value
}

impl Eq for i64 {
    fn eq(&self, other: &i64) -> bool {
        *self == *other
    }
}

// Static Dispatch
// Most common way to *consume* a trait is through generics:
fn print_hash<T: Hash>(t: &T) {
    println!("The Hash is {}", t.hash())
}

fn main() {
    print_hash(&true);
    print_hash(&12_i64);
    // print_hash(&12_u64); // this will result in a compiler error! 
    // Because the trait bound Hash wasn't safisfied for i64
    let hash_map = HashMap{
        key: 1 as i64,
        val: "one"
    };
    
    println!("hashmap = {:?}", hash_map);
    println!("key = {}", hash_map.key);
    println!("val = {}", hash_map.val);
    
}
