use std::{thread};
use std::time::Duration;
use lazy_static::lazy_static;
use log::{info, error, debug};
use rayon;
use std::sync::mpsc::{channel};
use std::sync::{Arc};
use std::time::{Instant};

use super::super::exchange::nasdaq::NASDAQ;
use super::super::exchange::nse::NSE;
use super::super::exchange::api::Exchange;
use super::super::exchange::response::ExchangeResponse;
use super::super::datamodels::entity::Equity;
use super::super::view::report;
use super::watchlist;
use super::logger;

pub static mut ALERT_BEEP_SOUND:bool = true;

lazy_static!{
   pub static ref WATCHLIST:watchlist::MyWatchList = watchlist::read_watchlist_config();
   pub static ref WL:Vec<Equity> = WATCHLIST.get_watchlist().to_vec();
   pub static ref EXCHANGE_API:Arc<dyn Exchange> = match WATCHLIST.get_exchange_name(){
      "NASDAQ" => Arc::new(NASDAQ::new()),
      "NSE" => Arc::new(NSE::new()),
      _ => {
            error!("EXCHANGE not mentioned in the config");
            panic!("EXCHANGE not mentioned in the config")
         }
   };
   pub static ref API_CALL_DELAY_TIME:u64 = WATCHLIST.get_api_call();
   pub static ref THREAD_POOL_SIZE:usize = 10;
}

pub fn run(cmd_beep_fag:i8){
   if cmd_beep_fag == 0{
      unsafe{
         ALERT_BEEP_SOUND = false;
      }
   }
   logger::init_log("INFO");
   info!("CheckStockQuote started .....");
   
   loop {
         //let start_time = Instant::now();
         
         //let stocks_quotes_coll = get_quotes();
         let stocks_quotes_coll = get_quotes_with_threads();
         report::console_report(stocks_quotes_coll);

         //let time_taken = start_time.elapsed();   
         //println!("");
         //println!("time taken : {:?}", time_taken);
      
         thread::sleep(Duration::from_millis(*API_CALL_DELAY_TIME));
   }      
}

fn get_quotes()->Vec<ExchangeResponse>{
   let mut stocks_quotes_coll:Vec<ExchangeResponse> = Vec::new();
   for eq in WL.iter(){
      let equity:Equity = eq.clone();
      let symbol:&str = equity.get_symbol();
      let class:&str = equity.get_class();
         
      let result = EXCHANGE_API.get_response(symbol, class);
      stocks_quotes_coll.push(result);
   }
   stocks_quotes_coll
}

fn get_quotes_with_threads()->Vec<ExchangeResponse>{
   let pool = rayon::ThreadPoolBuilder::new()
    .num_threads(*THREAD_POOL_SIZE)
    .build()
    .unwrap();
    
   let (tx, rx) = channel();
   for eq in WL.iter() {
      let tx = tx.clone();  
      let api = Arc::clone(&EXCHANGE_API); 
      pool.spawn(move || {
         tx.send(
                  {
                     let equity:Equity = eq.clone();
                     let symbol:&str = equity.get_symbol();
                     let class:&str = equity.get_class();
                     let res = api.get_response(symbol, class);
                     res
                  }
            );
      });
   }
   //close all senders
   drop(tx);
   let mut stocks_quotes_coll:Vec<ExchangeResponse> = Vec::new();
   for received in rx{
      stocks_quotes_coll.push(received);
   }
   //sorting collection by symbol of stocks/etf
   stocks_quotes_coll.sort_by(|a, b| a.get_symbol().cmp(&b.get_symbol()));
   stocks_quotes_coll
}
