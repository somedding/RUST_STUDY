use std::io;

fn main()
{
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("입력에러");

    let parts: Vec<&str> = input.split_whitespace().collect();

    let a: i32 = parts[0].parse().expect("정수로 변환 실패");

    if 89 < a && a < 101
    {
        println!("A")
    }


    else if 79 < a && a < 90
    {
        println!("B")
    }

    else if 69 < a && a < 80
    {
        println!("C")
    }

    else if 59 < a && a < 70
    {
        println!("D")
    }

    else
    {
        println!("F")
    }
}