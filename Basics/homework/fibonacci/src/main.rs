fn run_fibonnaci(num: i32) -> i32 {
   let mut prev = 0;
   let mut current = 1;

   for _ in 2..=num {
        let next = prev + current;
        prev = current;
        current = next;
   }

   current
}


fn main() {

    let result = run_fibonnaci(7);

    println!("{}",result);
}
