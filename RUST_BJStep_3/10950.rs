/*
문제
두 정수 A와 B를 입력받은 다음, A+B를 출력하는 프로그램을 작성하시오.

입력
첫째 줄에 테스트 케이스의 개수 T가 주어진다.

각 테스트 케이스는 한 줄로 이루어져 있으며, 각 줄에 A와 B가 주어진다. (0 < A, B < 10)

출력
각 테스트 케이스마다 A+B를 출력한다.
*/

use std::io::{self, Read};

fn main() {
    let mut input = String::new();
    loop {
        let result = io::stdin().read_line(&mut input);
        match result {
            Ok(0) => break, // EOF reached
            Ok(_) => {
                let numbers2: Vec<&str> = input.split_whitespace().collect();
                let number_first2: i32 = numbers2[0].parse::<i32>().unwrap();
                let number_first3: i32 = numbers2[1].parse::<i32>().unwrap();
                println!("{}", number_first2 + number_first3);
                input.clear(); // Clear the string for the next line of input
            }
            Err(_) => {
                eprintln!("Error reading input.");
                break;
            }
        }
    }
}
