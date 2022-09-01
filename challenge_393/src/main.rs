fn main() {
    println!("{}", change(0));
    println!("{}", change(12));
    println!("{}", change(468));
    println!("{}", change(123456));
}

fn change(units: i32) -> i32 {
    let denominations: [i32; 6] = [500, 100, 25, 10, 5, 1];
    let mut coins: i32 = 0;  // track # of coins
    let mut value = units.clone();  // track remaining value
    for coin_value in denominations {
        let new_coins = value / coin_value;
        coins += new_coins;
        value -= new_coins * coin_value
    }
    coins
}