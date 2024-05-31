use std::sync::mpsc;

fn main() {
    let (tx, rx) = mpsc::channel();

    for i in 0..10 {
        let my_tx = tx.clone();
        std::thread::spawn(move || {
            let numbers: Vec<usize> = (i*10 .. i*1000).collect();
            let sum: usize = numbers.iter().sum();
            my_tx.send(sum).unwrap();
        });
    }

    let mut total = 0;
    for _ in 0..10 {
        total += rx.recv().unwrap();
    }
    println!("Total: {}", total);
}
