use std::{
    collections::VecDeque,
    sync::{Condvar, Mutex},
    thread::{self, sleep},
    time::Duration,
};

fn main() {
    let task = Mutex::new(0);
    let max_buffer_size = 10;
    let space_available = Condvar::new();
    let more_stuff = Condvar::new();
    let buffer: Mutex<VecDeque<i32>> = Mutex::new(VecDeque::with_capacity(max_buffer_size));

    thread::scope(|s| {
        let mut threads = Vec::new();
        for _ in 0..2 {
            let producers = s.spawn(|| {
                loop {
                    let item = {
                        let mut t = task.lock().unwrap();
                        *t += 1;
                        *t
                    };
                    let mut container = buffer.lock().unwrap();
                    while container.len() == max_buffer_size {
                        container = space_available.wait(container).unwrap();
                    }
                    container.push_back(item);
                    println!("Produced:{:?}", container);
                    drop(container);
                    more_stuff.notify_one();
                    sleep(Duration::from_secs(1));
                }
            });
            threads.push(producers);
        }
        for _ in 0..2 {
            let consumers = s.spawn(|| {
                loop {
                    let mut container = buffer.lock().unwrap();
                    while container.len() == 0 {
                        container = more_stuff.wait(container).unwrap();
                    }
                    container.pop_front();
                    println!("Consumed:{:?}", container);
                    drop(container);
                    space_available.notify_one();
                    sleep(Duration::from_secs(1));
                }
            });
            threads.push(consumers);
        }
    });
    let buffer = buffer.lock().unwrap();
    println!("Its Done?: {:?}", buffer);
}
