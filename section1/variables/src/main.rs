//FUNCTION DEFINITION
// fn square(x: i32) -> i32{
//     x * x
// }

//RETURN DEMONSTRATION
// fn square_or_nothing(x: i32) -> i32 {
//     if x > 0 {
//         return x * x;
//     }
//     0
// }

// fn greet(s: String) {
//     println!("Hello {}", s);
// }

//BORROWING
// fn greet_borrow(s: &String){
//     println!("{}",s);
// }

//BORROWING MUTABLE
// fn greet_borrow_mut(s: &mut String) {
//     *s = format!("{}!", s);
//     println!("Hello {}", s);
// }



fn main() {
    //MAKE A VARIABALE MUTABLE
    // let mut n = 5;
    // n += 1;
    // println!("{n}");

    // SCOPING
    // let n = 5;
    // let n = {
    //     let n = 6;
    // };
    // println!("{:?}", n);

    //FUNCTION CALL
    // let x = 5;
    // println!("Square of {x} is {}", square(x));


    // let mut s = String::from("User1");
    // greet(s);
    // greet_borrow(&s);
    // greet_borrow_mut(&mut s)
    // let input = read_line();
    // println!("Entered Value: [{input}]")
}
