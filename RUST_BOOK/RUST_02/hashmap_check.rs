use::std::collections::HashMap;

fn main()
{
    let mut map = HashMap::new();
    map.insert("A", 30);
    map.insert("B", 50);

    if map.get("D") == None //get 메서드 : Option값(무언가의 유무 확인)을 반환하는 메서드
    {
        println!("D는 존재하지 않음")
    }

    else
    {
        println!("D={}", map["D"]);
    }
}

/* 
none: 값이 존재 하지 않을 때 
some: 뭔가 값이 있을떄
*/

