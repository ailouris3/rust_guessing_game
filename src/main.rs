use std::io; // tarnished input ahead
use std::cmp::Ordering; // be wary of infinite 'if' statements, try mercy
use rand::Rng; // behold, trait for random number generator !

fn main() {



    // first off, configure user-adjustable range 

        let mut range = String::new();

        println!("please input a range (e.g 10, 100, 1000) !!! ");
    
        io::stdin()
            .read_line(&mut range)
            .expect("failed to read line ,,,");

        let range:u32 = range
            .trim()
            .parse()
            .expect("bro if you seriously got this error you need to input a number between 0 and whatever giant fuck off number (2^32)-1 is equal to. you arent deserving of 64 bits anyway");
        
        println!("you have selected range ,,, {range}");



    // let there be secret number !

        let secret_number = rand::thread_rng()
            .gen_range(1..=range);
        
        println!("secret number generated !!! ");


    // offer guess

        loop {
    
            println!("please guess the number ,,, ");
    
            let mut guess = String::new();

            io::stdin()
                .read_line(&mut guess)
                .expect("failed to read line ,,,");

            println!("Your guess is ,,, {guess}");



    // 'guess' type conversion required ahead (String -> u32)

            let guess: u32 = match guess
                .trim()
                .parse() {
                    Ok(num) => num,
                    Err(_) => continue,
                };
            
            // bro i could be playing sekiro rn ,,, however i love rust
            // also my mum said shes gonna call me rusty from now on
            // its a cute nickname tbh
            // autism aint that bad
            // c:



    // could this be secret number ?

            match guess.cmp(&secret_number) {
                Ordering::Less => println!("too small !"),
                Ordering::Greater => println!("too large !"), // why is it always user input ?
                Ordering::Equal => {
                    println!("You win !");
                    break;
                }

            }

        } // ahh, end of loop ,,,

} // try jumping
