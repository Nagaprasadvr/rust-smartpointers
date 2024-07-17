use std::{cell::RefCell, rc::Rc};

// 1. Rc - Refrence Counter - (unsafe rust, provides shared ownership meaning a value is owned by
//         multiple owners and the value is dropped when the last varianle goes out of scope)
// 2. RefCell - Reference Cell (provides Interior Mutability and obeys all borrowing rules)

fn main(){
    //RefCell without Rc
    let demo_data = RefCell::new(100);

    *demo_data.borrow_mut() += 20;
    println!("Demo Data:{:?}",demo_data);
   
    //data allocated on stack having value 10 with Rc
    let data = Rc::new(RefCell::new(10));

    //borrow as mut and mutate the vaule even if data is not marked mut
    *data.borrow_mut() += 20;

    //prints 20
    println!("data:{:?}",data);


    //share ownership with another variable
    let data_clone = Rc::clone(&data);


    //mutate data val
    *data_clone.borrow_mut() +=100;

    //original data
    println!("Original data:{:?}",data);


    //shared ownership data
    println!("Shared ownership data:{:?}",data_clone);

} // here data_clone goes out of scope so all owners are dropped

