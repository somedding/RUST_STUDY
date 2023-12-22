fn main()
{
    let g1 = String:::from("실수할줄 아는 사람이 아름답다")
    show_message(&g1); //참조 값 전달
    println!("{}", g1); //소유권 이동 안됨
}