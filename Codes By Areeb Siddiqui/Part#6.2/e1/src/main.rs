//Difference between Struct and Enums
#[derive(Debug)] // Annotation for debugging our custom data type.
struct User {
    name: String,
    age: u8,
    email: String,
    country: String,

} //Struct Definition
#[derive(Debug)]
enum Months {
    Jan,
    Feb,
    March,
    April,
} //Enum Definition

fn main () {
    let user1 = User { // making an instance of User Datatype
        name:String::from("Ali"),
        age:25,
        email:String::from("Ali@gmail.com"),
        country:String::from("Pakistan"),
    }; //all fields must be initialized.

    println!("{:#?}",user1);
    let J = Months::Jan; //it's not mandtory to use every variant of enum.
    println!("{:#?}",J); 
}

