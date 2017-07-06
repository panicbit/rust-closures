extern crate closures;

use std::iter::Map;
use closures::Closure1;

fn main() {
    for n in add(10, vec![1,2,3]) {
        println!("{}", n);
    }
}

fn add<T>(n: i32, v: T) -> Map<T::IntoIter, Closure1<i32, i32, i32>>
    where T: IntoIterator<Item=i32>,
{
    v.into_iter().map(Closure1::new(n, |&n, m| n + m))
}
