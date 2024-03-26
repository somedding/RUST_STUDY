use std::io;

fn main()
{
    let mut input1 = String::new();
    io::stdin().read_line(&mut input1).expect("첫 번째 입력 에러");

    let mut input2 = String::new();
    io::stdin().read_line(&mut input2).expect("두 번째 입력 에러");

    let x_result = input1.trim().parse::<i32>();
    let y_result = input2.trim().parse::<i32>();

    match (x_result, y_result) {
        (Ok(x), Ok(y)) => {
            if x > 0 
            {
                if y > 0 
                {
                    println!("1");
                } 
                
                else 
                {
                    println!("4");
                }
            }

            if x == 0 || y == 0
            {
                println!("축");
            }

            else 
            {
                if y > 0 
                {
                    println!("2");
                } 
                else 
                {
                    println!("3");
                }
            }
        }
        _ => {
            println!("입력값을 정수로 변환할 수 없습니다.");
        }
    }
}

