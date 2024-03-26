/*
문제
n이 주어졌을 때, 1부터 n까지 합을 구하는 프로그램을 작성하시오.

입력
첫째 줄에 n (1 ≤ n ≤ 10,000)이 주어진다.

출력
1부터 n까지 합을 출력한다.

예제 입력 1 
3
예제 출력 1 
6
*/

use std::io;

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("입력 에러");

    // 입력값을 정수로 변환
    let n: i32 = input.trim().parse().expect("정수로 변환할 수 없습니다.");

    // 1부터 n까지 합을 계산
    let mut sum = 0;
    for i in 1..=n {
        sum += i;
    }
    println!("{}", sum);
}
