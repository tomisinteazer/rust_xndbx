pub fn composer() -> String {
    //final impl fast but slow compared to javascript
    let hexletters = ["a", "b", "c", "d", "e", "f"];
    let colorhex: Vec<_> = hexletters
        .iter()
        .map(|_| match fastrand::usize(0..16) <= 6 {
            true => hexletters[fastrand::usize(0..6)].to_string(),
            _ => fastrand::usize(0..10).to_string(),
        })
        .collect();
    return colorhex.join("");
}

// fn composer_multiple() -> String {
//this function turned out to be mutually exclusive
//     let mut hexletters = vec![
//         "a", "b", "c", "d", "e", "f", "1", "2", "3", "4", "5", "6", "7", "8", "9", "0",
//     ];
//     fastrand::shuffle(&mut hexletters);
//     let colorhex: Vec<_> = hexletters
//         .iter()
//         .take(6)
//         .map(|hex| hex.to_string())
//         .collect();
//     return colorhex.join("");
// }
