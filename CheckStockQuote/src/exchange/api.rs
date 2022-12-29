use super::response::*;

pub trait Exchange:Sync+Send {
   fn get_response(&self, equity_symbol: &str, equity_type: &str)->ExchangeResponse;
}
