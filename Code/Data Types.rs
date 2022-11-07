fn main() {
  //Numeric operations 
    let _sum = 5 + 10;
    let _diff = 95.5 - 4.3;
    let _prod = 4 * 3;
    let _div = 55.5 / 11.1;
    let _floor = 2 / 3; // 0
    let _rem = 43 % 5;// modulo is 3
  
  //Boolean type
    let _rust_is_easy = true;
    let _rust_is_easy_expl: bool = false; // with explicit type annotation
    
  //Character type
   let _character = 'c';
   let _character_expl: char = 'C'; // with explicit type annotation
   let _space_ship = '🚀'; // Unicode
   
  //Compound Types
    let tup: (i32, f64, u8) = (500, 6.4, 1);
    let (_x, _y, _z) = tup;
    
    println!("The value of y is {_y}."); // 6.4
    
    let _access_first_by_index = tup.0;
    let _access_second_by_index = tup.1;
    let _access_third_by_index = tup.2;
    
  //Array Type
    let _numbers = [1, 2, 3, 4, 5];
    let _months = ["January", "February", "March", "April", "May", "June", "July", "August", "September", "October", "November", "December"];
    
    let array: [i32; 5] = [1, 2, 3, 4, 5]; // with explicit type annotation
    let _array_of_3s = [3;5]; // all 5 elements in the array are set to 3
    
    let _first_element = array[0]; // 1
    let _second_element = array[1]; // 2
}
