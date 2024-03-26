use std::fmt::Write;
use std::io;
fn main() {
    let mut numbers_arry = String::new();
    io::stdin().read_line(&mut numbers_arry).expect("정수처리 에러");

    let numbers: Vec<&str> = numbers_arry.split_whitespace().collect();
    let number_first: i32 = numbers[0].parse::<i32>().expect("정수처리 에러");
    let mut output = String::new();

    for i in 0..number_first {
        let mut numbers_arr2 = String::new();
        io::stdin().read_line(&mut numbers_arr2).expect("정수처리 에러");
        let number2: Vec<&str> = numbers_arr2.split_whitespace().collect();

        let number_first2: i32 = number2[0].parse::<i32>().expect("정수처리 에러");
        let number_first3: i32 = number2[1].parse::<i32>().expect("정수처리 에러");
        writeln!(output, "Case #{}: {} + {} = {}", i + 1, number_first2, number_first3, number_first2 + number_first3).expect("정수처리 에러");
    }

    print!("{}", output);
}