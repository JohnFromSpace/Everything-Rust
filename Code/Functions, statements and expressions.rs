fn main() {
    print_labeled_measurement(5, 'h');
  
    let _x = 1; // this is an expression which returns value '1' while 'let' is a statement
    // used to declare a new variable '_x'
    //
    // you cannot have statements like "x = y = 1" in Rust
    // or statements like
    // let x = (let y = 5);
    
    let y = {
        let x = 3;
        x + 1
    };
    //this can be represented as a scope for 'y' variable
    //
    // { 
    //   let x = 3;
    //   let y = x + 1;
    // }
    //
    // in that way the value of 'x' is "active" only inside the scope for 'y' and cannot be used outside of it 
    
    println!("The value of y is {y}."); // 4, where "x + 1" is an expression of variable 'y'
    
    let x = one(); // 1
    println!("The value of x is {x}."); 
    
    let x = plus_one(5); // 6
    println!("The value of x is {x}.");
}

//does not matter if the function's definition is before or after calling the main function 
//in other words, there is no need to apply forward declaration as in C/C++ and other languages
fn print_labeled_measurement(value: i32, unit_label: char) { 
    println!("The measurement is {value}{unit_label}.");
}

fn one() -> i32 {
    1
}

fn plus_one(x: i32) -> i32 {
    x + 1
}
