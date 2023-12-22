/*
꼬마 정민이는 이제 A + B 정도는 쉽게 계산할 수 있다. 이제 A + B + C를 계산할 차례이다!

첫 번째 줄에 A, B, C (1 ≤ A, B, C ≤ 1012)이 공백을 사이에 두고 주어진다.
*/

use std::io;

fn main() {
    
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("입력에러");

    let parts: Vec<&str> = input.split_whitespace().collect();

    let a: f64 = parts[0].parse().expect("정수로 변환 실패");
    let b: f64 = parts[1].parse().expect("정수로 변환 실패");
    let c: f64 = parts[2].parse().expect("정수로 변환 실패");

    println!("{}", a + b + c );
}