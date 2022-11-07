fn main() {
    let x = 5;
    
    let x = x + 1; //6
    
    {
        let x = x * 2; // 12
        println!("The value of x in the inner scope is {x}.");
    }
    
    println!("The value of x is {x}.");//6
    
    //The curly brackets '{ }' act as guards (scope) 
    //which does not let the variable 'x' 
    //affect the 'outside world'. 
    //
    //You can consider that the variable 'x' is a mask for 
    //a completely different variable ('y' in the scope)
    //which takes the latest instance of 'x' and multiplies it 
    //with 2 which results in '12' but inside the scope only.
    //
    //That explains why the scope does not let the next instance/s of the variable 'x'
    //carry on the instances of 'x'
    //from inside the scope, which guards its values.
}
