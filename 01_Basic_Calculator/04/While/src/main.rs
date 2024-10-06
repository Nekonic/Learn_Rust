fn main() {
    let mut sum:i32 = 0;
    let mut i:i32 = 1;
    while i < 10 {
        sum+=i;
        i+=1;
    }
    println!("sum is {}", sum);
}
