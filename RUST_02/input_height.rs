fn main()
{
    let mut height;

    loop
    {
        println!("키(cm): ");
        height = input_f(0.0);

        if height > 0.0{break;}

        println!("숫자만 입력 가능합니다")
    }

    let weight = 22.0 * (height / 100.0).powf(2.0); // 제곱한 수로 height /100 만큼 함
    println!("표준체중은 {:.1}kg 입니다", weight); // 소수점 한자리 까지만 표시
}

fn input_str() -> String
{
    let mut s = String::new();
    std::io::stdin()
        .read_line(&mut s)
        .expect("입력에러");
    s.trim_end().to_string()
}

fn input_f(def:f64) -> f64
{
    let s = input_str(); // 표준입력에서 문자열 읽어오기
    match s.trim().parse() /// 문자열 양 끝 공백 제거 후 숫자 변환
    {
        Ok(v) => v, // 변환성공시 : 그 값을 반환
        Err(_) => def, // 변환 실패시 : def 반환
    }
}