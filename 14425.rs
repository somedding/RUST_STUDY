use std::collections::HashSet;
use std::io::{self, BufRead};

fn main() {
    // 집합 S를 저장할 HashSet 생성
    let mut set = HashSet::new();

    // 입력 받기
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines().map(|line| line.unwrap());

    // 집합 S의 크기 N과 검사해야 하는 문자열의 개수 M 입력 받기
    let line = lines.next().unwrap(); // 첫 번째 줄
    let mut input_iter = line.split_whitespace();
    let n: usize = input_iter.next().unwrap().parse().unwrap();
    let m: usize = input_iter.next().unwrap().parse().unwrap();

    // 집합 S에 포함된 문자열 입력 받기
    for _ in 0..n {
        let word = lines.next().unwrap();
        set.insert(word);
    }

    // 검사해야 하는 문자열들을 입력 받고 집합 S에 포함되어 있는지 확인하기
    let mut count = 0;
    for _ in 0..m {
        let word = lines.next().unwrap();
        if set.contains(&word) {
            count += 1;
        }
    }

    // 결과 출력
    println!("{}", count);
}