use rand::seq::SliceRandom;

fn main()
{
    //1에서 75까지의 값을 벡터에 대입

    let mut nums = vec![];
    for i in 1..=75
    {
        nums.push(i);
    }

    //섞기
    let mut rng = rand::thread_rng();
    nums.shuffle(&mut rng);

    //카드섞기
    for i in 0..25
    {
        if i == 12
        {
            print!(" *,");
        }

        
        else 
        {
            print!("{:3},", nums[i]);
        }
        if i % 6 == 4 {println!(""); }
    }
}