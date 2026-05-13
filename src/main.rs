#[allow(dead_code)]

fn main() {
    println!("Hello, world!");

    // create a function that takes a function as an argument and executes it
    fn execute<F>(f: F)
    where
        F: Fn(),
    {
        f();
    }
    
}

fn example_function() {
    println!("This is an example function.");
}