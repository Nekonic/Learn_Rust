fn main() {
    let end: i32 = 5;
    let mut sum: i32 = 0;
    for i in 1..(end + 1) {
        sum+=i;
    }
    println!("Sum: {}", sum);
}
