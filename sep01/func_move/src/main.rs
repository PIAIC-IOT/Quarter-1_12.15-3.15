fn main() {
    let name = String::from("OVAIS MEMON");  
    moving(name.clone());  
    println!("main {}",name);           
    let age = 5;                      
    age_copy(age);
    println!("main {}",age);       
} 
fn moving(some_string: String) { 
    println!("user {}", some_string);
}
fn age_copy(some_integer: i32) { 
    println!("user {}", some_integer); 
}
