
pub fn run(){
    let name = "Nihal";
    
    println!("My name is {}", name);


    const ID: i32 = 001;

    static HASH: i32 = 123456;

    println!("ID: {}", ID);

    println!("Static : {}", HASH);

    let ( _my_name, _my_age) = ("Nihal Rai", 22);
}
