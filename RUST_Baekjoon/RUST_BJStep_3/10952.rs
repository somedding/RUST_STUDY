/*
두 정수 A와 B를 입력 받은 다음 , A + B 를 출력하는 프로그램을 작성하시오

입력은 여러 개의 테스트 케이스로 이루어져 있다.

각 테스트 케이스는 한 줄로 이루어져 있으며, 각 줄에 A와 B가 주어진다. (0 < A, B < 10)

입력의 마지막에는 0 두 개가 들어온다.

각 테스트 케이스마다 A+B를 출력한다.
*/
use std::io;
use io::ErrorKind;

fn main() {
    loop {
        let mut numbers_arr2 = String::new();
        io::stdin().read_line(&mut numbers_arr2).unwrap();
        if numbers_arr2.eof() {
            break;
        }
        let numbers2: Vec<&str> =
numbers_arr2.split_whitespace().collect();
        let number_first2: i32 = numbers2[0].parse::<i32>().unwrap();
        let number_first3: i32 = numbers2[1].parse::<i32>().unwrap();
        println!("{}", number_first2 + number_first3);
    }
}