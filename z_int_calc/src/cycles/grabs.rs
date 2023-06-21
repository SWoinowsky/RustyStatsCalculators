use std::io;

pub fn alpha() -> String {
    let mut return_me = String::new();

    println!("\nProvide alpha for Z value:");
    match io::stdin().read_line( &mut return_me ) {
        Ok( _s ) => {},
        Err( err ) => println!( "Error: {err}" )
    };

    return return_me;
}

pub fn xbar() -> String {
    let mut return_me = String::new();

    println!( "\nProvide sample mean: ");
    match io::stdin().read_line( &mut return_me ) {
        Ok( _s ) => {},
        Err( err ) => println!( "Error: {err}" )
    };

    return return_me;
}

pub fn sigma() -> String {
    let mut return_me = String::new();

    println!("\nProvide variance value");
    match io::stdin().read_line( &mut return_me ) {
        Ok( _s ) => {},
        Err( err ) => println!( "Error: {err}" )
    }

    return return_me;
}

pub fn sample_size() -> String {
    let mut return_me = String::new();

    println!("\nProvide sample size");
    match io::stdin().read_line( &mut return_me ) {
        Ok( _s ) => {},
        Err( err ) => println!( "Error: {err}" )
    };

    return return_me;
}