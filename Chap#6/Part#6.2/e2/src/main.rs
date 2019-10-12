// #[derive(Debug)]
// enum Days { //Compiler identify the variants in integers at back-end.
//     Mon, //0
//     Tue, //1
//     Wed, //2
//     Thr, //3
//     Fri, //4
//     Sat, //5
//     Sun, //6
// }


// fn main()
// {
//     let Days = ["Mon","Tue","Wed","Thr","Fri","Sat","Sun"];
//     let Today = Days[2];  // Anyone could not tell instantly that 'Day[2]' means it's Wednesday
//     println!("Today is: {}",Today);

//     let Today = Days::Wed;  //better Readability 
//     println!("Today is: {:#?}",Today);

// }





#[derive(Debug)]
enum batch {
    online(String),
    onsite(String),
}


fn main () {
    let student_1 = batch::online(String::from("I study online!"));
    let student_2 = batch::onsite(String::from("I study onsite"));
    println!("{:#?}",student_1);
    println!("{:#?}",student_2);
}