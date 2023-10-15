// mod front_of_house {
//     mod hosting {
//         fn add_to_waitlist() {}

//         fn seat_at_table() {}
//     }

//     mod serving {
//         fn take_order() {}

//         fn serve_order() {}

//         fn take_payment() {}
//     }
// }

// mod front_of_house {
//     pub mod hosting {
//         pub fn add_to_waitlist() {}
//     }
// }

// pub fn eat_at_restaurant(){
//     crate::front_of_house::hosting::add_to_waitlist();
//     front_of_house::hosting::add_to_waitlist();
// }


// fn serve_order(){}

// mod back_of_house {
//     fn fix_incorrect_order(){
//         cook_order();
//         // relative path using super which represent crate
//         super::serve_order()
//     }

//     fn cook_order(){}
// }

// use back_of_house::BreakFast;



// mod back_of_house {
//     pub struct  BreakFast{
//         pub toast:String,
//         seasonal_fruit:String,
//     }

//     impl BreakFast {
//         pub fn summer(toast:&str)->BreakFast{
//             BreakFast { 
//                 toast:String::from(toast),
//                 seasonal_fruit: String::from("peach") 
//             }
//         }
//     }
// }

// pub fn eat_at_restaurant(){
//     let mut meal = back_of_house::BreakFast::summer("Jitul");
//     let meal1 = back_of_house::BreakFast{
//         toast:String::from("testing"),
//         seasonal_fruit:String::from("testing"),
//     };
//     meal.toast = String::from("apple");
// }


// mod back_of_house{
//     pub enum Appetized {
//         Soup,
//         Salad,
//     }
// }

// pub fn eat_at_restaurant(){
//     let order1 = back_of_house::Appetized::Soup;
//     let order1 = back_of_house::Appetized::Salad;
// }

// mod front_of_house{
//     pub mod hosting {
//         pub fn add_to_waitlist(){}
//     }
// }
// // use crate::front_of_house::hosting;
// use self::front_of_house::hosting;
// // use self::front_of_house::hosting::add_to_waitlist;
// pub fn eat_at_restaurant(){
//     hosting::add_to_waitlist();
// }


// use std::fmt;
// use std::io;

// fn fmt_test()->fmt::Result{

// }
// fn fio_test()->io::Result<>{
    
// }

// use std::fmt::Result;
// use std::io::Result as IOResult;

// mod front_of_house{
//     pub mod hosting {
//         pub fn add_to_waitlist(){}
//     }
// }
// pub use crate::front_of_house::hosting;
// pub fn eat_at_restaurant(){
//     hosting::add_to_waitlist();
// }

// use std::io;
// use std::io::Write;

// use std::io::{self,Write};

// use std::*;





mod front_of_house;
pub use crate::front_of_house::hosting;
pub fn eat_at_restaurant(){
    hosting::again::add_to_waitlist();
}
