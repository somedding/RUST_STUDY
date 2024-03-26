
    let mut numbers_arry = String::new();
    io::stdin().read_line(&mut numbers_arry).unwrap();

    let numbers: Vec<&str> = numbers_arry.split_whitespace().collect();
    
    let num1: i32 = numbers[0].parse::<i32>().unwrap();
    let num2: i32 = numbers[1].parse::<i32>().unwrap();
    let num3: i32 = numbers[2].parse::<i32>().unwrap();

    if num1 == num2 && num1 == num3 {
        println!("{}", 10000 + num1 * 1000)
    } else if num1 == num2 || num1 == num3 {
        println!("{}", 1000 + num1 * 100)
    } else if num2 == num3 {
        println!("{}", 1000 + num2 * 100)
    } else {
        let vec = vec![num1, num2, num3];
        let result = *vec.iter().max().unwrap();
        println!("{}", result * 100);
    }
}