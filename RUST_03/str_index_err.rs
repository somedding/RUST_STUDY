fn main()
{
    let s = "안녕하세요";
    println!("{}", s[0]);
}

// = note: you can use `.chars().nth()` or `.bytes().nth()`
//러스트에서는 문자열을 저장한 변수에 직접 s[N]과 같은 형태로 접근 할 수 없음