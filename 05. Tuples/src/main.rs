fn main() {

    // Tuples
    // Group a fixed number of datatypes together 
    // in one data structure.
    let tuple = (1, 1.0, '1', true);
    println!("{}, {}, {}, {}", tuple.0, tuple.1, tuple.2, tuple.3);

    let o = 135.1;
    let h = 139.5;
    let l = 133.7;
    let c = 133.8;
    let v = 100478;
    let token = ("Solana", "SOL", (o, h, l, c, v));
 
    println!(
        "{} ohlcv: ({}, {}, {}, {}, {})",
        token.0,
        (token.2).0,
        (token.2).1,
        (token.2).2,
        (token.2).3,
        (token.2).4
    );  
   
    let (name, ticker, ohlcv) = token;
    let (sol_o, sol_h, sol_l, sol_c, sol_v) = ohlcv;
 
    println!(
        "{} {} ohlcv: ({}, {}, {}, {}, {})",
        name,
        ticker,
        sol_o,
        sol_h,
        sol_l,
        sol_c,
        sol_v
    );
 
    println!(
        "{} ohlcv: {:?}",
        token.0,
        token.2
    );
 
    println!(
        "{} ohlcv: {:#?}",
        token.0,
        token.2
    );    

}
