fn main() {
    println!("Class Assignment Sep 15, 2019");
    let actual_price   = 2000;
    let discount_price = 1600;
    let customer_paid  = 5000;
    println!("Actual Price      : {}",actual_price);
    println!("Discounted Price  : {}",discount_price);
    println!("Customer Paid     : {}\n",customer_paid);
    let (discount_availed,percent_discount,balance) = billing(actual_price,discount_price,customer_paid);
    println!("Discount Availed   : {}",discount_availed);
    print!("% Discount Availed : {} %",percent_discount);
    if percent_discount<=10 {
        println!(" Azadi Offer");
    }
    else if percent_discount>10 && percent_discount<=20 { 
        println!(" Eid Offer");
    }
    else if percent_discount>20 {
        println!(" Clearance Sale");
    } 
    println!("Balance returned   : {}",balance);
}

fn billing (act_price:u16,disc_price:u16,cust_paid:u16)->(u16,u16,u16){
    let discount = act_price-disc_price;
    let percent_disc = (discount*100)/act_price;
    let bal_paid = cust_paid - disc_price;
    (discount,percent_disc,bal_paid)

}