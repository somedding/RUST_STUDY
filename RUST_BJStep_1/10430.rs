/*
(A+B)%C는 ((A%C) + (B%C))%C 와 같을까?

(A×B)%C는 ((A%C) × (B%C))%C 와 같을까?

세 수 A, B, C가 주어졌을 때, 위의 네 가지 값을 구하는 프로그램을 작성하시오.
*/

use std::io;

fn main() {
    
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("입력에러");

    let parts: Vec<&str> = input.split_whitespace().collect();

    let a: f64 = parts[0].parse().expect("정수로 변환 실패");
    let b: f64 = parts[1].parse().expect("정수로 변환 실패");
    let c: f64 = parts[2].parse().expect("정수로 변환 실패");

    println!("{}", (a + b) % c );
    println!("{}", (( a % c ) + ( b % c )) % c);
    println!("{}", ( a * b ) % c);
    println!("{}", (( a % c ) * ( b % c )) % c);
}