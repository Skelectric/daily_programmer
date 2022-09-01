use std::collections::HashMap;

fn main() {
    println!("{}", numcompare("I", "I"));
    println!("{}", numcompare("I", "II"));
    println!("{}", numcompare("II", "I"));
    println!("{}", numcompare("V", "IIII"));
    println!("{}", numcompare("MDCLXV", "MDCLXVI"));
    println!("{}", numcompare("MM", "MDCCCCLXXXXVIIII"));
}

fn numcompare(x: &str, y: &str) -> bool {
    //initialize roman numeral to value mapping
    let mut numerals: HashMap<&str,i32> = HashMap::new();
    numerals.insert("M", 1000);
    numerals.insert("D", 500);
    numerals.insert("C", 100);
    numerals.insert("L", 50);
    numerals.insert("X", 10);
    numerals.insert("V", 5);
    numerals.insert("I", 1);

    //initialize numerical values for x and y
    let mut x_val : i32 = 0;
    let mut y_val : i32 = 0;

    if x == y {false} else {

        let x_vec : Vec<&str> = x.split("").collect();
        let y_vec : Vec<&str> = y.split("").collect();

        for char in x_vec {
            let val = numerals.get(char);
            let val_i32 = check_optional(val);
            x_val += val_i32
        };

        for char in y_vec {
            let val = numerals.get(char);
            let val_i32 = check_optional(val);
            y_val += val_i32
        };
        if x_val < y_val {true} else {false}
    }
}


fn check_optional(optional: Option<&i32>) -> &i32 {
    match optional {
        Some(int) => int,
        None => &0,
    }
}