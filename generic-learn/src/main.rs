// fn main() {
//     let num_list = vec![1,2,3,4,0,10,29,5,0];
//     let largest = largest_val(num_list);
   
//     println!("Largest number is {}",largest);

//     let num_list = vec![0,19,2,14,10000,29,3,];
//     let largest = largest_val(num_list);
//     println!("Largest number is {}",largest);

//     let num_list = vec!['2','x','a','c','1'];
//     let largest = largest_val(num_list);
//     println!("Largest number is {}",largest);
// }

// fn largest_val<T: PartialOrd+Copy>(num_list:Vec<T>)->T{
//     let mut largest = num_list[0];
//     for num in num_list {
//         if num>largest{
//             largest = num;
//         }
//     }
//     largest
// }


// struct Point<T,U> {
//     x:T,
//     y:U
// }

// fn main(){
//     let p1 = Point{x:1,y:2};
//     let p2 = Point{x:'1',y:10.0};
// }

// fn main(){
//     enum Option<T>{
//         Some(T),
//         None
//     }

//     enum Result<T,E>{
//         Ok(T),
//         Err(E)
//     }
// }
// struct Point<X1, Y1> {
//     x: X1,
//     y: Y1,
// }

// impl<X1, Y1> Point<X1, Y1> {
//     fn mixup<X2, Y2>(self, other: Point<X2, Y2>) -> Point<X1, Y2> {
//         Point {
//             x: self.x,
//             y: other.y,
//         }
//     }
// }

// fn main() {
//     let p1 = Point { x: 5, y: 10.4 };
//     let p2 = Point { x: "Hello", y: 'c' };

//     let p3 = p1.mixup(p2);
    
//     println!("p3.x = {}, p3.y = {}", p3.x, p3.y);
// }

enum  Option<T> {
    Some(T),
    None,
}

fn main(){
    let integer = Option::Some(5);
    let float = Option::Some(5.0);
}