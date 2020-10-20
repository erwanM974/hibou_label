


pub struct ProcessPriorities {
    pub emission : i32,
    pub reception : i32,
    pub in_loop : i32,
    pub step : Option<i32>
}

impl ProcessPriorities {
    pub fn new(emission : i32,
               reception : i32,
               in_loop : i32,
               step : Option<i32>) -> ProcessPriorities {
        return ProcessPriorities{emission,reception,in_loop,step};
    }
}

impl std::string::ToString for ProcessPriorities {
    fn to_string(&self) -> String {
        let mut my_str = String::new();
        match &self.step {
            None => {},
            Some(step) => {
                my_str.push_str( &format!("step={:},",step) );
            }
        }
        my_str.push_str( &format!("emission={:},",self.emission) );
        my_str.push_str( &format!("reception={:},",self.reception) );
        my_str.push_str( &format!("loop={:}",self.in_loop) );
        return my_str;
    }
}

