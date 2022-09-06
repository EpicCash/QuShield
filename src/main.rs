use std::io;
use rand::Rng;
use std::cmp::Ordering;
//sign.h of dilithium

//return should be the hash of crypted block
extern int generate_pow(char* block_input, size_t input_buffer_len, char* output_buffer, size_t output_buffer_len);

//minimal example to get pow working, count '57' ocurrences in hash, the higher score wins
extern int check_block(char* block, size_t block_size);

fn main() {
    //code of pow should come here
    println!("Guess a number");
    let mut guess = String::new();
    let secret = rand::thread_rng().gen_range(1..=100);

    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");

    match guess.cmp(&secret){
        Ordering::Less => println!("por baixo"),
        Ordering::Greater => println!("por cima"),
        Ordering::Equal => println!("ACERTOOOOOOOOOOOOUUUUUUUUUUUUUU"),
    }

    println!("You guessed: {guess} the number was: {secret}");
}
