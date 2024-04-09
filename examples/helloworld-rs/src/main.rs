use std::vec::Vec;

fn main() {
  println!("helloworld!");
  let mut v = Vec::new();

  v.push(1);
  v.push(2);

  println!("First element: {}", v[0]);
}
