fn main() {
    //https://www.quora.com/Is-zero-even-or-odd
    //Class Assignment
    //Write a Rust program which iterates a number from 0 to 10 and on every 
    //iteration check the number is odd or even and print on screen.

    //Solution 1 with loop
    println!("Solution 1 with loop LOOP start");
    let mut digit :u8 = 0;
    loop{
        if digit%2==0{ println!("{} is Even",digit);}
        else {println!("{} is Odd",digit);}
        digit += 1;
        if digit>10 { break }
    }
    println!("Solution 1 with loop LOOP end \n");

    println!("Solution 2 with while LOOP start");
    let mut value :u8 = 0;
    while value <=10 {
        if value%2==0{ println!("{} is Even",value);}
        else {println!("{} is Odd",value);}
        value += 1;
    }
    println!("Solution 2 with while LOOP end\n");

    println!("Solution 3 with for LOOP start");
    for numeric in 0..11 {
        if numeric%2==0{ println!("{} is Even",numeric);}
        else {println!("{} is Odd",numeric);}
    }
    println!("Solution 3 with for LOOP end\n");

    println!("https://www.quora.com/Is-zero-even-or-odd\n");
}
