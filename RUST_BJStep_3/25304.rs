/*
입력
첫째 줄에는 영수증에 적힌 총 금액 
X가 주어진다.

둘째 줄에는 영수증에 적힌 구매한 물건의 종류의 수 
$N$이 주어진다.

이후 
N개의 줄에는 각 물건의 가격 
a와 개수 
b가 공백을 사이에 두고 주어진다.

출력
구매한 물건의 가격과 개수로 계산한 총 금액이 영수증에 적힌 총 금액과 일치하면 Yes를 출력한다. 일치하지 않는다면 No를 출력한다.

제한
 
1 ≤ X ≤ 1000000000 
 
1 ≤ N ≤ 100
 
1 ≤ a ≤ 1000000 
 
1 ≤ b ≤ 10
예제 입력 1 
260000
4
20000 5
30000 2
10000 6
5000 8
예제 출력 1 
Yes
*/

use std::io;

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("입력 에러");

    // 영수증에 적힌 총 금액을 정수로 변환
    let total_amount: i32 = input.trim().parse().expect("정수로 변환할 수 없습니다.");

    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("입력 에러");

    // 물건의 종류 수를 정수로 변환
    let n: i32 = input.trim().parse().expect("정수로 변환할 수 없습니다.");

    let mut calculated_total = 0; // 계산된 총 금액

    for _ in 0..n {
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("입력 에러");

        // 공백으로 분리된 가격과 개수를 읽고 정수로 변환
        let parts: Vec<i32> = input
            .trim()
            .split_whitespace()
            .map(|s| s.parse().expect("정수로 변환할 수 없습니다."))
            .collect();

        let price = parts[0]; // 가격
        let quantity = parts[1]; // 개수

        calculated_total += price * quantity; // 물건의 가격과 개수로 총 금액 계산
    }

    // 계산된 총 금액과 영수증에 적힌 총 금액을 비교하여 결과 출력
    if calculated_total == total_amount {
        println!("Yes");
    } else {
        println!("No");
    }
}