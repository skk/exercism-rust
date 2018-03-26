fn _collatz(n: u64, iteration_count: u64) -> Option<u64> {
    if n == 0 {
        return None;
    } else if n == 1 {
        return Some(iteration_count);
    } else {
        if n % 2 == 0 {
            return Some(_collatz(n / 2, iteration_count + 1).unwrap());
        } else {
            return Some(_collatz(3 * n + 1, iteration_count + 1).unwrap());
        };
    }
}

// return Some(x) where x is the number of steps required to reach 1
pub fn collatz(n: u64) -> Option<u64> {
    _collatz(n, 0)
}
