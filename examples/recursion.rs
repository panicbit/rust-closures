extern crate closures;

use closures::RecClosure1;

fn main() {
    let mult5 = RecClosure1::new(5, |this, n| {
        if n > 0 {
            this.state() + this(n-1)
        }
        else {
            0
        }
    });

    println!("{}", mult5(9));

    assert_eq!(mult5(9), 45);
}
