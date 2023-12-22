

use std::io;

fn main() {
    
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("입력에러");

    let parts: Vec<&str> = input.split_whitespace().collect();

    let a: i32 = parts[0].parse().expect("정수로 변환 실패");
    let b: i32 = parts[1].parse().expect("정수로 변환 실패");

    println!("{}", a + b);
}
