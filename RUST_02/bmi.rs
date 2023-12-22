fn main()
{
    let height_cm = input("키(cm): ");
    let weight = input("몸무게(kg): ");

    //Bmi 계산
    let height = height_cm / 100.0;
    let bmi = weight / height.powf(2.0);
    println!("BMI = {:.1}", bmi);

    //비만도 진단
    if bmi < 18.5 {println!("저체중");}
    else if bmi < 23.0 {println!("정상");}
    else if bmi < 25.0 {println!("과체중");}
    else if bmi < 30.0 {println!("1단계 비만");}
    else if bmi < 35.0 {println!("2단계 비만");}

    else {println!("3단계 비만");}
    
}



//input 함수 정의 표준 입력엣 ㅓ한줄 씩 읽어서 f64타입으로 변환 하는 함수

fn input(prompt: &str) -> f64
{
    println!("{}", prompt);


    let mut s = String::new();
    std::io::stdin().read_line(&mut s).expect("입력 에러");


    // 변수의 공백 제거후 f64 타입으로 변환해 반환
    // parse 메서드 >> 문자여를 숫자열로 반환
    return s.trim().parse().expect("숫자가 아닙니다.")
}