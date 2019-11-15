// https://tratt.net/laurie/blog/entries/a_quick_look_at_trait_objects_in_rust.html
trait T {
    fn m(&self) -> u64;
}

struct S {
    i: u64
}

impl T for S {
    fn m(&self) -> u64 {
        self.i
    }
}

// Sort-of solution to consume type that calls .m()
// When we specify the type parameter X, monomorphisation kicks in,
// and a specialized version of f is generated for every distinct type we
// want to pass to X, allowing Rust to make everything statically dispatched.
fn f<X: T>(x: X) {
    println!("{}", x.m())
}

fn main() {
    let s = S{i: 100};
    f(s);
}
