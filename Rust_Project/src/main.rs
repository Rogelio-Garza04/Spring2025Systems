use serde::Deserialize;
use std::error::Error;
use std::fs::File;
use std::io::Write;
use std::thread::sleep;
use std::time::Duration;

// Structs for different assets
#[derive(Deserialize, Debug)]
struct Bitcoin {
    price: f64,
}

#[derive(Deserialize, Debug)]
struct Ethereum {
    price: f64,
}

#[derive(Deserialize, Debug)]
struct SP500 {
    price: f64,
}

// Pricing trait definition
trait Pricing {
    fn fetch_price(&mut self) -> Result<f64, Box<dyn Error>>;
    fn save_to_file(&self, price: f64) -> Result<(), Box<dyn Error>>;
}

// Implementations for Bitcoin
impl Bitcoin {
    fn new() -> Self {
        Bitcoin { price: 0.0 }
    }
}

impl Pricing for Bitcoin {
    fn fetch_price(&mut self) -> Result<f64, Box<dyn Error>> {
        // Using a placeholder API endpoint (replace with actual API)
        let response: serde_json::Value = ureq::get("https://api.coingecko.com/api/v3/simple/price?ids=bitcoin&vs_currencies=usd")
            .call()?
            .into_json()?;
        
        let price = response["bitcoin"]["usd"]
            .as_f64()
            .ok_or("Failed to parse Bitcoin price")?;
        
        self.price = price;
        Ok(price)
    }

    fn save_to_file(&self, price: f64) -> Result<(), Box<dyn Error>> {
        let mut file = File::create("bitcoin_price.txt")?;
        writeln!(file, "Bitcoin Price: {}", price)?;
        Ok(())
    }
}

// Implementations for Ethereum
impl Ethereum {
    fn new() -> Self {
        Ethereum { price: 0.0 }
    }
}

impl Pricing for Ethereum {
    fn fetch_price(&mut self) -> Result<f64, Box<dyn Error>> {
        // Using a placeholder API endpoint (replace with actual API)
        let response: serde_json::Value = ureq::get("https://api.coingecko.com/api/v3/simple/price?ids=ethereum&vs_currencies=usd")
            .call()?
            .into_json()?;
        
        let price = response["ethereum"]["usd"]
            .as_f64()
            .ok_or("Failed to parse Ethereum price")?;
        
        self.price = price;
        Ok(price)
    }

    fn save_to_file(&self, price: f64) -> Result<(), Box<dyn Error>> {
        let mut file = File::create("ethereum_price.txt")?;
        writeln!(file, "Ethereum Price: {}", price)?;
        Ok(())
    }
}

// Implementations for SP500
impl SP500 {
    fn new() -> Self {
        SP500 { price: 0.0 }
    }
}

impl Pricing for SP500 {
    fn fetch_price(&mut self) -> Result<f64, Box<dyn Error>> {
        // Using a placeholder API endpoint (replace with actual API)
        let response: serde_json::Value = ureq::get("https://query1.finance.yahoo.com/v8/finance/chart/^GSPC")
            .call()?
            .into_json()?;
        
        let price = response["chart"]["result"][0]["meta"]["regularMarketPrice"]
            .as_f64()
            .ok_or("Failed to parse S&P 500 price")?;
        
        self.price = price;
        Ok(price)
    }

    fn save_to_file(&self, price: f64) -> Result<(), Box<dyn Error>> {
        let mut file = File::create("sp500_price.txt")?;
        writeln!(file, "S&P 500 Price: {}", price)?;
        Ok(())
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    // Create vector of assets
    let mut assets: Vec<Box<dyn Pricing>> = vec![
        Box::new(Bitcoin::new()),
        Box::new(Ethereum::new()),
        Box::new(SP500::new()),
    ];

    // Main loop
    loop {
        for asset in assets.iter_mut() {
            match asset.fetch_price() {
                Ok(price) => {
                    println!("Fetched price: {}", price);
                    if let Err(e) = asset.save_to_file(price) {
                        eprintln!("Failed to save price: {}", e);
                    }
                }
                Err(e) => eprintln!("Failed to fetch price: {}", e),
            }
        }

        // Wait for 10 seconds
        sleep(Duration::from_secs(10));
    }
}