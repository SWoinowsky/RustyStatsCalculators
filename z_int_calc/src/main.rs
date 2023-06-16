
#[allow(unused_assignments)]
#[allow(unused_variables)]
fn main() {
    
    use statrs::distribution::{Normal, ContinuousCDF};

    use std::io;

    let mut alpha: f64 = 0.0;
    let mut variance: f64 = 0.0;
    let mut sample_size: i64 = 0;
    let mut sample_mean: f64 = 0.0;
    let mut z_score: f64 = 0.0;

    let min: f64;
    let max: f64;

    let mut string_in = String::new();

    // STEP 1: COLLECT THE ALPHA
    loop {
        string_in.clear();

        println!("\nProvide alpha for Z value:");
        match io::stdin().read_line( &mut string_in ) {
            Ok( _s ) => {},
            Err( err ) => println!( "Error: {err}" )
        };

        match string_in.trim().parse::<f64>() {
            Ok( f ) => {
                alpha = f;
                match ( alpha > 0.0 ) && ( alpha < 1.0 ) {
                    true => break,
                    false => println!("Warning! Value must be in [0,1]\n")
                };
            }, 
            Err(..) => println!( "The input '{}' could not be made into a float.\n", string_in )
        }
    }

    // STEP 2: COLLECT SAMPLE MEAN 
    loop {
        string_in.clear();

        println!( "\nProvide sample mean: ");
        match io::stdin().read_line( &mut string_in ) {
            Ok( _s ) => {},
            Err( err ) => println!( "Error: {err}" )
        }

        match string_in.trim().parse::<f64>() {
            Ok( f ) => {
                sample_mean = f; break; },
            Err(..) => println!( "The input '{string_in}' could not be made into a float.\n" )
        }
    }

    // STEP 3: COLLECT THE VARIANCE
    loop {
        string_in.clear();

        println!("\nProvide variance value");
        match io::stdin().read_line( &mut string_in ) {
            Ok( _s ) => {},
            Err( err ) => println!( "Error: {err}" )
        }

        match string_in.trim().parse::<f64>() {
            Ok( f ) => {
                variance = f;
                match variance > 0.0 {
                    true => break,
                    false => println!( "Please provide a positive value for variance.\n" )
                };
            },
            Err(..) => println!( "The input '{string_in}' could not be made into a float.\n")
        }
    }

    // STEP 4: COLLECT THE SAMPLE SIZE
    loop {
        string_in.clear();

        println!("\nProvide sample size");
        match io::stdin().read_line( &mut string_in ) {
            Ok( _s ) => {},
            Err( err ) => println!( "Error: {err}" )
        };

        match string_in.trim().parse::<i64>() {
            Ok( i ) => {
                sample_size = i;
                match sample_size > 0 {
                    true => break,
                    false => println!( "Please provide a positive value for sample size.\n" )
                };
            },
            Err(..) => println!( "The input '{string_in}' could not be made into an integer.\n")
        };
    }
    
    let distribution = Normal::new(0.0, 1.0).unwrap();
    z_score = distribution.inverse_cdf( alpha / 2.0 ).abs();

    let boundary: f64 = z_score * variance.sqrt() / ( sample_size as f64 ).sqrt();

    min = sample_mean - boundary;
    max = sample_mean + boundary;
    
    let confidence: f64 = (1.0 - alpha) * 100.0;
    println!( "\n Found {confidence:.2}% confidence interval of {{{min:.3},{max:.3}}}");

}
