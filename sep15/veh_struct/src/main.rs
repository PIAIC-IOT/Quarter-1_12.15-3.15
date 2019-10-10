struct Vehcile {
    manuf: String,
    model: u16,
    price: u64,
    color: String,
} 
fn main() {
    let mut veh1 = Vehcile {
       color: String::from("Black"),
       model: 2020, 
       manuf: String::from("Suzuki"),
       price: 15_00_000,
    };
    println!("manuf: {}",veh1.manuf);
    veh1.model=2025;
    println!("model: {}",veh1.model);
}
