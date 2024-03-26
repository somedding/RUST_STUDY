//명령줄 인수 다루기

fn main()
{
    //명령줄 인수 취득 (?)
    let args = std::env::args();
    let mut total = 0.0;

    //명령줄 인수를 순서대로 연산
    for (i,s) in args.enumerate()
    {
        if i == 0 {continue} // 명령줄 인수의 가장 처음에는 명령어 자신의 경로 임 >> 0번째 요소는 넘기는 코드

        let num: f64 = match s.parse() //parse 메서드는 Result 값을 반환함 >> match 구문 사용
        {
            Ok(v) => v,
            Err(_) => 0.0
        };
        total += num;
    }

    println!("{}", total);
}