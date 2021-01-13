/*
String conversion : https://stackoverflow.com/questions/24990520/how-do-i-convert-from-an-integer-to-a-string
() : Unit type i.e void type/ implicit return in void

? Search what is borrow checker

env::set_var("RUST_BACKTRACE", "1") // backtrace method

rust string : https://www.tutorialspoint.com/rust/rust_string.htm


*/

fn helper(name : String, age: i32)-> u8 {
    println!("Hello world! My name is {} and my age is {}", name, age);
    1
}

// Diverging functions are the end of the code, after this no instructions are executed
fn diverging_function(name : String, age: i32) -> ! {
    panic!("Hello world! My name is {} and my age is {}", name, age);
}

fn main(){
    let _a = helper("Nihal Rai".to_string(), 22);
    let format_string = format!("{} {}", "Hi", "Nihal");

    println!("{}", format_string);

    let _float_var32:f32 = 5.2;
    let _float_var64:f64 = 5.2;
    let _integer_var:i32 = 5000;
    println!("{}  {}   {}", _float_var32, _float_var64, _integer_var);

    // error no implicit type casting
    //explicit is done using as
    let integer_var = _float_var32 as i32;
    println!("Typecasting variable : {}", integer_var);

    let is_true:bool = true;
    println!("Boolean variable : {}", is_true);

    let alphabet:char = 'N';
    println!("Character variable : {}", alphabet);

    //immutable variables
    let check_immutability;

    check_immutability = 100;
    println!("Immutable variable : {}", check_immutability);
    // mutable variables are declared using let mut

    let mut mutable_var = 100;
    print!("Mutable variable : {} and ", mutable_var);

    mutable_var = 200;
    println!("it is mutated to : {}", mutable_var);

    let string_var:&str = "Nihal Rai";
    println!("Str not String : {}", string_var);

    println!("Hi");
    println!("{:?}", _a);

    // In general, the `{}` will be automatically replaced with any
    // arguments. These will be stringified.
    println!("{} days", 31);

    // Without a suffix, 31 becomes an i32. You can change what type 31 is
    // by providing a suffix. The number 31i64 for example has the type i64.

    // There are various optional patterns this works with. Positional
    // arguments can be used.
    println!("{0}, this is {1}. {1}, this is {0}", "Alice", "Bob");

    // As can named arguments.
    println!("{subject} {verb} {object}",
             object="the lazy dog",
             subject="the quick brown fox",
             verb="jumps over");

    // Special formatting can be specified after a `:`.
    println!("{} of {:b} people know binary, the other half doesn't", 1, 2);

    // You can right-align text with a specified width. This will output
    // "     1". 5 white spaces and a "1".
    println!("{number:>width$}", number=1, width=6);

    // You can pad numbers with extra zeroes. This will output "000001".
    println!("{number:>0width$}", number=1, width=6);

    //diverging_function("Nihal Rai".to_string(), 22);
}