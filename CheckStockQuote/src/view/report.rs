
use log::{info, error, debug};
use comfy_table::*;
use comfy_table::presets::UTF8_FULL;
use comfy_table::modifiers::UTF8_ROUND_CORNERS;
use comfy_table::modifiers::UTF8_SOLID_INNER_BORDERS;
use clearscreen;

use super::super::core::kernel::WATCHLIST;
use super::super::core::kernel::ALERT_BEEP_SOUND;
use super::super::exchange::response::ExchangeResponse;
use super::super::utility::sound;


fn gen_report(res: &ExchangeResponse, alert_price:f64, table: &mut Table){
   let equity_title = res.get_name();
   let equity_symbol = res.get_symbol();
   let delta_indicator = res.get_delta();
   let net_change = res.get_price_change();
   let percentage_net_change = res.get_price_change_percentage();
   let last_update_time = res.get_date_time();
   let price_up_down = match delta_indicator {
      "up" => '↑',
      _ => '↓',
   };
   let price:f64 = res.get_price().replace("$","").parse::<f64>().unwrap();
   let prev_day_price:f64 = res.get_prev_day_price().replace("$","").parse::<f64>().unwrap();
   let mut cell_symbol = Cell::new(equity_symbol); 
   let mut cell_title = Cell::new(equity_title);
   let mut cell_price = Cell::new(price); 
   let cell_arrow = Cell::new(price_up_down);
   let cell_prev_day_price = Cell::new(prev_day_price);
   let mut cell_net_change = Cell::new(net_change);
   let mut cell_percentage_net_change = Cell::new(percentage_net_change);
   let mut cell_alert_price = Cell::new(alert_price);
   let cell_update_time= Cell::new(last_update_time);

   if price !=0.0 && price <= alert_price{
      cell_symbol = cell_symbol.bg(Color::Red).fg(Color::White).add_attributes(vec![Attribute::Bold,Attribute::SlowBlink]);
      cell_title = cell_title.bg(Color::Red).fg(Color::White).add_attributes(vec![Attribute::Bold,Attribute::SlowBlink]);
      cell_price = cell_price.bg(Color::Red).fg(Color::White).add_attributes(vec![Attribute::Bold,Attribute::SlowBlink]);
      cell_net_change = cell_net_change.bg(Color::Red).fg(Color::White).add_attributes(vec![Attribute::Bold,Attribute::SlowBlink]);
      cell_percentage_net_change = cell_percentage_net_change.bg(Color::Red).fg(Color::White).add_attributes(vec![Attribute::Bold,Attribute::SlowBlink]);
      cell_alert_price = cell_alert_price.bg(Color::Red).fg(Color::White).add_attributes(vec![Attribute::Bold,Attribute::SlowBlink]);
   }

   table.add_row(vec![
      cell_symbol,
      cell_title,
      cell_prev_day_price,
      cell_arrow,
      cell_price,
      cell_net_change,
      cell_percentage_net_change,                     
      cell_alert_price,
      cell_update_time,                       
   ]);
}

pub fn console_report(stocks_quotes_coll:Vec<ExchangeResponse>){
   if stocks_quotes_coll.len() > 0{
      clearscreen::clear().expect("failed to clear screen");
      let mut beep_flag = false; 
      let mut table = Table::new();
      table.load_preset(UTF8_FULL);
      table.apply_modifier(UTF8_ROUND_CORNERS);
      table.apply_modifier(UTF8_SOLID_INNER_BORDERS);
      table.set_content_arrangement(ContentArrangement::Dynamic);
      
      table.set_header(vec![
            Cell::new("\nSymbol     ").bg(Color::Blue).fg(Color::White),
            Cell::new("\nTitle                  ").bg(Color::Blue).fg(Color::White),
            Cell::new("Previous\nclose price").bg(Color::Blue).fg(Color::White),
            Cell::new("\nDelta ").bg(Color::Blue).fg(Color::White),
            Cell::new("Current\nPrice").bg(Color::Blue).fg(Color::White),
            Cell::new("Net\nchange").bg(Color::Blue).fg(Color::White),
            Cell::new("% Net\nchange").bg(Color::Blue).fg(Color::White),
            Cell::new("Alert\nprice").bg(Color::Blue).fg(Color::White),
            Cell::new("\nLast update time").bg(Color::Blue).fg(Color::White),
      ]); 
      for stock_etf in stocks_quotes_coll.into_iter(){
         match stock_etf.get_response_status(){
            "200" =>{
                     let equity_price = stock_etf.get_price();
                     let equity_price = equity_price.replace("$","").parse::<f64>().unwrap();
                     let stock_etf_symbol =  stock_etf.get_symbol();
                     let alert_price:f64 = match WATCHLIST.get_alert_price(stock_etf_symbol){
                                                Some(p) => p,
                                                None => 0.0,
                                          };
                     
                     if !beep_flag && equity_price !=0.0 && equity_price <= alert_price{
                        beep_flag = true;
                     }
                     gen_report(&stock_etf, alert_price, &mut table);
            }
            _ => {
                  table.add_row(vec!["data not available"]);
                  error!("view:: report: data not available, response: {:?}", stock_etf);
            }
         }
      }
      println!("{}",table);
      println!("feed from {:?}, by Anjani Maurya",WATCHLIST.get_exchange_name());
      unsafe{
         if ALERT_BEEP_SOUND && beep_flag{
            sound::play_beep();
         }
      }
   }else{
      println!("No results found from Exchange {:?}", WATCHLIST.get_exchange_name());   
   }
   println!("");
}