/*
45분 일찍 알람 설정하기
*/

/* 
use std::io;

fn main()
{
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("입력에러");

    let mut result_h = String::new();
    let mut result_m = String::new();

    let parts: Vec<&str> = input.split_whitespace().collect();

    let h: i32 = parts[0].parse().expect("정수로 변환 실패");
    let m: i32 = parts[1].parse().expect("정수로 변환 실패");

    if m < 45
    {
        if h == 0
        {
            result_h = 23;
            result_m = (59 - (45 - m));
        }

        else
        {
            result_h = (h - 1);
            result_m = (59 - (45 - m));
        }
    }

    else if m > 46
    {
        result_h = h;
        result_m = (m - 45);
    }

    println!("{} {}",result_h,result_m)
}

*/

use std::io;
fn main() {
    let mut numbers_arry = String::new();
    io::stdin().read_line(&mut numbers_arry).unwrap();
    
    let numbers: Vec<&str> = numbers_arry.split_whitespace().collect();

    let mut hour: i32 = numbers[0].parse::<i32>().unwrap();
    let mut min: i32 = numbers[1].parse::<i32>().unwrap();
    if hour == 0 {
        if min < 45 {
            hour = 23;
            min = min + 60 - 45;
            println!("{} {}", hour, min);
        } else if min >= 45 {
            min = min - 45;
            println!("{} {}", hour, min);
        }
    } else if hour > 0 {
        if min < 45 {
            hour = hour - 1;
            min = min + 60 - 45;
            println!("{} {}", hour, min);
        } else if min >= 45 {
            min = min - 45;
            println!("{} {}", hour, min);
        }
}
}