// test1.rs
// This is a test for the following sections:
// - Variables
// - Functions

// Mary is buying apples. One apple usually costs 2 Rustbucks, but if you buy
// more than 40 at once, each apple only costs 1! Write a function that calculates
// the price of an order of apples given the order amount. No hints this time!

// Put your function here!
pub fn calculate_apple_price(can: i32) -> i32 {
    let mut price: i32 = 2;
    let mut total: i32 = 0;
    if can > 40 {
        price = 1;
        total = price * can;
    } else {
        total = price * can;
    }

    total
}

// Don't modify this function!
#[test]
fn verify_test() {
    let price1 = calculate_apple_price(35);
    let price2 = calculate_apple_price(65);

    assert_eq!(70, price1);
    assert_eq!(65, price2);
}
