/*
입력
첫 번째 줄에는 문제의 정수 
N이 주어진다. 
(4 =< N =< 1000 ; N은 4의 배수)

출력
N 을 4로 나눈 몫 만큼 'long'을 적고, 마지막엔 int를 적는다

예제 입력 1 
4
예제 출력 1 
long int
*/

use std::io;

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("입력 에러");

    // 입력값을 정수로 변환
    let n: i32 = input.trim().parse().expect("정수로 변환할 수 없습니다.");

    // N을 4로 나눈 몫을 계산
    let int = n / 4;

    // "long"을 몫만큼 출력
    for _ in 0..int {
        print!("long ");
    }

    // 마지막으로 "int" 출력
    println!("int");
}
