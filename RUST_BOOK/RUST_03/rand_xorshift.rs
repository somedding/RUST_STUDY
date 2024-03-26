use std::time::{SystemTime, UNIX_EPOCH};

fn main()
{
    let mut seed = rand_init();

    for _ in 0..100
    {
        let v = rand(&mut seed, 1, 6);
        println!("{}", v);
    }
}

fn rand_init() -> u32
{
    SystemTime::now()
        .duration_since(UNIX_EPOCH).unwrap()
        .as_millis() as u32
}

fn rand(seed: &mut u32, start: u32, end: u32) -> u32
{
    *seed ^= *seed << 13;
    *seed ^= *seed >> 17;
    *seed ^= *seed << 5;
    return *seed % (end - start + 1) + start;
}

//^= 는 비트 xor 연산과 할당 연산을 합친 연산자

//0101 과 0011 의 XOR 연산은 0110

//Mutavle reference 원칙 

/*
- 한 시점에 오직 하나의 mut ref 만 존재해야 한다.
- 컴파일러는 한 문장이 아닌 코드 전체 문맥에 따라 mut ref의 갯수를 결정한다.
- immutable reference는 무수히 많아도 상관 없다.
- immut ref과 mut ref은 공존할 수 없다.
(immut ref이 참조하는 값이 mut ref으로 수정하기 전인지 후인지 판단할 수 없기 때문)
 */