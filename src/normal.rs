use std::{sync::mpsc, thread, time::Duration};

use crossbeam_channel::bounded;

use crate::Wait;

pub fn proc() {
    let (out_tx, out_rx) = bounded::<i32>(0);
    let (in_tx, in_rx) = mpsc::sync_channel::<String>(0);

    // generator thread
    thread::spawn(move || {
        for i in 0..20 {
            thread::sleep(Duration::from_millis(100));
            out_tx.send(i).unwrap();
        }
    });

    // worker thread
    for _i in 0..5 {
        let out_rx = out_rx.clone();
        let in_tx = in_tx.clone();
        thread::spawn(move || {
            let mut w = Wait {
                rng: rand::thread_rng(),
            };
            for i in out_rx {
                w.wait(i);

                // compute in worker
                let out = (i * 10).to_string();

                in_tx.send(out).unwrap();
            }
        });
    }
    drop(in_tx);

    // prrinter loop in main thread
    for i in in_rx {
        println!("{}", i);
    }
}
