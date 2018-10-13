pub fn verse(n: i32) -> String {
    let bottle_or_bottles = if n >= 3 { "bottles".to_string() } else { "bottle".to_string() };

    return match n {
        0 => "No more bottles of beer on the wall, no more bottles of beer.\nGo to the store and \
        buy some more, 99 bottles of beer on the wall.\n".to_string(),
        1 => "1 bottle of beer on the wall, 1 bottle of beer.\nTake it down and pass it around, no \
         more bottles of beer on the wall.\n".to_string(),
        _ => format!("{} bottles of beer on the wall, {} bottles of beer.\nTake one down and pass \
        it around, {} {} of beer on the wall.\n", n, n, n - 1, bottle_or_bottles),
    };
}

pub fn sing(start: i32, end: i32) -> String {
    let mut verses_idx = (end..start + 1).collect::<Vec<_>>();
    verses_idx.sort_by(|a, b| b.cmp(a));
    verses_idx.iter().map(|idx| verse(*idx)).collect::<Vec<_>>().join("\n")
}
