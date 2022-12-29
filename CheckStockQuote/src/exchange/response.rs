
#[derive(Debug, Clone, Eq, Ord, PartialEq, PartialOrd)]
pub struct ExchangeResponse{
   response_status:String,
   response_error:String,
   equity_symbol:String,
   equity_name:String,
   equity_price:String,
   prev_day_price:String,
   equity_delta:String,
   equity_price_change:String,
   equity_price_change_percentage:String,
   date_time:String,
}

impl ExchangeResponse{
   pub fn new(response_status:String, response_error:String, equity_symbol:String, equity_name:String, equity_price:String, prev_day_price:String, equity_delta:String,
               equity_price_change:String, equity_price_change_percentage:String, date_time:String)->Self{
                  ExchangeResponse{
                        response_status,
                        response_error,
                        equity_symbol,
                        equity_name,
                        equity_price,
                        prev_day_price,
                        equity_delta,
                        equity_price_change,
                        equity_price_change_percentage,
                        date_time
                     }     
   }
   pub fn get_response_status(&self)->&str{
      &self.response_status
   }
   pub fn get_response_error(&self)->&str{
      &self.response_error
   }
   pub fn get_symbol(&self)->&str{
      &self.equity_symbol
   }
   pub fn get_name(&self)->&str{
      &self.equity_name   
   }
   pub fn get_price(&self)->&str{
      &self.equity_price   
   }
   pub fn get_prev_day_price(&self)->&str{
      &self.prev_day_price
   }
   pub fn get_delta(&self)->&str{
      &self.equity_delta   
   }
   pub fn get_price_change(&self)->&str{
      &self.equity_price_change   
   }
   pub fn get_price_change_percentage(&self)->&str{
      &self.equity_price_change_percentage
   }
   pub fn get_date_time(&self)->&str{
      &self.date_time   
   }

}