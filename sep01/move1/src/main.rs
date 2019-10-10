fn main() {
    let s1 = String::from("PIAIC");
    let s2 = s1.clone();
    println!("s1: {}", s1);
    println!("s2: {}", s2);

    let age:u8 = 55;
    let age1:u8 = age;
    println!("age :{} age1:{}", age,age1);

    let str_lit1 = "PIAIC";
    let str_lit2 = str_lit1;
    println!("str 1  {} str 2 {}", str_lit1,str_lit2);
}
