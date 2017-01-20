extern crate num;

use std::env;
use num::bigint::BigUint;
use num::One;

struct Fib {
    curr: BigUint,
    next: BigUint,
}

impl Iterator for Fib {
    type Item = BigUint;
    fn next(&mut self) -> Option<BigUint> {
        let tmp = self.curr.clone() + self.next.clone();
        self.curr = self.next.clone();
        self.next = tmp;
        Some(self.curr.clone())
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let number: usize = args[1].parse().unwrap();

    let fib_iter = Fib { curr: BigUint::one(), next: BigUint::one() + BigUint::one()};
    let fibs: Vec<BigUint> = fib_iter.take(number).collect();

    let digits: Vec<Vec<u64>> = fibs.iter()
        .map(|x| {
            x.to_string()
                .chars()
                .map(|d| d.to_digit(10).unwrap() as u64)
                .collect()
        })
        .collect();

    let parts: Vec<u64> = digits.iter()
        .map(|x| x.iter().sum::<u64>() % 10)
        .collect();

    let val: u64 = parts.iter().sum::<u64>() % 10;

    println!("{}", val);
}
