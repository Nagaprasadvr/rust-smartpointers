use std::cell::RefCell;
use std::rc::Rc;

#[derive(Debug)]
struct Data{
    pub key:String,
    pub value:String
}

impl Data{

    pub fn new()->Self{
        return Data{
            key:"solana".to_owned(),
            value:"LFG".to_owned()
        };

    }
    pub fn change_key(&mut self,new_key:String){
        self.key = new_key;
    }

    pub  fn change_value(&mut self,new_value:String){
        self.value = new_value;
    }
}

fn main(){

    //Reference counter for shared ownership and refcell for interior mutability
    let data = Rc::new(RefCell::new(Data::new()));

    //change value
    data.borrow_mut().change_value("lets fucking go!".to_owned());

    println!("Data:{:?}",data);

    //share ownership
    let data_clone = Rc::clone(&data);

    //share ownershipHMMqtjVBqxwC2kqGMd3oPQ7pSwyqbpGLzYTp1onr21G2
    data_clone.borrow_mut().change_key("SVM".to_owned());

    println!("Data:{:?}",data);

}
