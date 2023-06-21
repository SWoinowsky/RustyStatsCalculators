mod grabs;

pub fn alpha() -> f64 {
    let mut string_in = String::new();
    let mut alpha: f64 = 0.0;

    loop {
        string_in.clear();

        string_in = grabs::alpha();

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

    return alpha;
}


pub fn sample_mean() -> f64 {
    let mut string_in = String::new();
    let mut sample_mean: f64 = 0.0;

    loop {
        string_in.clear();

        string_in = grabs::xbar();

        match string_in.trim().parse::<f64>() {
            Ok( f ) => { sample_mean = f; break; },
            Err(..) => println!( "The input '{string_in}' could not be made into a float.\n" )
        }
    }

    return sample_mean;
}


pub fn variance() -> f64 {
    let mut string_in = String::new();
    let mut variance:f64 = 0.0;

    loop {
        string_in.clear();

        string_in = grabs::sigma();

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

    return variance;
}


pub fn sample_size() -> i64 {
    let mut string_in = String::new();
    let mut sample_size: i64 = 0;

    loop {
        string_in.clear();

        string_in = grabs::sample_size();

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

    return sample_size;
}