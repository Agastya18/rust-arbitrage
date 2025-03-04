use reqwest::blocking::get;
use serde::Deserialize;
use std::thread;
use std::time::Duration;

#[derive(Deserialize)]
struct Bin {

    symbol:String,
    price:String,

}
#[derive(Deserialize)]
struct  Jup{

    data:Data,

}
#[derive(Deserialize)]
struct  Data{
    So11111111111111111111111111111111111111112:SOLID,
}
#[derive(Deserialize)]
struct SOLID{
    id:String,
    price:String,

}
fn main() {


 loop {
    let url= format!("https://api.binance.com/api/v3/ticker/price?symbol=SOLUSDT");

    const jup:&str="https://api.jup.ag/price/v2?ids=So11111111111111111111111111111111111111112";
    let resp_b:Bin= get(url).expect("faled read").json().expect("parse error");
    let resp_j:Jup= get(jup).expect("errroe resp").json().expect("parse ereror");
    println!("{} {}",resp_b.symbol,resp_b.price);
     println!("{} {}",resp_j.data.So11111111111111111111111111111111111111112.id, resp_j.data.So11111111111111111111111111111111111111112.price);
   
   //   let binance_price:f32= resp_b.price.parse();
   //   let solana_price:f32=resp_j.data.So11111111111111111111111111111111111111112.price.parse();
     check_arbitrage(resp_b.price,resp_j.data.So11111111111111111111111111111111111111112.price);
     thread::sleep(Duration::from_secs(10));
    
 }


}

 fn check_arbitrage(bp:String,jp:String){

    println!("inside function:");

    println!("{} {}",bp,jp);

    let binance_price:f32=bp.parse().unwrap();
    let jup_price:f32=jp.parse().unwrap();

    let binance_price_with_fee= binance_price;
    let jup_price_with_fee= jup_price;

    if binance_price_with_fee>jup_price_with_fee {
        let profit = binance_price_with_fee-jup_price_with_fee;
        println!("Arbitrage Opportunity! Buy on jup and sell on binance at {}",profit);

    }else if binance_price_with_fee<jup_price_with_fee {
        let profit = jup_price_with_fee-binance_price_with_fee;
        println!("Arbitrage Opportunity! Buy on binance and sell on jup at {}",profit);

    }else{
        println!("No arbitrage opportunity found.");
    }
   


    


 }
