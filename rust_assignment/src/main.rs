fn fahrenheit_to_celsius(x:f64)-> f64
{
    (x*9.0/5.0)+32.0
}
fn celsius_to_fahrenheit(x:f64)-> f64
{
    (x-32.0)*(5.0/9.0)
}

fn main() {
    let mut x = 32.0;
    let mut y = celsius_to_fahrenheit(x);
     println!("{:.2}F = {:.2}C",x, y);

    for _count in 1..=5{
        x += 1.0;
        let y = celsius_to_fahrenheit(x);
        println!("{:.2}F = {:.2}C",x,y);
    }
}
//////////////////////////////////////////////////////////
//////////////////////////////////////////////////////////
    fn main(){
        let numbers: [i32; 10] = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
        for &num in numbers.iter(){
            if num % 3 == 0 && num % 5 == 0{
                println!("FizzBuzz");
            } 
            else if num % 3 == 0 {
                println!("Fizz");
            }
            else if num % 5 == 0{
                println!("Buzz");
            }
            else{
                if is_even(num){
                    println!("{} is even", num);
                }
                else {
                    println!("{} is odd", num);
                }
            }
        }
        let mut sum = 0;
        let mut index = 0;
        while index < numbers.len(){
            sum += numbers[index];
            index += 1;
        }
        println!("The sum of all numbers is: {}", sum);

        let mut largest = numbers[0];
        for &num in numbers.iter(){
            if num > largest{
                largest = num;
            }
        }
        println!("the largest number is: {}", largest);
    }
    fn is_even(n: i32) -> bool{
        n % 2 == 0
    }
////////////////////////////////////////////////////////////////
////////////////////////////////////////////////////////////////
fn main(){
    let secret: i32 = 4;
    let mut num_guesses = 0;
    let mut guess: i32 = 6;
    loop{
        
        num_guesses += 1;
        let result = check_guess(guess, secret);
        if result == 0{
            println!("Congratulations! You guessed the correct number: {}", secret);
            break;
        }
        else if result == 1 {
            println!("Your guess of {} is too high", guess);
            guess -= 1;
        }
        else{
            println!("Your guess of {} is too low", guess);
            guess += 1;
        }
    }
    println!("It took you {} guesses", num_guesses)
}
fn check_guess(guess: i32, secret: i32) -> i32{
    if guess == secret{
        return 0;
    }
    else if guess > secret{
        return 1;
    }
    else{
        return -1
    }
}


     