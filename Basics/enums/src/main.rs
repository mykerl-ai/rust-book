// #[derive(Debug)]
// enum IpAddrKind {
//     V4(u8, u8, u8, u8),
//     V6,
// }

// fn route (ip_kind: &IpAddrKind) {
//     println!("Routing through {:#?}...", ip_kind);
// }

// fn main() {

//     let four = IpAddrKind::V4(127, 0, 0, 1);
//     let six: IpAddrKind = IpAddrKind::V6;

//     route(&four);
//     route(&six);
// }


// #[derive(Debug)] // so we can inspect the state in a minute
// enum UsState {
// Alabama,
// Alaska,
// Aloha,
// // --snip--
// }
// enum Coin {
// Penny,
// Nickel,
// Dime,
// Quarter(UsState),
// }


// fn value_in_cents(coin: Coin) -> u8 {
//     match coin {
//         Coin::Penny => 1,
//         Coin::Nickel => 5,
//         Coin::Dime => 10,
//         Coin::Quarter(state) => {
//             println!("Quarter is from {:?} state", state);
//             25
//         }
//     }
// }

// fn main () {
//     let state_choice = UsState::Alaska;

//     value_in_cents(Coin::Quarter(state_choice));
// }


// fn plus_one(x: Option<i32>) -> i32 {
//     match x {
//     None => 0,
//     Some(i) => i + 1,
//     }
//     }

// fn main(){
//     let five = Some(5);
//     let six = plus_one(five);
//     let none = plus_one(None);

//     println!("{:?}", six);
//     println!("{:?}", none);
// }

// fn describe_state_quarter(coin: Coin) -> Option<String> {
//     let Coin::Quarter(state) = coin else {
//     return None;
//     };
//     if state.existed_in(1900) {
//     Some(format!("{state:?} is pretty old, for America!"))
//     } else {
//     Some(format!("{state:?} is relatively new."))
//     }
// }
    
enum Response {
    Success { code: u16, message: String },
    Error(String),
}

fn main(){
    let custom_res: Response = Response::Success{code: 200, message: String::from("Success")};
    match custom_res {
        Response::Success { code, message } => println!("Server responded with {}, and message: {}", code, message),
        Response::Error(err_message) => println!("{}", err_message) 
    };
}