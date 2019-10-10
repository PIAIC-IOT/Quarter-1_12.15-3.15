fn main() {
    let age = 35;
    
    if age >= 20 && age<=55   {
        // 55 >=20 aur 55<=55
        // true  aur true    
        println!("&& if Your age is {}",age);
    }  // end of if
    
    if age == 30 || age==40   {
        // 30 ==30 aur 56==40
        // true  aur false    
        println!("Or IF Your age is {}",age);
    }  // end of if

    println!("End of Program");
}
