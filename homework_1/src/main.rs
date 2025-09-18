fn assignment1(){
    //Temp converter
}

fn assignment2(){
    //Number Analyzer
}
fn assignment3(){
    //Guessing Game
}
//Tempature Converter
const FREEZINGPOINT : f32  = 32.0;

fn fahrenheit_to_celsius(f : f32) -> f32{
   let celsius = (f-32.0)/1.8;

    return celsius
}

fn celsius_to_fehrenheit(c : f32) -> f32{
    let fehrenheit = (c*2.0)+30.0;
    return fehrenheit
}

// Number Analyzer
fn is_even(n: i32) -> bool{
    if n%2 == 0 {
        return true
    }
    else{
        return false
    };
}
//Guesing Game

fn check_guess(guess : i32, secret : i32) -> i32 {
    if guess == secret {
        return 0
    }
    else if guess < secret{
        return -1
    }
    else {
        return 1
    }
}

fn main(){
     assignment1();
    let x = 80.5;

    println!("{}",fahrenheit_to_celsius(x));

    let z = 30.6;

    println!("This is cesius to fahrenheit {}",celsius_to_fehrenheit(z));
    
    let mut y = 66.0;

    let mut i = 0;

    while i < 5{
        println!("{}",fahrenheit_to_celsius(y));
        y= y+1.0;
        i+=1;
    }
    assignment2();
    println!("Assingment 2 ");

    let array = [100,20,3,4,5,60,7,8,9,10];

    let mut i = 0;

    while i<array.len(){
        println!("{}",is_even(array[i]));
        if array[i]%3 == 0{
            println!("Fizz");
        }
        else if array[i]%5 == 0{
            println!("Buzz");
        }
        else if array[i]%3 == 0 && array[i]%5 == 0{
            println!("FizzBuzz");
        }
        i+=1;
    }

    let mut j = 0;
    let mut total = 0;

    while j < array.len(){
        total = array[j]+total;
       j+=1;
    };
   println!("{}",total);

   let mut k = 0;
   let mut largestNum = array[0];
    while k < array.len(){
       if array[k]>largestNum{
        largestNum = array[k];
       }
        k+=1;

    }
    println!("{}",largestNum);


    assignment3();
    let mut myNum:i32  = 56;
    let guesses = [24,86,13,5,56];
    let mut guess_count = 0;
    for guess in guesses {
        let result = check_guess(guess,myNum);
        if result == 1 {
            println!{"Oops {} is too high!",guess};
        }
        else if result == -1{
            println!{"Uh-Oh {} is too low",guess};
        }
        else{
            println!{"{} Is right you got it!",guess};
            
        }
        guess_count+=1;
    }
    println!{"It took you {} Guesses",guess_count};
    


}