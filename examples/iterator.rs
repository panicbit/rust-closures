extern crate closures;

use closures::ClosureMut;

fn main() {
    for ch in alphabet() {
        println!("{}", ch);
    }
}

fn alphabet() -> ClosureMut<u8, Option<char>> {
    ClosureMut::new(b'a'-1, |ch| {
        if *ch == b'z' {
            return None
        }

        *ch += 1;

        Some(*ch as char)
    })
}
