

enum EQUITIES{
    STOCKS,
    ETF,
}


#[derive(Debug, Clone)]
pub struct Equity{
    symbol:String,
    class:String,
    threshold: Option<f64>,
}

impl Equity{
    pub fn new()->Self{
        Equity{
            symbol:"".to_string(),
            class:"".to_string(),
            threshold:None,
        }
    }
    pub fn set_symob(&mut self, code:String){
        self.symbol = code;
    }
    pub fn get_symbol(&self)->&str{
        &self.symbol
    }

    pub fn set_class(&mut self, class:String){
        self.class = class;
    }
    pub fn get_class(&self)->&str{
        &self.class
    }

    pub fn set_threshold(&mut self, value:Option<f64>){
        self.threshold = value;
    }
    pub fn get_threshold(self)->Option<f64>{
        self.threshold
    }
}




