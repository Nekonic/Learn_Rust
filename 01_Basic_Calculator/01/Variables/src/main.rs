fn add_one(x:u32)->u32{
    x+1
}

fn main() {
    let x:u32 = 8;
    let y:u32 = x+2;
    println!("x = {x} y = {y} ");
    println!("{}", add_one(x));
}
