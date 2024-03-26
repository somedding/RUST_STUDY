static mut TAX: f32 = 0.1;

fn main ()
{
    unsafe //가변번역 변수 사용전 안전하지 않다는걸 명시
    {
        println!("Price: {}", TAX * 300.0);

        TAX = 0.08;
        println!("Price: {}", TAX * 300.0);
    }
}