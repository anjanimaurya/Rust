use reqwest;
use lazy_static::lazy_static;
use serde_json::{Value};

use log::{info, error, debug};

use super::api::Exchange;
use super::response::*;

const NASDAQ: &str = "https://api.nasdaq.com/api/quote/";
lazy_static! {
    static ref CLIENT: reqwest::Client = reqwest::Client::new();
}

#[derive(Debug, Clone)]
pub struct NASDAQ{}

impl NASDAQ{
   pub fn new()->Self{
      NASDAQ{}
   }
}

impl Exchange for NASDAQ{
   fn get_response(&self, equity_symbol: &str, equity_type: &str)->ExchangeResponse{
      match get_quote(equity_symbol, equity_type){
         Ok(json_res) => translate_response(json_res),
         Err(e) => {
                     ExchangeResponse::new("401".to_string(), e.to_string(), equity_symbol.to_string(), "".to_string(), "".to_string(),
            "".to_string(),"".to_string(),"".to_string(),"".to_string(),"".to_string()) 
                  }
      }
   }
}

pub fn get_quote(item:&str, equities_type:&str)->Result<Value, String>{
    let mut link:String = String::from(NASDAQ);
    link.push_str(item);
    link.push_str("/info?assetclass=");
    link.push_str(equities_type);

    match &mut CLIENT.get(&link).send() {
        Ok(result) => {
                        let response = result.text().unwrap();
                        let json_res:Value = serde_json::from_str(&response).unwrap();
                        Ok(json_res)
                     }
        Err(e) => {
                     error!("There was a problem calling the nasdaq url for symbol: {:?}: {:?}", item, e);
                     Err(e.to_string())
                  }
    }
}

fn translate_response(json_res:Value)->ExchangeResponse{
   let response_status_code = json_res["status"]["rCode"].as_i64().unwrap();
    match response_status_code{
        200 => {
            let res_code = response_status_code.to_string();
            let error = "".to_string();
            let eqty_symbol = json_res["data"]["symbol"].as_str().unwrap().to_string();
            let eqty_name = json_res["data"]["companyName"].as_str().unwrap().to_string();
            let eqty_current_price = json_res["data"]["primaryData"]["lastSalePrice"].as_str().unwrap().to_string().replace("$","");
            let delta = json_res["data"]["primaryData"]["deltaIndicator"].as_str().unwrap().to_string();
            let eqty_price_change = json_res["data"]["primaryData"]["netChange"].as_str().unwrap().to_string();
            
            let last_day_price = match eqty_price_change.parse::<f64>(){
               Ok(val) => {
                           let net_val = eqty_current_price.parse::<f64>().unwrap() + ((-1.0) * val);
                           format!("{:.2}",net_val).to_string()
               }
               Err(_) => eqty_current_price.clone()
            };
            
            let prev_day_price = match json_res["data"]["secondaryData"]["lastSalePrice"].as_str(){
                                    Some(val)=> {
                                                   if val.is_empty(){
                                                      last_day_price
                                                   }else{
                                                      val.to_string()
                                                   }
                                                }
                                    None => last_day_price            
            };
            
            let price_percentage_change = json_res["data"]["primaryData"]["percentageChange"].as_str().unwrap().to_string();
            let mut time_stamp = json_res["data"]["primaryData"]["lastTradeTimestamp"].as_str().unwrap().to_string();
            time_stamp = time_stamp.replace("DATA AS OF ","");
            ExchangeResponse::new(res_code, error, eqty_symbol, eqty_name, eqty_current_price, 
                                 prev_day_price, delta, eqty_price_change, price_percentage_change, 
                                 time_stamp)
        }
        _ => {
            let msg = json_res["status"]["bCodeMessage"][0]["errorMessage"].as_str().unwrap();
            ExchangeResponse::new(response_status_code.to_string(), msg.to_string(), "".to_string(), "".to_string(), "".to_string(),
            "".to_string(),"".to_string(),"".to_string(),"".to_string(),"".to_string())
        }
    }
}
