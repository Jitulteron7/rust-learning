// use std::any;


// struct User <T>{
//     active: bool,
//     username: String,
//     email: String,
//     sign_in_acount: u64,
//     test:T

// }

// fn main() {

//     let user = User {
//         active: false,
//         username: String::from("Jitul"),
//         email: String::from("jjteron@gmail.com"),
//         sign_in_acount: 1,
//         test:"testin"

//     };
//     // let user1 = User {
//     //     active:user.active,
//     //     username: String::from("Yoyo"),
//     //     email: user.email.clone(),
//     //     sign_in_acount: 1,
//     //     test:&user // & : is added to resolve error of owership and borrow
//     // };

//      let user1 = User {
//         active:user.active,
//         ..user // 
//     };

//     // println!("Jitul {}", user.username); // ..user for string cause string is created in heap
//     println!("Jitul1 {}", user1.username);

// }

// struct User {
//     active: bool,
//     username: String,
    

// }
// fn build_user(username: String) -> User {
//     User {
//         username, 
//         active: true,
//     }
// }

// fn main() {
//     let mut  user = build_user(String::from("Jitul"));

//     println!("{}",user.username);

//     user.username=String::from("Kankan");

//     println!("{}",user.username);
    
// }

use std::io::Read;

// struct Color(i32,i32,String);
// fn main(){
//     let color = Color(15,10,String::from("red"));
//     print!("{}",color.0);
// }
#[derive(Debug)]
struct User{
    name:i32
}

// fn main (){
//     let user = User{
//         name:1
//     };
    // let x=();
    // println!("{:?}",user);
    // println!("{:?}",x);
    // dbg!(&user);
// }
// #[derive(Debug)]
// struct  User {
//     name:String,
//     active:bool
// }
// fn main (){
//     let user = User{
//         name:String::from("Jitul"),
//         active:true
//     };
//     print!("{:#?}",user);
// }


// struct Rectangle{
//     width: i32,
//     height: i32
// }

// fn main(){
//     let rect = Rectangle{
//         width:100,
//         height:50
//     };

//     print!("Area of rectangle {}",area(&rect));
// }

// fn area(rect:&Rectangle)->i32{

//     return rect.height*rect.width;
// }

// struct Rectangle{
//     width: i32,
//     height: i32,
    
// }

// impl  Rectangle {
//      fn area(&self) -> i32{
//          self.height*self.width
//     }
//     fn width(&self) -> i32{
//         self.width
//     }
// }


// fn main(){
//     let rect = Rectangle{
//         width:100,
//         height:50
//     };

//     println!("Area of rectangle {}",rect.area());
//     println!("Area of rectangle {}",rect.width());
// }


#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }
    
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }

    // assiciated method
    fn square(size: u32) -> Rectangle {
        Rectangle {
            width: size,
            height: size,
        }
    }
    fn hello(){
        println!("Hello");
    }
}

fn main() {
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };

    println!(
        "The area of the rectangle is {} square pixels.",
        rect1.area()
    );

    let sq = Rectangle::square(3);
    Rectangle::hello()
}