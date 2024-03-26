fn add <T: std::ops::Add<Output=T>> (a:T, b:T) -> T {
    a + b
}

fn main() {
    println!("{}", add(10,25));
    println!("{}", add(10.0,25.0));
    println!("{}", add::<i32>(10,25)); //타입을 명시 할 경우
    //println!("{}", add('a','a')); //char 타입은 미구현 이라 에러 발생
}