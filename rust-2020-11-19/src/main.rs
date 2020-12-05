use rand::random;
use std::sync::mpsc::channel;
use std::thread;
use std::time::Duration;

fn main() {
    let (tx, rx) = channel();
    for _ in 0..8 {
        let jh = thread::spawn(|| {
            for _ in 0..100 {
                thread::sleep(Duration::from_secs(1));
                if random::<u8>() == 1 {
                    panic!("Oh no I panicked! ({})", thread::current().name().unwrap_or("<unnamed>") );
                }
            }
        });
        let tx = tx.clone();

        thread::spawn(move || {
            // This unwrap will succeed until rx is closed, and then silently kill this
            // thread
            tx.send(jh.join()).unwrap();
        });
    }

    for ordered_join_result in rx {
        if ordered_join_result.is_err() {
            println!("A thread errored");
        }
    }
    println!("Hello, world!");
}
