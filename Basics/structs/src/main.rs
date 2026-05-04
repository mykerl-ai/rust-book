#[derive(Debug)]
struct Rectangle {
    width: i32,
    height: i32
}

// fn area(rectangle: &Rectangle) -> i32 {
//     rectangle.width * rectangle.height
// }

impl Rectangle {
    fn area(&self) -> i32 {
        self.width * self.height
    }
}
fn main() {
   let first_rectangle = Rectangle {
    width: 20,
    height: 80
   };

   let area_val = first_rectangle.area();

   println!("The area of the rectangle is {}", area_val);
   dbg!(first_rectangle);
//    println!("This is the {:#?}", first_rectangle)
}
