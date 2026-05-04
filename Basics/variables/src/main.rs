// const SUM_OF_TWO_NUMBERS: i32 = 4 +8;
fn main() {
    // println!("{SUM_OF_TWO_NUMBERS}");
    // let x = 5;

    // let x = x +1;

    // {
    //     let x = x *2;

    //     println!("The value of x is {}", x)
    // }

    // println!("The original value of x is still {}", x)

    //TUPLES///

    //Can contain multiple types, have a fixed length,empty tuple is ccalled a unit () and is returned when an expression returns no/empty value.

    // let my_tup: (i32, f64, u8, String) = (21, 6.2, 1, String::from("This is a tuple item"));

    // println!("{}", my_tup.3)


    //ARRAYS

    //Must contain elements of the same type, have fixed length (does not grow or shrink)

    let a: [f64; 4] = [2.0, 1.4, 5.6, 7.2];
    println!("{}", a[3])

}
