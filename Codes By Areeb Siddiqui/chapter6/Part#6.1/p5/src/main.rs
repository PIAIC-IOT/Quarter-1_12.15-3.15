
fn main () {
    let number = Option::Some(5);
    let name   = Option::Some ("Alex");
    let absent_number:Option<i32> = Option::None;

    println!("{:#?}",number);
    println!("{:#?}",absent_number);

}
