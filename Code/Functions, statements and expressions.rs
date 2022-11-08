fn main() {
  print_labeled_measurement(5, 'h');
}

//does not matter if the function's definition is before or after calling the main function 
//in other words, there is no need to apply forward declaration as in C/C++ and other languages
fn print_labeled_measurement(value: i32, unit_label: char) { 
    println!("The measurement is {value}{unit_label}.");
}
