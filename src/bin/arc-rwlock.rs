use std::thread; 
use std::sync::Arc;
use std::sync::RwLock;
use std::thread::JoinHandle;


fn main(){
    // Arc - Atomic referecne counter
    // RwLock - Read Writer Lock (allows multiple reader to access data and only one write at a
    //          time to mutate the critical section data)
    let critical_sec_data = Arc::new(RwLock::new(100));

    let mut thread_handles:Vec<JoinHandle<()>> = Vec::new();

    // cloning dara usig Arc::clone
    let data_clone_1 = Arc::clone(&critical_sec_data);

    // spawning two threads
    thread_handles.push(
    thread::spawn(move ||{
            for _ in 0..10{
                *data_clone_1.write().unwrap() +=1;
                println!("data in thread 1:{:?}",data_clone_1);
            }
        })
    );
    
    let data_clone_2 = Arc::clone(&critical_sec_data);
   
    thread_handles.push(    
    thread::spawn(move ||{
            for _ in 0..10 {  
                *data_clone_2.write().unwrap() +=1;
                println!("data in thread 2:{:?}",data_clone_2);
            }
        })
    );

    println!("Main thread exec..");

    //wait for both threads to finish
    for handles in thread_handles{
        handles.join().unwrap();
    }


    assert_eq!(*critical_sec_data.read().unwrap(),120);


}
