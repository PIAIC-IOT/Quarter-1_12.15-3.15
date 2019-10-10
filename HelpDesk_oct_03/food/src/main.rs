#[derive(Debug)]
struct Food {
    name:String,
    quantity:u8,
    price: u16,
}
impl Food {
    
    fn print(iter:u8){
        for number in 1..(iter+1) {
        println!("Welcome in associated function {}",number);
        }
    }

    fn bill (&self){
        println!("");
        println!("Method: Your Bill for {} is Rs.{}",self.name,self.price*self.quantity as u16);
    }
}


fn main() {
    let item1 = Food {
        name:String::from("Mutton Kunna"),
        quantity:20,
        price:890,
    };

    println!("{:#?}",item1);

    Food::print(2);
    
    item1.bill();
}
