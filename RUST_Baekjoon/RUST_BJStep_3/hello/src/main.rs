
use iowrap::Eof;
use std::io;

fn main() {
    loop {
        let mut numbers_arr2 = String::new();
        io::stdin().read_line(&mut numbers_arr2).unwrap();
        let numbers2: Vec<&str> = numbers_arr2.split_whitespace().collect();
        let number_first2: i32 = numbers2[0].parse::<i32>().unwrap();
        let number_first3: i32 = numbers2[1].parse::<i32>().unwrap();
        if numbers_arr2.Eof() {
            break;
        }
        println!("{}", number_first2 + number_first3);
    }
}
