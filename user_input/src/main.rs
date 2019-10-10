use std::io;  // Standard input outpit library provides you with a number of useful
              // features, including the ability to accept user input
fn main() {   //start of main function
        println!("Please input number :");   //Print static string
        let mut data = String::new();      //Declaring new String Type Variable
        io::stdin().read_line(&mut data).expect("Failed to read line");  
        println!("You entered: {:?}", data); //it will print your entered data
        
        //Getting input from user
        let converted_integer: u32 = data.trim().parse().unwrap();  //declaring a u32 variable


        //above method used to convert String input into Integer or 
        //float whatever you want.
        println!("You entered converted: {}", converted_integer); //it will print your entered data

        // let converted_int: u32 = match data.trim().parse() {
        //     Ok(num) => num,
        //     Err(_) => 0,
        // };
        //println!("You entered: {}", converted_int); //it will print your entered data
} //scope ends of main function