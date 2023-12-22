
//인수의 문자열을 변경하는 함수
fn add_quote(msg: &mut String)
{
    msg.insert(0,'"');
    msg.push('"');
}

fn main()
{
    let mut msg = String::from("건강한 신체에 건강한 정신이 깃든다"); //String 타입값 만들고 변수 msg 에 대입
    println!("{}", msg);
    add_quote(&mut msg); // `&mut`추가 > 가변 참조자임을 표시 (따옴표를 추가하는 함수)
    println!("{}",msg); 
}