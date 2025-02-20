use rand::prelude::*;



fn main() {
    let mut total : usize = 0;
    let mut neg : usize = 0;
    let mut rng =  rand::rng();

    loop {
        let a = 2.0*rng.random::<f64>() - 1.0;  // I want randoms betwee -1 and 1, the RNG gives me randoms from 0 to 1
        let b = 2.0*rng.random::<f64>() - 1.0;
        let c = 2.0*rng.random::<f64>() - 1.0;
        let d = 2.0*rng.random::<f64>() - 1.0;

        total += 1;
        if (a-d).powi(2) + 4.0*b*c < 0.0 {  // oh now I remember why I get annoyed at rust for numerical stuff
            neg += 1;
        }


        if total % 100000 == 0 {
            
            println!("{}:  {}",total,(neg as f64)/(total as f64));
        }
    }
}
