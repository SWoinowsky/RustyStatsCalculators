
pub fn alpha(string_in: &String) -> f64 {

    match string_in.trim().parse::<f64>() {
        Ok( f ) => {
            match ( f > 0.0 ) && ( f < 1.0 ) {
                true => f,
                false => panic!("Warning! Float value must be in [0,1]\n")
            }
        }, 
        Err(..) => panic!( "The input '{}' could not be made into a float.\n", string_in )
    }
}


pub fn float(string_in: &String) -> f64 {

    match string_in.trim().parse::<f64>() {
        Ok( f ) => f, 
        Err(..) => panic!( "The input '{}' could not be made into a float.\n", string_in )
    }
}


pub fn int(string_in: &String) -> i64 {

    match string_in.trim().parse::<i64>() {
        Ok( i ) => i, 
        Err(..) => panic!( "The input '{}' could not be made into an integer.\n", string_in )
    }
}