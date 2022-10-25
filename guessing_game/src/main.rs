use std::io; // 모든 프로그램의 스코프에 prelude(서곡) 내의 타입들을 가져옴
use std::cmp::Ordering;
use rand::Rng;

fn main(){
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1, 101);

    println!("The secret number is: {}", secret_number);

    loop {
        println!("Please input your guess.");

        let mut guess = String::new(); //빈 스트링을 형성
    
        //러스트에서는 변수는 기본적으로 불변이다.
    
        let foo = 5; // immutable
        let mut bar = 5; // mutable
    
        io::stdin().read_line(&mut guess).expect("Failed to read line");
    
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };
            
        println!("You Guessed: {}", guess);
    
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
}