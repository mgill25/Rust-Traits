trait T {
  fn m(&self) -> u64;
}

struct S1 {
  i: u64
}

impl T for S1 {
  fn m(&self) -> u64 { self.i * 2 }
}

struct S2 {
  j: u64
}

impl T for S2 {
  fn m(&self) -> u64 { self.j * 4 }
}

// Here we use a Box (smart pointer) instead of a reference
// and achieve the same effect of a Dynamic Dispatch!
fn f2(x: Box<dyn T>) {
    println!("{}", x.m())
}

fn main() {
    let b: Box<S1> = Box::new(S1{i: 100});
    f2(b);
}
