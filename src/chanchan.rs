use std::{sync::mpsc, thread, time::Duration};

use crossbeam_channel::{bounded, Receiver, Sender};

use crate::Wait;

struct ChanChanTx<T, U> {
    payload: T,
    tx: Sender<U>,
}
struct ChanChanRx<U> {
    rx: Receiver<U>,
}

pub fn proc() {
    let (out_tx, out_rx) = bounded::<ChanChanTx<i32, String>>(0);
    // 以下の channel の容量を増やすと `normal.proc()` に近い実行時間となる(オーバーヘッド分は遅くなる)
    let (in_tx, in_rx) = mpsc::sync_channel::<ChanChanRx<String>>(0);

    // generator thread
    thread::spawn(move || {
        for i in 0..20 {
            thread::sleep(Duration::from_millis(100));
            let (tx, rx) = bounded::<String>(0);
            out_tx.send(ChanChanTx { payload: i, tx }).unwrap();
            in_tx.send(ChanChanRx { rx }).unwrap();
        }
    });

    // worker thread
    for _i in 0..5 {
        let out_rx = out_rx.clone();
        thread::spawn(move || {
            let mut w = Wait {
                rng: rand::thread_rng(),
            };
            for i in out_rx {
                w.wait(i.payload);

                // compute in worker
                let out = (i.payload * 10).to_string();

                i.tx.send(out).unwrap();
            }
        });
    }

    // prrinter loop in main thread
    for i in in_rx {
        println!("{}", i.rx.recv().unwrap());
    }
}
