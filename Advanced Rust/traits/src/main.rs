// Classes in Rust are called  Structs.

// Fields of a class are defined using struct
struct Candle {
    ticker  : String,
    open    : i64,
    high    : i64,
    low     : i64,
    close   : i64,
    zeros   : u32,

    year    : u32,
    month   : u32,
    day     : u32
}

trait Dateable {
    fn get_date(&self) -> String;
}

impl Dateable for Candle {
    fn get_date(&self) -> String {
        format!("{}-{}-{}", self.year, self.month, self.day)
    }
}

// Methods of a class are defined using impl
impl Candle {
    // &self - reference to `this` (javascript) or  self (python)
    fn convert_value_to_f64(&self, value: i64) -> f64 {
        value as f64 / (10_i64.pow(self.zeros)) as f64
    }

    fn get_delta(&self) -> f64 {
        self.convert_value_to_f64(self.close - self.open)
    }

    fn get_info(&self) -> String {
        format!(
            "{} ({}, {}, {}, {})",
            self.ticker,
            self.convert_value_to_f64(self.open),
            self.convert_value_to_f64(self.high),
            self.convert_value_to_f64(self.low),
            self.convert_value_to_f64(self.close),
        )
    }
}

fn main() {

    let today: Candle = Candle {
        ticker: "BTCUSD".to_string(),
        open: 23208,
        high: 23295,
        low: 21093,
        close: 21147,
        zeros: 2,

        year: 2022,
        month: 6,
        day: 3
    };

    println!("{}", today.get_delta());
    println!("{}", today.get_info());
    println!("{}", today.get_date());

}
