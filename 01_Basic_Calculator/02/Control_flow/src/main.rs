fn main() {

    let number = 3;

    if number < 5 {
        println!("`number` is smaller than 5");
    } else {
        println!("`number` is greater than or equal to 5");
    }

    let number = 3;
    let _message = if number < 5 {
        "smaller than 5"
    } else {
        "greater than or equal to 5"
    };
    println!("{}", _message);
}
