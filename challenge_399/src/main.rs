#[allow(unused_assignments)]

use std::convert::Into;

fn main() {
    let words : [&str; 6] = ["", "a", "z", "cab", "excellent", "microspectrophotometries"];
    for word in words {
        let mut sum : u32 = 0;
        sum = lettersum(&word);
        println!("{} = {}", word, sum)
    }
}

fn lettersum(str: &str) -> u32 {
    let mut sum: u32 = 0;
    let offset: u32 = 'a'.into();
    let chars: Vec<char> = str.chars().collect();
    for char in chars {
        let _char : char = char.to_owned();
        let ord: u32 = _char.into();
        sum += ord - offset + 1;
    }
    sum
}