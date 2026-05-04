fn main() {
    // let num = 6;

    // if num > 3 {
    //     println!("> 3");
    // }else {
    //     println!("<=3")
    // }
    // let mut counter = 0;
    // let count_result: i32 = loop {
    //      counter += 1;

    //      if counter > 18 {
    //         let double_count = counter * 2;

    //         break double_count;
    //      }
    // };

    // println!("Double the result is {}", count_result);


    // let mut count = 0;

    // 'count_up: loop {

    //     println!("count = {count}");

    //     let mut remaining = 20;

    //     loop {
    //         println!("remaining = {remaining}");
    //         if remaining == 18 {
    //             break;
    //         }

    //         if count == 4 {
    //             break 'count_up;
    //         }

    //         remaining -=1;
    //     }

    //     count += 1;
    // }

    // println!("End count = {count}")

    // let mut cx = 10;

    // while cx != 0 {
    //     println!("<<{cx}>>");

    //     cx -= 1
    // }

    // println!("LIFTOFF!!🚀🚀🚀")

    let a: [i32; 6] = [23, 43, 10, 5, 16, 12];

    // let mut index = 0;

    // while index  < 6 {
    //     println!("The current value is {}", a[index]);

    //     index+=1;
    // }

    for element in a {
        println!("{}", element * 2);
    }
}
