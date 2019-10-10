fn main() {
    let salary =30_000;
    println!("Salary         : {}",salary);
    println!("Salary Pointer : {:p}",&salary);
    let salary1 = &salary;
    println!("");
    println!("Salary1 data        : {}",salary1);
    println!("Salary1 pointer      : {:p}",salary1);
    println!("Salary1 self Pointer : {:p}",&salary1);
}
