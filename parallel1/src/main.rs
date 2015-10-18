use std::thread::sleep_ms;
use std::io;
use std::sync::mpsc as comm;
use std::thread;
use std::io::BufRead;

fn worker(task_num: u64, rx: comm::Receiver<()>) {
    let mut j = 0i32;
    loop {
        sleep_ms(1000);

        match rx.try_recv() {
            Ok(()) => {
                println!("Task #{} recv'd kill signal; terminating", task_num);
                return
            },
            Err(_) => ()
        }

        println!("Task #{}; iteration {}.", task_num, j);
        j += 1;
    }
}

fn main() {
    let stdin = io::stdin();
    let mut task_senders: Vec<(u64, comm::Sender<()>)> = Vec::new();
    let mut next_task = 0_u64;

    println!("Type c<cr> to create a task; k<cr> to kill the newest task; q<cr> to quit.");

    for line in stdin.lock().lines() {
        match line.unwrap().trim() {
            "c" => {
                let (tx, rx) = comm::channel();
                task_senders.push((next_task, tx));

                println!("Spawning task #{}", next_task);
                thread::spawn(move|| {
                    worker(next_task, rx);
                });
                next_task += 1;
            },
            "k" => {
                match task_senders.pop() {
                    Some((task_num, tx)) => {
                        tx.send(());
                        println!("Kill signal sent to task #{}", task_num);
                        next_task -= 1;
                    },
                    None     => println!("No task on stack to kill!"),
                }
            },
            "q" => {
                println!("Exiting. Byee!");
                break
            }
            other => println!("\"{}\" unexpected; try again", other),
        }
    }
}
