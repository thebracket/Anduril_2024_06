fn main() {
    use std::thread;
    use std::time::Duration;
    use crossbeam_channel::{select, unbounded};
    
    let (s1, r1) = unbounded();
    let (s2, r2) = unbounded();
    
    thread::spawn(move || {
        thread::sleep(Duration::from_secs(1));
        s1.send(10).unwrap();
    });
    thread::spawn(move || {
        thread::sleep(Duration::from_millis(500));
        s2.send(20).unwrap();
    });
    
    // None of the two operations will become ready within 100 milliseconds.
    select! {
        recv(r1) -> msg => panic!(),
        recv(r2) -> msg => panic!(),
        default(Duration::from_millis(100)) => println!("timed out"),
    }
}