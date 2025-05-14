fn main() {
    println!("Hello from WASI!");
    
    // Demonstrate accessing environment variables
    if let Some(args) = std::env::args().nth(1) {
        println!("Argument provided: {}", args);
    } else {
        println!("No arguments provided");
    }
    
    // Current directory
    println!("Current directory: {:?}", std::env::current_dir().unwrap());
}