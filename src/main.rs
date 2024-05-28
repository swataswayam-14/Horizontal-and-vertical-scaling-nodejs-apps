//multithreading
use std::thread;

fn main() {
    for _ in 0..3{ //Spawn three threads
        thread::spawn(||{ //it is running on 4 cores of the cpu are utilised in their peak capacity
            let mut counter: f64 = 0.00;
            loop{
                counter+=0.0001;
            }
        });
    }    
    loop{

    }
}
