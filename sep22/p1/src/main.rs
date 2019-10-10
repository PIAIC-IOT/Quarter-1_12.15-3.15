// // #[derive(Debug)] //anotation
// // struct Rectangle {
// //     width: u32,
// //     height: u32,
// // } //Definition

// // fn main() {
// //     let rect1 = Rectangle { width: 30, height: 50 };

// //     println!(
// //         "The area of the rectangle is {} square pixels.",
// //         area(&rect1)
// //     );
    
// //     println!("{:#?}",rect1); //:#?
// //     println!("{:?}",rect1); //:#?
// //     println!("{:x?}",rect1); //:#?
    
// // }

// // fn area(rectangle: &Rectangle) -> u32 {
// //     rectangle.width * rectangle.height
// // }

// struct Rectangle {
//     width:u32,
//     height:u32,
// }

// impl Rectangle {
//     fn area(&self)-> u32 {
//         self.width*self.height
//     }
// }

// fn main () {
//     let rec1 = Rectangle {width:50,height:100};
//     let rec2 = Rectangle {width:10,height:90};
    
//     println!("Rec1:{}",rec1.area());
//     println!("Rec2:{}",rec2.area());

// }



#[derive(Debug)]
struct Rectangle {
    width:u32,
    height:u32,
}

impl Rectangle {
    fn size (value:u32, value_1:u32) -> Rectangle { //sig 
        Rectangle {
            width:value,
            height:value_1,
        }
    }
}
impl Rectangle {

fn area(&self) -> u32 {
    self.width * self.height //50
    }
}
fn main () {

let rec1 = Rectangle::size(5,10);
let rec2 = Rectangle::size(4,9);
println!("{}",rec1.width);
println!("{:#?}",rec2);
println!("Area of rectangle1 is: {}",rec1.area());

}















