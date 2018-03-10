pub fn raindrops(n: usize) -> String {
    let mut v: Vec<String> = vec![];

    if n % 3 != 0 && n % 5 != 0 && n % 7 != 0 {
        v.push(n.to_string());
    } else {
        if n % 3 == 0 {
            v.push("Pling".to_string());
        }
        if n % 5 == 0 {
            v.push("Plang".to_string());
        }
        if n % 7 == 0 {
            v.push("Plong".to_string());
        }
    }
    v.join("")
}
