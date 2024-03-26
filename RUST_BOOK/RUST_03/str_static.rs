fn echo(s: &`static str)
{
    println!("{}", s);
}

fn main()
{
    echo{"웅변은 은이요"}
    echo{"침묵은 금이다"}
}

