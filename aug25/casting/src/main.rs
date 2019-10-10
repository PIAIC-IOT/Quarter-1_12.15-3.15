fn main() {
    let utype: u8 = 5;
    let ftype: f32 = 44.44;
    let utype_ftype = ftype + utype as f32;
    let ftype_utype = ftype as u8 + utype ;
    println!("UF {}",utype_ftype);
    println!("FU {}",ftype_utype);
}
