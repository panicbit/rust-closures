extern crate closures;

use closures::ClosureMut;

fn main() {
    let mut counter1 = ClosureMut::new(0, |n| { *n += 1; *n });

    println!("counter1() = {}", counter1());

    let mut counter2 = counter1.clone();
    println!("Cloned counter1 to counter2");

    println!("counter1() = {}", counter1());
    println!("counter2() = {}", counter2());
}
