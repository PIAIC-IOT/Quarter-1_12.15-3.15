use std::io;
fn main() {
    println!("Enter you name: ");
    let mut name = String::new();
    io::stdin().read_line(&mut name).expect("Error");
    println!("You enter this data : {:?}",name.trim());

    println!("Enter you age: ");
    let mut age = String::new();
    io::stdin().read_line(&mut age).expect("Error");
    println!("You enter this data : {:?}",age.trim());

    let age_int : u8 = age.trim().parse().unwrap();
    println!("You enter this data : {:?}",age_int);    

}
