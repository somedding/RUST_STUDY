
/* 
입력
첫 줄에 테스트케이스의 개수 T가 주어진다. T는 최대 1,000,000이다. 다음 T줄에는 각각 두 정수 A와 B가 주어진다. A와 B는 1 이상, 1,000 이하이다.

출력
각 테스트케이스마다 A+B를 한 줄에 하나씩 순서대로 출력한다.

예제 입력 1 
5
1 1
12 34
5 500
40 60
1000 1000
예제 출력 1 
2
46
505
100
2000
*/
/* 
use std::io;

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("입력 에러");

    // 테스트 케이스의 개수를 정수로 변환
    let t: i32 = input.trim().parse().expect("테스트 케이스 개수를 읽을 수 없습니다.");

    for _ in 0..t {
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("입력 에러");

        // 공백으로 분리된 A와 B를 읽고 정수로 변환
        let parts: Vec<i32> = input
            .trim()
            .split_whitespace()
            .map(|s| s.parse().expect("정수로 변환할 수 없습니다."))
            .collect();

        let a = parts[0];
        let b = parts[1];

        // A와 B를 더한 결과를 문자열로 만듭니다.
        let result = format!("{}", a + b);

        // 결과를 출력
        println!("{}", result);
    }
}
*/

use std::fmt::Write;
use std::io;
fn main() {
    let mut numbers_arry = String::new();
    io::stdin().read_line(&mut numbers_arry).expect("정수처리 에러");

    let numbers: Vec<&str> = numbers_arry.split_whitespace().collect();
    let number_first: i32 = numbers[0].parse::<i32>().expect("정수처리 에러");
    let mut output = String::new();

    for _ in 0..number_first {
        let mut numbers_arr2 = String::new();
        io::stdin().read_line(&mut numbers_arr2).expect("정수처리 에러");
        let number2: Vec<&str> = numbers_arr2.split_whitespace().collect();
        
        let number_first2: i32 = number2[0].parse::<i32>().expect("정수처리 에러");
        let number_first3: i32 = number2[1].parse::<i32>().expect("정수처리 에러");
        writeln!(output, "{}", number_first2 + number_first3).expect("정수처리 에러");
    }

    print!("{}", output);
}