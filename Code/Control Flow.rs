fn main() {
    let x = 3;
    let y = 5;
    
    if x > 5 || y > 5 {
        println!("All variables are bigger than 5.");
    }
    else if x == 5 || y == 5 {
        println!("One of the variables is equal to 5.");
    }
    else {
        println!("All of the variables are less than 5.");
    }
    
    // "if" and "else/if" statements get only boolean-type statements, i.e. 
    //statements that compare two numbers or a flag
    
    // conditions can be used to compare real-life states such as likeness, hate, etc.
    // where we can combine them to predict a possible 
    // outcome based on the gathered data only
    // or that can be used in bigger data containers like truth tables
    //
    // the following example is such
    
    let i_like_football = true;
    let john_hates_football = false; // this is the same as john_loves_football
    
    if i_like_football && john_hates_football {
        println!("We may play football together."); 
    }
    else {
        println!("We may not play football together.");
    }
    
    // in that problem we see that there are only two possible 
    //outcomes but with different backgrounds
    // for example
    
    if i_like_football || john_hates_football {
        println!("We may play football together."); // but i am more likely to be eager    
    }
    else if i_like_football && !john_hates_football {
        println!("We may play football."); // but john changed his mind
    }
    
    // here both results lead to one truth and that is the output is the same
    // but that does not mean only based on the results that we know for sure
    // what could possibly have changed 
    // the usage of different logical operators makes up the same results
    
    
  // Using if in an let statement
    let condition = true;
    let number = if condition {1} else {0};
    
    println!("The value of the condition as type int is {number}.");
    
      // Repetition with Loops
    loop {
        println!("again");
    } // this loop won't stop until conditions are met 
      // and since there are no conditions, i.e. the code block contains no if/else statements to break the cycle
      // the program continues forever

  // Using if in an let statement
    let condition = true;
    let number = if condition {1} else {0};
    
    println!("The value of the condition as type int is {number}.\n");
    
    let mut counter = 0;
    let result = loop {
        counter += 1; // same as "counter = counter + 1"
        
        if counter == 10 {
            break counter * 2; 
        }
    }; // 20
    
    println!("The result after the loop is {result}.\n");
    
  // Loop labels to disambiguate between multiple loops
    let mut count = 0;
    'counting_up: loop {
        println!("count = {count}");
        
        let mut remaining = 10;
        
        loop {
            println!("remaining = {remaining}");
            if remaining == 9 {
                break;
            }
            
            if count == 2 {
                break 'counting_up;
            }
            remaining -= 1;
        }
        count  += 1;
    }
    println!("End count = {count}");
    
    
    
     
}
