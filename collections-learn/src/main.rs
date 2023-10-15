// fn main() {
//     let a = [1,2,3];
//     let mut v:Vec<i32> = Vec::new();
//     v.push(10);
//     v.push(20);
//     let mut third = v[0];
    
//     {
//         let v2 = vec![1,2,3];
//         third = v2[0];
//         let forth = v2[0];
//     }
    
    
//     print!("{} ",third);


// }
// fn main() {
    
//     let mut v:Vec<i32> = Vec::new();
//     v.push(10);
//     v.push(20);
//     v.push(30);


//     match v.get(10) {
//         Some(third)=>println!("{}",third),
//         None=>println!("nothing!!"),
//     }


// }

// fn main() {
//     let mut v = vec![1,2,3];
//     let mut num = &v[0];
//     num = &10;
//     v[0]=10;
    
//     println!("{}",num);
//     println!("{}",v[0]);
//     // match v.get(10) {
//     //     Some(third)=>println!("{}",third),
//     //     None=>println!("nothing!!"),
//     // }

// }

// fn main(){
//     let mut v = vec![1,2,3];
//     for i in v {
//         println!("{}",i);
//     }
// }