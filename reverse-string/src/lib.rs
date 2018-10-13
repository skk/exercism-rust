
pub fn reverse(input: &str) -> String {
    let chars = input.chars().collect::<Vec<_>>();
    let mut data: String = String::with_capacity(input.len());

    for x in chars {
        data.insert(0, x);
    }
    data
}


//pub fn reverse(input: &str) -> String {
//    let mut chars = input.chars().collect::<Vec<_>>();
//    let mut data: String = String::with_capacity(input.len());
//    while let Some(x) = chars.pop() {
//        data.push(x);
//    }
//    data
//}

//fn reverse(input: &str) -> String {
//    let s = input.to_string();
//    let c = s.chars();
//    let rev_str = c.rev().collect::<String>();
//    rev_str
//}
