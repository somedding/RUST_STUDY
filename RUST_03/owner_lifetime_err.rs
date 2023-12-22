fn gen_message() -> &str{
    let msg = String::from("실수할줄아는 사람이 아름답다");
    return &msg;
}

fn main()
{
    let m = gen_message();
    println!("{}", m);
}