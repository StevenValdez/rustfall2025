fn assignment1(){
    //Temp converter
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

   
    
}