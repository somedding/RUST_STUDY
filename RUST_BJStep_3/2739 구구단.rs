/*N을 입력받은 뒤, 구구단 N단을 출력하는 프로그램을 작성하시오. 출력 형식에 맞춰서 출력하면 된다. */

use std::io;
fn main() {
    let mut num = String::new();

    io::stdin().read_line(&mut num).unwrap();

    let nums: Vec<&str> = num.split_whitespace().collect();
    
    let num1: i32 = nums[0].parse::<i32>().unwrap();

    for i in 1..10 {
        println!("{} * {} = {}", num1, i, num1 * i)
    }
}