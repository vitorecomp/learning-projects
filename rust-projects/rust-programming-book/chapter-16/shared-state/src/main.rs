use std::{sync::{Mutex, Arc}, rc::Rc, thread};

fn main() {
    let m = Mutex::new(5);

    {
        let mut num = m.lock().unwrap();
        *num = 6;
    }

    println!("m = {:?}", m);


    //the main point here is the usage of the atomic reference counter
    //the atomic implementations are thread safe, so anytime
    //it's necessary to use the data between threads, it's necessary
    //to use the atomic reference counter (it should implement the send trait)

    let counter = Arc::new(Mutex::new(0));
    let mut handles = vec![];

    for _ in 0..10 {
        let counter = Arc::clone(&counter);
        let handle = thread::spawn(move || {
            let mut num = counter.lock().unwrap();

            *num += 1;
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    println!("Result: {}", *counter.lock().unwrap());
}