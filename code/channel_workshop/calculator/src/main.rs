use std::{sync::mpsc, thread};

enum Operation {
    Add(f64),
    Subtract(f64),
    Multiply(f64),
    Divide(f64),
}

struct Command {
    operation: Operation,
    tx: mpsc::Sender<f64>,
}

fn main() {
    let (tx, rx) = mpsc::channel::<Command>();
    let (result_tx, result_rx) = mpsc::channel();

    // Spawn a thread to be the calculator actor
    thread::spawn(move || {
        let mut current_value = 0.0;

        while let Ok(command) = rx.recv() {
            match command.operation {
                Operation::Add(value) => current_value += value,
                Operation::Subtract(value) => current_value -= value,
                Operation::Multiply(value) => current_value *= value,
                Operation::Divide(value) => current_value /= value,
            }

            command.tx.send(current_value).unwrap();
        }
    });

    // Send some commands to the calculator actor
    let commands = vec![
        Command {
            operation: Operation::Add(10.0),
            tx: result_tx.clone(),
        },
        Command {
            operation: Operation::Subtract(5.0),
            tx: result_tx.clone(),
        },
        Command {
            operation: Operation::Multiply(2.0),
            tx: result_tx.clone(),
        },
        Command {
            operation: Operation::Divide(4.0),
            tx: result_tx.clone(),
        },
    ];

    let n_commands = commands.len();
    for command in commands {
        tx.send(command).unwrap();
    }

    // Receive the results
    for _ in 0..n_commands {
        println!("Result: {}", result_rx.recv().unwrap());
    }
}
