/*
첫째 줄에는 별 1개, 둘째 줄에는 별 2개, N번째 줄에는 별 N개를 찍는 문제
하지만, 오른쪽을 기준으로 정렬한 별(예제 참고)을 출력하시오.

첫째 줄에 N(1 ≤ N ≤ 100)이 주어진다.

첫째 줄부터 N번째 줄까지 차례대로 별을 출력한다.
*/

use std::fmt::Write;
use std::io;
fn main() {         
    let mut array = String::new();

    io::stdin().read_line(&mut array).unwrap();

    let numbers: Vec<&str> = array.split_whitespace().collect();

    let number: i32 = numbers[0].parse::<i32>().unwrap();

    for i in 0..number {
        for j in 0..number {
            if j < number-i-1 {
                print!(" ");
            } else {
                print!("*");
            }
        }
        print!("\n");
    }
}