**참조 할때는 `&` 를 붙여서 표현한다.** (sample . owner_func_ref.rs)

#### 함수에서의 참조자 변환 

Lifetime(수명 에러) > `error[E0106]: missing lifetime specifier` 
`this function's return type contains a borrowed value, but there is no value for it to be borrowed from`

에러 해결 방법: 실젯값 사용하기 > `String` 객체의 실젯값을 반환하도록 수정한다.


#### 가변 참조자를 인수로 활용하기 
- 함수의 인수로 지정한 값을 변경 > `&___` 타입 형태로 지정 
- 함수를 호출할때 > `&___` 을 지정해 함수 호출

#### `println!` 매크로에 대하여

##### `println!` 매크로는 소유권 문제가 발생하지 않는다.
이 코드는 `println!` 의 기능을하는 echo라는 함수를 정의하여 만든 코드이다 (책에 있음)

```rust
fn main()
{
    let s = "서로 사랑하면 서로 싸우면 죽는다".to_string();
    echo(s); // 소유권의 이동
    println!("{}", s);
}

fn ech(s: String)
{
    println!("{}", s);
}
```
이 코드는 컴파일 해보면 소유권 관련 에러가 발생한다.

```rust
fn main() 
{
let s = "서로 사랑하면 살고 서로 싸우면 죽는다".to_string();

println!("{}", s); //echo 대신 println! 사용
println!(”{}”, s); 
}
```
반면 이 코드는 에러가 발생하지 않는다. 두 코드 모두 같은 기능을 하지만 `println!` 매크로를 사용하면 몇번을 써도 소유권 문제가 발생하지 않는다.

##### 값을 임의 장소에 넣을 수 있다

변수나 값을 문자열 안의 임의의 장소에 넣을 수 있다. 그리고 값을 순서대로 채워넣는 것이 아니라 ‘...{2}...{1}...{0}..과 같이 순 서를 임의로 지정할 수도 있다.
예를 들어 연도 표시 방법은 나라에 따라 상이하다. 미국에서는 ‘월/ 일/년’ 순으로 쓰지만 영국에서는 ‘일/월/년’ 순으로 쓴다. 인수 순서는 변경하지 않고 서식만 변경해서 이런 문제에 대응할 수 있다.

```rust
fn main() 
{
let year = 2023;
let month = 12;
let day = 1;

println!("KR:{0}/{1}/{2}", year, month, day); // 한국
println!("US:{1}/{2}/{0}", year, month, day); // 미국 
println!("UK:{2}/{1}/{0}", year, month, day); // 영국
}
```

이 코드 처럼 숫자를 {} 내부의 숫자를 바꾸는 것 만으로 년, 월, 일을 손쉽게 바꿀 수 있다.

##### 서식 지정
파이썬 문자열의 `format` 메서드처럼 정렬, 채우기. 소수점 지정 등을 할수 있다.

`{:<n}` : n 자리 왼쪽 정렬
`{:>n}` : n 자리 오른쪽 정렬
`{:^n}` : n 자리 가운데 정렬
`{:0<n}`: n 자리 왼쪽 정렬후 0으로 채우기 (예시(n=6): `{:0<6}` > 120000) 
`{:>0n}`: n 자리 오른쪽 정렬후 0으로 채우기
`{:b}` : 이진수
`{:0nb}` : 이진수에서 n만큼 0으로 채우기 (예시(n=6): `{:08b}` > 00000100) 
`{:o}` : 8진법 (octa)
`{:x}` : 16진수
`{:0nx}` : 16진수에서 n 만큼 0으로 채우기
`{:.n}` : 소수점 이하 n 자리 표시하기 
`{:e}` : 지수 표시
`{:?}` : 디버그 출력
`{:#?}` : 디버그 출력후 형태 정렬
`{{` : `{` 출력
`}}` : `}` 출력
`{:p}` 포인터 (?)
`\"` : `"` 출력
`\\` : `\` 출력

```rust
fn main()
{
   println!("|{:>8}| #{:06x}", "red", 0xFF0000); 

   println!("|{:>8}| #{:06x}", "green", 0x00FF00); 

   println!("|{:>8}| #{:06x}", "blue", 0x0000FF);

   println!("|{:>8}| RGB{:?}", "yellow", (255,255,0));
}
```

출력하면 이렇게 나온다.
```
|     red| #ff0000
|   green| #00ff00
|    blue| #0000ff
|  yellow| RGB(255, 255, 0)
```

### 튜플

#### 튜플 만들기

> **예시 코드**
> ```rust
> fn main() 
> {
> let banana = ("바나나", 300);
> let apple = ("사과", 200);
> let total = banana.1 + apple.1;
> print_tuple(&banana);
> print_tuple(&apple);
> println!("합계는 {}원입니다.", total);
> }
> fn print_tuple(item: &(&str, i64)) 
> {
> println!("{}를 {}원에 구입.", item.0, item.1); 
> }
> ```

`("바나나", 300)` 에서 `"바나나"` 가 `banana.0` , `"300"`이 `banana.1` 이다

`print_tuple(&banana);` 에서는 소유권 이동이 발생 하므로, `&`를 이용해서 참조자를 지정한다

`fn print_tuple(item: &(&str, i64))` 에서 `print_tuple`를 정의 한다. `(&str, i64)` 를 이용해서 위의 튜플형식과 동일 하게 타입을 정한다. 물론 참조자로 튜플을 이용해야 하므로 `&` 붙이는걸 잊지 않는다.

#### 함수를 호출해서 인수를 변경하기

>**튜플을 구조체로 정의하기**
>`struct 구조체명 (타입 1, 타입 2, ... );` 

```rust
struct Item(String, i64);

fn main()
{
    let banana = Item("바나나".to_string(), 300); 
    let apple = Item("사과".to_string(), 200); 
    let mango = Item("망고".to_string(), 500);

    let items = vec![banana, apple, mango];

    let total = print_and_sum_items(&items);

    println!("합계는 {}원입니다.", total);

}

//print_tuple 정의
fn print_tuple(item: &Item) 
{
    println!("{}를 {}원에 구입", item.0, item.1); 
}

//print_and_sum_items 정의
fn print_and_sum_items(items: &Vec<Item>) -> i64 
{
et mut total = 0; 
for it in items 
{
print_tuple(&it);
total += it.1; 
}
total
}
```


### 슬라이드 타입

#### 슬라이스 : 배열 문자열 같은 데이터 의 요소 중 일부를 참조하기 위한것 (참조자 > 소유권 없음) 

사용방법 : `&변수명[n..m]` 형태

예시 코드

```rust
fn main() 
{
    let s = String::from("beep");
    let ss = &s[0..3];

    println!("{}", ss);
}
```

0 에서 부터 3까지의 슬라이스를 원하는 코드 이므로 `bee` 가 출력 된다


```rust
fn main() 
{
    let a = [0, 1, 2, 3, 4, 5];
    let a_slice = &a[0..3];

    println!("{:?}", a_slice);

    println!("{:?}", &a[3..5]); // 4번째 부터 5번째 >> [3..5]

    println!("{:?}", &a[4..6]); // 4번째 이후의 슬라이스를 얻는다

}
```

`println!("{:?}", &a[3..5]);` : [3 , 4] 

`println!("{:?}", &a[4..6]);` : [4 , 5]


### 러스트의 구조체

struct 

### Mutable reference 원칙 
- 한 시점에 오직 하나의 mut ref 만 존재해야 한다.
- 컴파일러는 한 문장이 아닌 코드 전체 문맥에 따라 mut ref의 갯수를 결정한다.
- immutable reference는 무수히 많아도 상관 없다.
- immut ref과 mut ref은 공존할 수 없다. (immut ref이 참조하는 값이 mut ref으로 수정하기 전인지 후인지 판단할 수 없기 때문)

### xorshift 알고리즘
- XOR 연산 + 비트 쉬프트


## 4-2. 공통 동작을 정의하는 트레잇

### 트레잇
트레잇(Trait)을 영어 사전에서 찾아보면 ‘특성’이라는 뜻을 찾을 수 있다. 러스트에서의 트레잇은 다른 타입에 대해 공통된 동작(메서드)을 정의하기 위해 사용된다. 트레잇은 자바나 C#에 있는 인터페이스의 기능과 유사하다.

## 4-6. 러스트의 모듈, 크레이트 패키지

### 패키지, 크레이트 모듈

러스트의 기본 단위는 모듈이다. 모듈을 모은 것이 크레이트이고, 크래이트를 관리하는 패키지가 있다.


‘모듈(module)’은 스코프(범위) 단위다. 모듈의 관계는 트리 구조로 표현할 수 있다. 
‘크레이트(crate)’는 트리 구조로 표현되는 모듈군 전체를 말한다. 
모듈의 트리 가장 위에는 크레이트가 있다.
‘패키지(package)’는 복수의 크레이트를 관리한다.
패키지 관리 시스템인 Cargo와 밀접하게 엮인 단위 로 크레이트를 빌드하거나 테스트한다.