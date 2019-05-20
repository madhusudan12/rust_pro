use std::io;
fn main() {
    let mut buff = String::new();
    io::stdin().read_line(&mut buff).expect("Read line error");
    let n:i64 = buff.trim().parse::<i64>().unwrap();
    println!("{}",n);
    if n%2 ==0{
        println!("even");
    }
    else{
        println!("odd");
    }
}

