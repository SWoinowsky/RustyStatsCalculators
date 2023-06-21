#[allow(unused_assignments)]
#[allow(unused_variables)]
use statrs::distribution::{Normal, ContinuousCDF};
use std::env;
mod matches;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.iter().any(|s: &String| s == "--help")  {
        println!( "\n\tUse command with arguments in order of alpha mu sigma sample_size\n" );
        std::process::exit(0x0100);
    } else if args.len() != 5  {
        println!( "\nArguments must be passed in this order: \n \t alpha mu sigma sample_size");
        println!( "\tUse the --help tag for instructions.\n");
        std::process::exit(0x0100);
    }

    let alpha_string = &args[1];
    let alpha: f64 = matches::alpha(alpha_string);

    let mean_string = &args[2];
    let sample_mean: f64 = matches::float(mean_string);

    let variance_string = &args[3];
    let variance: f64 = matches::float(variance_string);


    let size_string = &args[4];
    let sample_size: i64 = matches::int(size_string);

    let distribution = Normal::new(0.0, 1.0).unwrap();
    let z_score: f64 = distribution.inverse_cdf( alpha / 2.0 ).abs();

    let boundary: f64 = z_score * variance.sqrt() / ( sample_size as f64 ).sqrt();

    let min: f64 = sample_mean - boundary;
    let max: f64 = sample_mean + boundary;
    
    let confidence: f64 = (1.0 - alpha) * 100.0;
    println!( "\n Found {confidence:.2}% confidence interval of {{{min:.3},{max:.3}}}");

}

