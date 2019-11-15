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

/*fn f(x: &T) {
  println!("{}", x.m())
}*/

// dyn is just syntactic signpost. This is just to make it clear that
// a coersion to a trait object is about to happen.
fn f(x: &dyn T) {
  println!("{}", x.m())
}

fn main() {
  let s1 = S1{i : 100};
  // We just use references instead of values, which leads to indirection and we can escape
  // from pre-allocating sizes at compile time for dispatching the calls.
  f(&s1);                   // Compiler does implicit coersion from &S1 to &T
  // This coersion attaches some extra information so the runtime system knows to call S1's methods
  // on that object.
  // This is Dynamic Dispatch in action.
  let s2 = S2{j : 100};
  f(&s2);
}

// In both cases, we have turned an *unsized thing* (a trait object) 
// into a sized thing (a box or a reference)
