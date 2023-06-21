#[allow(unused_assignments)]
#[allow(unused_variables)]
mod cycles;
use statrs::distribution::{Normal, ContinuousCDF};


fn main() {

    let mut alpha: f64 = 0.0;
    let mut variance: f64 = 0.0;
    let mut sample_size: i64 = 0;
    let mut sample_mean: f64 = 0.0;
    let mut z_score: f64 = 0.0;

    let min: f64;
    let max: f64;

    // STEP 1: COLLECT THE ALPHA
    alpha = cycles::alpha();

    // STEP 2: COLLECT SAMPLE MEAN 
    sample_mean = cycles::sample_mean();

    // STEP 3: COLLECT THE VARIANCE
    variance = cycles::variance();

    // STEP 4: COLLECT THE SAMPLE SIZE
    sample_size = cycles::sample_size();
    
    let distribution = Normal::new(0.0, 1.0).unwrap();
    z_score = distribution.inverse_cdf( alpha / 2.0 ).abs();

    let boundary: f64 = z_score * variance.sqrt() / ( sample_size as f64 ).sqrt();

    min = sample_mean - boundary;
    max = sample_mean + boundary;
    
    let confidence: f64 = (1.0 - alpha) * 100.0;
    println!( "\n Found {confidence:.2}% confidence interval of {{{min:.3},{max:.3}}}");

}

