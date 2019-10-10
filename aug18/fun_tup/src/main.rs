fn five() ->(i32,f64,u8)
{
    let tup=(400,5.5,1);    
    tup
}

fn main() {
    let x=five();
    println!("{} {} {}",x.0,x.1,x.2);
}
