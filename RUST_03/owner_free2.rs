fn main() {
    {
        let s1 = String::from("재능은 한계가 있지만 노력엔 한계가 없다");
        let s3 = String::from("계단을 밟아야 계단 위에 올라설 수 있다");

        {
            let s2 = s1;
            println!("{}", s2);
        }
        println!("{}",s3);
    }
}