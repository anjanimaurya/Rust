use super::api::Exchange;
use super::response::*;
use lazy_static::lazy_static;
use log::{info, error, debug};
use serde_json::{Value};

const NSE_URL: &str = "https://www.nseindia.com/api/quote-equity";
lazy_static! {
    static ref CLIENT: reqwest::Client = reqwest::Client::new();
}

#[derive(Debug, Clone)]
pub struct NSE{}

impl NSE{
   pub fn new()->Self{
      NSE{}
   }
}

impl Exchange for NSE{
   fn get_response(&self, equity_symbol: &str, equity_type: &str)->ExchangeResponse{
      match get_quote(equity_symbol){
         Ok(json_res) => translate_response(json_res),
         Err(e) => {
                     ExchangeResponse::new("401".to_string(), e.to_string(), equity_symbol.to_string(), "".to_string(), "".to_string(),
                        "".to_string(),"".to_string(),"".to_string(),"".to_string(),"".to_string()) 
                  }
      }
   }
}

pub fn get_quote(item:&str)->Result<Value, String>{
   let mut link:String = String::from(NSE_URL);
   link.push_str("?symbol=");
   link.push_str(item);
   let nse_base_url_response = match CLIENT.get("https://www.nseindia.com").send(){
                               Ok(result) => result,
                               Err(e) => {
                                             error!("something went wrong,  {:?}",e);
                                             return Err(e.to_string());
                               }   
   };
   let cookies = nse_base_url_response.cookies();
   let mut nse_cookes=String::from("");
   let mut request_headers = reqwest::header::HeaderMap::new();
   for c in cookies{
     nse_cookes.push_str(c.name());
     nse_cookes.push_str("=");
     nse_cookes.push_str(c.value());
     nse_cookes.push_str(";");
   }
   request_headers.insert(
      reqwest::header::COOKIE,
      reqwest::header::HeaderValue::from_str(&nse_cookes.clone()).unwrap()
   );
    
   let mut url_response = CLIENT.get(&link).headers(request_headers).send();

   match &mut url_response {
        Ok(result) => {
                        let response = result.text().unwrap();
                        let json_res:Value = serde_json::from_str(&response).unwrap();
                        Ok(json_res)           
                  }
        Err(e) => {
                     error!("There was a problem calling the NSE India url: {:?}", e);
                     Err(e.to_string())
                  }
    }
}

fn translate_response(json_res:Value)->ExchangeResponse{
   //println!("NSE India response : {:?}",json_res);
   match json_res["info"]["symbol"].as_str(){
      Some(_) =>{
         let eqty_symbol = json_res["info"]["symbol"].as_str().unwrap().to_string();
         let eqty_name = json_res["info"]["companyName"].as_str().unwrap().to_string();
         let eqty_current_price = json_res["priceInfo"]["lastPrice"].as_f64().unwrap().to_string();
         let prev_day_price = json_res["priceInfo"]["previousClose"].as_f64().unwrap().to_string();
         let eqty_price_change = format!("{:.2}",json_res["priceInfo"]["change"].as_f64().unwrap());
         let delta_indicator = if json_res["priceInfo"]["change"].as_f64().unwrap() >= 0.0{
                                    "up".to_string()
                                 }else{
                                    "down".to_string()   
                                 };
         let price_percentage_change = format!("{:.2}",json_res["priceInfo"]["pChange"].as_f64().unwrap());
         let time_stamp = json_res["metadata"]["lastUpdateTime"].as_str().unwrap().to_string();
         ExchangeResponse::new("200".to_string(), "".to_string(), eqty_symbol, eqty_name, 
                              eqty_current_price, prev_day_price, delta_indicator, eqty_price_change,
                              price_percentage_change, time_stamp)
      }
      None => {
               ExchangeResponse::new("400".to_string(), "Bad request".to_string(), "".to_string(), "".to_string(), 
                                    "".to_string(), "".to_string(), "".to_string(), "".to_string(),
                                    "".to_string(), "".to_string())
      }
   }
}