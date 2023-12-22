fn main()
{
    let s = "서로 사랑하면 서로 싸우면 죽는다".to_string();
    echo(s); // 소유권의 이동
    println!("{}", s);
}

fn echo(s: String)
{
    println!("{}", s);
}