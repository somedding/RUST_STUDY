
/* 
use std::io;

fn main() {

    let mut input1 = String::new();
    let mut input2 = String::new();

    io::stdin().read_line(&mut input1).expect("첫 번째 입력 에러");
    io::stdin().read_line(&mut input2).expect("두 번째 입력 에러");

    let a: i32 = input1.trim().parse().expect("첫 번째 입력 변환 에러");
    let b: i32 = input2.trim().parse().expect("두 번째 입력 변환 에러");

    let parts: Vec<&str> = input2.split(" ").collect();

    let c: i32 = parts[0].parse().expect("정수로 변환 실패");
    let d: i32 = parts[1].parse().expect("정수로 변환 실패");
    let e: i32 = parts[2].parse().expect("정수로 변환 실패");

    println!("{}", a * e);
    println!("{}", a * d);
    println!("{}", a * c);
    println!("{}", a * b);
}
*/

/* 
use std::io;

fn main() {
    let mut input1 = String::new();
    io::stdin().read_line(&mut input1).expect("첫 번째 입력 에러");

    let a: f64 = input1.trim().parse().expect("첫 번째 입력 변환 에러");


    let mut input2 = String::new();
    io::stdin().read_line(&mut input2).expect("두 번째 입력 에러");

    let parts: Vec<i32> = input2
        .trim()
        .split_whitespace()
        .map(|s| s.parse().expect("정수로 변환 실패"))
        .collect();

    let c: f64 = parts[0].parse().expect("정수로 변환 실패");
    let d: f64 = parts[1].parse().expect("정수로 변환 실패");
    let e: f64 = parts[2].parse().expect("정수로 변환 실패");

    println!("{}", a * e);
    println!("{}", a * d);
    println!("{}", a * c);
    println!("{}", a * (c + d + e));
}

*/

use std::io;

fn main() {
    // 첫 번째 3자리 수 입력
    let mut input1 = String::new();
    io::stdin().read_line(&mut input1).expect("첫 번째 입력 에러");

    // 두 번째 3자리 수 입력
    let mut input2 = String::new();
    io::stdin().read_line(&mut input2).expect("두 번째 입력 에러");

    // 첫 번째 3자리 수 파싱
    let first_number: i32 = input1.trim().parse().expect("첫 번째 입력 변환 에러");

    // 두 번째 3자리 수 문자열을 각 자릿수로 분리하고 정수로 변환
    let second_number: i32 = input2.trim().parse().expect("두 번째 입력 변환 에러");
    let hundreds = second_number / 100;
    let tens = (second_number % 100) / 10;
    let ones = second_number % 10;

    println!("{}", first_number * ones);
    println!("{}", first_number * tens);
    println!("{}", first_number * hundreds);
    println!("{}", first_number * second_number);
}
