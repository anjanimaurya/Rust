use log::LevelFilter;
use log4rs::append::file::FileAppender;
use log4rs::encode::pattern::PatternEncoder;
use log4rs::config::{Appender, Config, Root};



pub fn init_log(level:&str){
    let log_level:LevelFilter = match level{
        "INFO"=> LevelFilter::Info,
        "ERROR"=> LevelFilter::Error,
        "DEBUG" => LevelFilter::Debug,
        _ => LevelFilter::Trace,
    };

    let pattern_encoder = PatternEncoder::new("{d(%Y-%m-%dT%H:%M:%S)} {l} - {m}\n");
    let logfile = FileAppender::builder()
        .encoder(Box::new(pattern_encoder))
        .build("logs/errors.log").unwrap();
    let config = Config::builder()
        .appender(Appender::builder().build("logfile", Box::new(logfile)))
        .build(Root::builder()
            .appender("logfile")
            .build(log_level)).unwrap();
    log4rs::init_config(config).unwrap();
}