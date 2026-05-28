

mod customer_experience {
   pub mod front_of_house {
   pub mod hosting {
       pub fn add_to_waitlist() {}

        fn seat_at_table() {}
    }

    mod serving {
        fn take_order() {}

        fn serve_order() {}

        fn take_payment() {}
    }
}


}

mod task {
fn do_sth() {
    println!("Doing");
}

mod dining {
    pub fn eat_at_restaurant() {
    //Absolute path
    crate::customer_experience::front_of_house::hosting::add_to_waitlist();

    //relative path
    // super::customer_experience::front_of_house::hosting::add_to_waitlist()
    super::do_sth()

}
}
}

