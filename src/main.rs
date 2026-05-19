use std::{thread, time::{self, Duration}};

fn main() {
    
    let thread1 = thread::spawn(|| {
        for i in 1..= 5{
            println!("hello world 1");
            thread::sleep(time::Duration::from_millis(500));
        }
    });

     thread::sleep(Duration::from_millis(1000));

    let thread2 = thread::spawn(|| {
        for i in 1..= 5{
            println!("hello world 2");
            thread::sleep(time::Duration::from_millis(1000));
        }
    });

    thread1.join().unwrap();
    thread2.join().unwrap();
}
