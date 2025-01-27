use anyhow::{anyhow, Result};
use std::{sync::mpsc, thread, time::Duration};
const NUM_PRODUCERS: usize = 4;
#[allow(dead_code)]
#[derive(Debug)]
struct Msg {
    idx: usize,
    value: usize,
}
impl Msg {
    fn new(idx: usize, value: usize) -> Self {
        Msg { idx, value }
    }
}
fn main() -> Result<()> {
    println!("Hello world");
    let (tx, rx) = mpsc::channel();
    for idx in 0..NUM_PRODUCERS {
        let tx = tx.clone();
        thread::spawn(move || producer(idx, tx));
    }
    let consumer = thread::spawn(|| {
        for msg in rx {
            println!("consumer: {:?}", msg);
        }
    });
    consumer
        .join()
        .map_err(|e| anyhow!("Thread join error: {:?}", e))?;
    Ok(())
}
fn producer(idx: usize, tx: mpsc::Sender<Msg>) -> Result<()> {
    loop {
        let value = rand::random::<usize>();
        tx.send(Msg::new(idx, value))?;
        thread::sleep(Duration::from_millis(1000));
    }
}
