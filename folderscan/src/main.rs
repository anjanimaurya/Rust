
extern crate chrono;
use chrono::offset::{Local};
use chrono::{DateTime,NaiveDate, NaiveTime, NaiveDateTime};
use std::env;
use std::{fs};
use std::time::{SystemTime,Instant};
use std::process;

use rayon::prelude::*;

static mut FOUND_FILES_COUNTER:u64 = 0;

#[derive(Clone)]
enum Operator{
   GT,
   LT,
   GTET,
   LTET,
   EQUALS,
   NONE,
}

impl Operator{
   fn parse<'a>(opt:String)->Result<Operator, &'a str>{
      match opt.as_str() {
         ">" => Ok(Operator::GT),
         "<" => Ok(Operator::LT),         
         _ => {
               Err(" wrong parametre of 'logical operator' for flag '-fwd' ")
              }
      }
   }
}

trait Validation{
   fn validate(&self, item:&Item)->bool;
}

struct FileExtValidation{
   ext:String,
}

struct DateTimeValidation{
   logical_optr:Operator,
   date_time:NaiveDateTime,
}

struct FileNameValidation{
   filename:String,
}

impl Validation for FileExtValidation{
   fn validate(&self, item:&Item)->bool{
      if "ALL"== &self.ext || item.name.ends_with(&self.ext){
         true
      }else{
         false
      }
   }
}

impl Validation for DateTimeValidation{
   fn validate(&self, item:&Item)->bool{
      let optr = &self.logical_optr;      
      let date_time = &self.date_time;
      let date_time_millisec = date_time.timestamp_millis();
      let items_datetime:DateTime<Local> = item.dom.into();
      let items_datetime_naive =  items_datetime.naive_local();
      let dt = items_datetime_naive.date();
      let time = items_datetime_naive.time();
      let items_naive_date_time = NaiveDateTime::new(dt,time);
      let items_mod_millisec = items_naive_date_time.timestamp_millis();

      let mut validation_flag = false;
      match optr{
         Operator::GT =>{                              
                           if items_mod_millisec > date_time_millisec{
                              validation_flag = true;
                           }
                        }
         Operator::LT =>{
                           if items_mod_millisec < date_time_millisec{
                              validation_flag = true;
                           }
                        }
         _ =>{
               validation_flag = false;
            } 
      }
      validation_flag
   }
}

impl Validation for FileNameValidation{
   fn validate(&self, item:&Item)->bool{
      let mut validation_flag = false;
      let mut fname:String = String::from(&item.name);
      fname.make_ascii_uppercase();
      let mut find_file_name = String::from(&self.filename);
      find_file_name.make_ascii_uppercase();
      let folder = item.file_or_folder;
      if fname.ends_with(&find_file_name) && !folder{
         validation_flag = true;
      }
      validation_flag
   }
}

struct ValidationColl{
   coll:Vec<Box<dyn Validation>>,   
}

impl ValidationColl {
   fn new()->Self{
      ValidationColl{
         coll: Vec::new(),
      }
   }
   fn add_validation<T: Validation + 'static>(&mut self, validation:T)->&mut Self{
      self.coll.push(Box::new(validation));
      self
   }
}

#[derive(Clone)]
struct Item{
   name:String,               //name of the file/folder
   dom:SystemTime,            //date of modification
   file_or_folder:bool,       //is this item a file or a directory, 1 for file, 0 for directory   
}

impl Item{
   fn new(name:String, dom:SystemTime, file_or_folder:bool)->Item{
      Item{
         name,
         dom,
         file_or_folder,
      }      
   }
   fn apply_validation(&self, validation_coll: &ValidationColl)->bool{
      let mut validation_flag = true;
      let coll = &validation_coll.coll;
      let item = &*self;
      for v in coll{
         if v.validate(item)==false{
            validation_flag = false;
            break;
         }
      }
      validation_flag
   }
}

trait FlagType{
   fn execute(&self);
   fn parse_args<'a>(self)->Result<Box<dyn FlagType>, &'a str>;
}

#[derive(Clone)]
struct FF{
   args: Vec<String>,
   path: String,
   filename: String,
}

#[derive(Clone)]
struct FWD{
   args: Vec<String>,
   path: String,
   operator: Operator,
   datetime: NaiveDateTime,
   file_ext: String,
}

impl FlagType for FF{
   fn execute(&self){
      let target_folder = &self.path;
      let filename = &self.filename;      
      let filename_validation = FileNameValidation{filename:filename.to_string()};      
      let mut validation_coll = ValidationColl::new();         
      validation_coll.add_validation(filename_validation);      
      folder_scan(target_folder.to_string(),&validation_coll);      
      println!("");
   }
   fn parse_args<'a>(mut self)->Result<Box<dyn FlagType>, &'a str>{
      if self.args.len() !=2{
         Err(" incorrect syntax for flag -ff")
      }else{
         self.path = self.args[0].clone();
         self.filename = self.args[1].clone();
         let cpy = (self).clone();
         Ok(Box::new(cpy))
      }
   }
}

impl FlagType for FWD{
   fn execute(&self){
      let target_folder = &self.path;
      let operator = (&self.operator).clone();      
      let file_ext = &self.file_ext;
      let input_date_time = self.datetime;

      let ext_validation = FileExtValidation{ext:file_ext.to_string()};
      
      let date_time_validation = DateTimeValidation{logical_optr:operator, date_time:input_date_time}; 

      let mut validation_coll = ValidationColl::new();
         
      validation_coll.add_validation(ext_validation);
      validation_coll.add_validation(date_time_validation);
      
      folder_scan(target_folder.to_string(),&validation_coll);
      println!("");
   }

   fn parse_args<'a>(mut self)->Result<Box<dyn FlagType>, &'a str>{
      if self.args.len() !=4{
         Err(" incorrect syntax for flag -fwd")
      }else{
         self.path = self.args[0].clone(); 
         let optr = self.args[1].clone();
         
         let mut parsed_flag = true;
         let mut err_msg = "";
         match Operator::parse(optr.to_string()){
            Ok(optr)=> {
                     self.operator = optr;                     
                     self.file_ext = self.args[3].clone();
            }
            Err(why)=> {
               parsed_flag = false;
               err_msg = why;
            }
         }
         
         let date_time = self.args[2].clone();
         
         match parse_datetime(date_time.to_string()){
            Ok(parsed_datetime) => {
                        self.datetime = parsed_datetime;
                     }
            Err(why) => {
               parsed_flag = false;
               err_msg = why;
            }
         }
         if parsed_flag{
            let cpy = self.clone();
            Ok(Box::new(cpy))
         }else{
            Err(err_msg)
         }
      }
   }
}

fn parse_datetime<'a>(datetime:String)->Result<NaiveDateTime, &'a str>{
   let date_time = NaiveDateTime::parse_from_str(&datetime, "%m-%d-%YT%H:%M:%S");
   match date_time{
                     Ok(result) =>Ok(result),         
                     _=> Err(" incorrect date/time format"),                           
                  }
}


fn folder_scan(folder_path:String, validation_coll:&ValidationColl){
   let input_path = fs::read_dir(&folder_path);
   match input_path{
      Ok(path)=>{
                  path.for_each(|entry| {
                  //for entry in path {
                  let e = entry.unwrap();
                  let path = e.path();
                  let mtdt = e.metadata().unwrap();
                  let items_mod = mtdt.modified().unwrap();

                  let items_datetime: DateTime<Local> = items_mod.into();
                  let items_datetime_naive = items_datetime.naive_local();

                  let file_name = path.to_str().unwrap();
                  let mut file_or_folder = false;

                  if mtdt.is_dir() {
                      file_or_folder = true;
                      let child_folder = path.to_str().unwrap();
                      folder_scan(child_folder.to_string(), &validation_coll);
                  }
                  let item = Item::new(file_name.to_string(), items_mod, file_or_folder);
                  let validation = item.apply_validation(validation_coll);
                  if validation {
                      unsafe {
                          FOUND_FILES_COUNTER = FOUND_FILES_COUNTER + 1;
                      }
                      println!("{:?}    {:?}", file_name, items_datetime_naive);
                  }
              }
              );
                }
      Err(why) => {
                     //println!("{}, {}",folder_path, why);
                  }
   }
}

fn read_command_line_args<'a>()->Result<Box<dyn FlagType>, &'a str>{
   let args: Vec<String> = env::args().collect();
   if args.len() < 2{
      Err(" missing flag and arguments")
   }else{   
      let flag = &args[1];
      match flag.as_str(){
         "-ff"=> {
                  let ff_args = args[2..].to_vec();
                  let ff = FF{args:ff_args, path:String::from(""), filename: String::from("")};                  
                  ff.parse_args()                         
                 }
         "-fwd"=>{
                  let fwd_args = args[2..].to_vec();
                  let d = NaiveDate::from_ymd(1999, 1, 1);                  //dummy date
                  let t = NaiveTime::from_hms_milli(12, 00, 00, 789);       //dummy time
                  let fwd = FWD{
                                 args: fwd_args,
                                 path: String::from(""),
                                 operator: Operator::NONE,
                                 datetime: NaiveDateTime::new(d, t),
                                 file_ext: String::from(""),
                              };
                  fwd.parse_args() 
                 }
         _=> {
               Err(" wrong flag")           
             }
      }      
   }   
}

fn print_correct_syntax(){
   println!("");
   println!(" correct syntax: folderscan <flag> <arguments>");
   println!("    flag and arguments could be:");
   println!("      -ff <path> <filename with ext>");
   println!("      -fwd <path> \">\" <mm-dd-yyyyTHH:mm:ss> <file extension or ALL>");
   println!("");
   println!("    example:");
   println!("      folderscan -ff c:\\temp hello.txt");
   println!("      folderscan -ff c:\\temp .txt   [for searching all files with extension of 'txt' in the dir c:\\temp]");
   println!("      folderscan -fwd c:\\temp \"> or <\" 08-22-2020T16:40:00 txt");
   println!("      folderscan -fwd c:\\temp \"> or <\" 08-22-2020T16:40:00 ALL");
   println!("");
}

fn main() {
   let start = Instant::now();   
   let args = read_command_line_args();
   match args{
      Ok(cmd_args) => {
                        cmd_args.execute();
                        unsafe{
                           println!("total files found: {:?}",FOUND_FILES_COUNTER);
                        }
                        let duration = start.elapsed();
                        println!("time taken: {:?}", duration);
                      }
      Err(why)=> {
                  println!("{}",why);
                  print_correct_syntax();
                  process::exit(1);
                 }
   }   
}
