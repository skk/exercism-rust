pub fn square(s: u32) -> u64 {
    match s {
        1 ... 64 => (2 as u64).pow(s - 1),
        _ => panic!("Square must be between 1 and 64")
    }
}

pub fn total() -> u64 {
    let values = 1..(64 + 1);
    let val: u64 = values.into_iter().map(|x| square(x)).sum();
    val
}
