pub fn square_of_sum(n: usize) -> usize {
    let values = 1..n + 1;
    let val: usize = values.sum();
    val.pow(2u32)
}

pub fn sum_of_squares(n: usize) -> usize {
    let values = 1..n + 1;
    let val: usize = values.into_iter().map(|x| x.pow(2)).sum();
    val
}

pub fn difference(n: usize) -> usize {
    square_of_sum(n) - sum_of_squares(n)
}
