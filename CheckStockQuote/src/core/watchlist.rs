
use std::fs;
use std::io::BufRead;

use super::super::datamodels::entity::Equity;

#[derive(Debug)]
pub struct MyWatchList{
    exchange_name:String,
    api_call:u64,
    watchlist:Vec<Equity>,
}

impl MyWatchList{
   pub fn new()->Self{
      MyWatchList{
         exchange_name: String::from(""),
         api_call:30,
         watchlist: Vec::new(),
      }
    }
    pub fn set_exchange_name(&mut self, exchange_name:String){
       self.exchange_name = exchange_name;
    }
    pub fn get_exchange_name(&self)->&str{
      &self.exchange_name
    }

    pub fn get_api_call(&self)->u64{
      self.api_call
    }
    pub fn set_api_call(&mut self, freq:u64){
       self.api_call = freq;
    }
    pub fn set_watchlist(&mut self, equity:Equity){
      self.watchlist.push(equity);
    }
    pub fn get_watchlist(&self)->&Vec<Equity>{
      &self.watchlist
    }
    pub fn get_alert_price(&self, symbol:&str)->Option<f64>{
      let equity = self.watchlist.iter().find(|equity| equity.get_symbol() == symbol).unwrap().clone();
      equity.get_threshold()
    }
}

pub fn read_watchlist_config()->MyWatchList{
    //println!("reading configuration");
    let f = fs::File::open("conf/watchlist.cfg").expect("Something went wrong while opening/reading the file '/conf/watchlist.cfg'");
    let file = std::io::BufReader::new(f);
    let mut my_watchlist = MyWatchList::new();
    for (_,line) in file.lines().enumerate(){
        let l = line.unwrap();
        if !(l.starts_with("#")) && !(l.starts_with(" ")) && !(l.len() < 1) {
            if l.contains("EXCHANGE"){
               let parts:Vec<&str> = l.split("=").collect();
               let exchange_name = parts[1].trim().to_string();
               my_watchlist.set_exchange_name(exchange_name);
            }else if l.contains("API_CALL"){
               let parts:Vec<&str> = l.split("=").collect();
               let call_freq = parts[1].trim().parse::<u64>().unwrap();
               my_watchlist.set_api_call(call_freq);
            }else{
               let parts:Vec<&str> = l.split("=").collect();
               let k = parts[0].trim().to_string();
               let eq_type:Vec<&str> = k.split("_").collect();

               let v = parts[1].trim().to_string();
               let threshold_val:Option<f64> = if v !=""{
                  Some(v.parse::<f64>().unwrap())
               }else{
                  None
               };
               let mut equity = Equity::new();
               equity.set_symob(eq_type[0].trim().to_string());
               equity.set_class(eq_type[1].trim().to_string());
               equity.set_threshold(threshold_val);
               my_watchlist.set_watchlist(equity);
            }
        }
    }
    my_watchlist
}
