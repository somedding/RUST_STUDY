
use std::io;

fn main(){
    let mut number = String::new();

    io::stdin().read_line(&mut number).expect("정수 임력 오류");
    let numbers: Vec<&str> = number.split_whitespace().collect();

    let num: i32 = numbers[0].parse::<i32>().expect("정수 입력 오류");

    for i in 1..num + 1 {
        for j in 1..i + 1{
            if j <= i {
                print!("*");
            }
        }
        print!("\n");
    }
}