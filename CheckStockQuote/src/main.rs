mod core;
mod exchange;
mod datamodels;
mod view;
mod utility;

use std::env;
use std::process;


fn main() {
   let args: Vec<String> = env::args().collect();
   if args.len() == 1{
      core::kernel::run(1);
   }else if args.len() > 2{
      println!("wrong syntax");
      print_correct_syntax();
      process::exit(1);
   }else{   
      let flag = &args[1];
      match flag.as_str(){
         "-ns"=> core::kernel::run(0),                         
          _=> {
               println!("wrong syntax");
               print_correct_syntax();
               process::exit(1);           
             }
      }      
   }
}

fn print_correct_syntax(){
   println!("");
   println!(" correct syntax: CheckStockQuote <flag>");
   println!("    flag could be:");
   println!("      -ns [without alert beep sound]");
   println!("");
   println!("    example:");
   println!("      CheckStockQuote -ns");
   println!("      CheckStockQuote [with alert beep sound]");
}