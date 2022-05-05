//we are building a color hexer  composer i dont know why rust is casing strict but i hope i can turn that off later

//use std::thread;

//use rand::distributions::{Alphanumeric, DistString};
use rand::distributions::Alphanumeric;
use rand::{thread_rng, Rng};

fn hex_char_gen() -> String {
    //first impl fast but slow compared to javascript
    let hexletters = ["a", "b", "c", "d", "e", "f"];
    let scale_weight = thread_rng().gen_range(0..2);
    let mut current_letter = "".to_string();
    match scale_weight {
        0 => current_letter = hexletters[thread_rng().gen_range(0..6)].to_string(),
        _ => current_letter = current_letter + &thread_rng().gen_range(0..10).to_string(),
    }
    return current_letter;
}

fn composer_alphanum() -> String {
    //faster than js but distribution of alphanumeric characters not ideal
    // also really short code essentially a one liner
    let rand_string: String = thread_rng()
        .sample_iter(&Alphanumeric)
        .take(6)
        .map(char::from)
        .collect();

    return rand_string.to_lowercase();
}
fn composer() -> String {
    let hexcode = "#".to_string()
        + &hex_char_gen()
        + &hex_char_gen()
        + &hex_char_gen()
        + &hex_char_gen()
        + &hex_char_gen()
        + &hex_char_gen();
    return hexcode;
}

fn composertwo() -> String {
    // fast but still slower than javascript
    let charset: &[u8] = b"1234567890abcdef";
    let rand_string: String = (0..7)
        .map(|_| {
            let idx = thread_rng().gen_range(0..charset.len());
            charset[idx] as char
        })
        .collect();
    return rand_string;
}

fn slow_hex_char_gen() -> String {
    // works but slow shortest amount of code though
    let hexletters = [
        "a", "b", "c", "d", "e", "f", "1", "2", "3", "4", "5", "6", "7", "8", "9", "0",
    ];
    let current_letter = hexletters[rand::thread_rng().gen_range(0..hexletters.len())].to_string();
    return current_letter;
}

fn main() {
    println!(
        "{} {} {} {}",
        composer(),
        slow_hex_char_gen(),
        composer_alphanum(),
        composertwo()
    );
}
