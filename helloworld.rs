extern crate rary;
extern crate wss;

fn main() {
    
    rary::public_function();

    // Error! `private_function` is private
    //rary::private_function();

    rary::indirect_access();

    // Statements here are executed when the compiled binary is called

    // Print text to the console
    println!("Hello World!");
}
