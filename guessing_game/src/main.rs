use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main(){ 
    println!("Guess the number!");
    loop {
    println!("Please input your guess");
    
    let secret_num = rand::thread_rng().gen_range(1,101);

    let mut guess = String::new();
    
    io::stdin()
	.read_line(&mut guess)
	.expect("Failed to read line");
    
    let guess: u32 = match guess.trim().parse() {
     Ok(num) => num,
     Err(_) => continue,
    };
   
    println!("You guessed: {}",guess);
    println!("The secret number is {}",secret_num);

    match guess.cmp(&secret_num){
	Ordering::Less => println!("Too small"),
	Ordering::Greater => println!("Too big"),
	Ordering::Equal => {
	 println!("You Win!");
	 break;	
    }
   }
  }
 }
