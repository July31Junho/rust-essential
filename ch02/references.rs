fn main(){
    let health:i32 = 32;
    println!("health : {} ", health);

    let mut game:&str = "Space Invaders";

    println!("Address of headlth-value: {:p}", &health);
    /*저장된 주소를 보려면 :p 포인터 연산자를 사용해야하는구나. */
    println!("Address of headlth-value: {}", &health);

    println!("address of game-value : {:p}", &game);
    println!("game-value : {} ",game);

    let mut score = 0;
    let score2 = &score;

    //*score2 = 5; //immutable 한 데이터를 수정하려고 하니깐 수정이 안되는거지.
    println!("*score2 : {} ", *score2);

    let mut score = 0;
    let score3 = &mut score; // 이렇게 mutable 하다는것을 넣어줘야 한다.

    *score3 = 5;

    println!("*score3 : {} ", *score3);

    // 한번만 참조된다는 사실!
    let score4 = &mut score;
    println!("score4 : {} ", *score4);


}