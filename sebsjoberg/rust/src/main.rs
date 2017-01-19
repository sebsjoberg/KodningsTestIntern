use std::env;

struct Fib {
    curr: u64,
    next: u64,
}

impl Iterator for Fib {
    type Item = u64;
    fn next(&mut self) -> Option<u64> {
        let tmp = self.curr + self.next;
        self.curr = self.next;
        self.next = tmp;
        Some(self.curr)
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let number: usize = args[1].parse().unwrap();

    let fib_iter = Fib { curr: 1, next: 1 };
    let fibs: Vec<u64> = fib_iter.take(number).collect();

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
