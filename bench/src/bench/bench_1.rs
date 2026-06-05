use std::{thread, time::Instant};

use mpac_rs::{BlockingReceive, BlockingSend, ChannelMaker};

use crate::{BenchEventData, BenchRunner};

pub struct Bench1Config {
    pub n_senders: usize,
    pub n_receivers: usize,
}

pub fn run_bench_1<Maker>(runner: &BenchRunner, maker: &Maker)
where
    Maker: ChannelMaker,
{
    let mut handles = vec![];

    // Scope to ensure values get dropped appropriately
    {
        let (tx, rx) = maker.channel();
        for i in 0..7 {
            let mut tx_runner = runner.spawn_runner(format!("tx_runner_{}", i));
            let tx_thread = tx.clone();
            let s_h: thread::JoinHandle<()> = thread::spawn(move || {
                let start = Instant::now();
                let mut counter = 0u64;
                let tx = tx_thread;
                loop {
                    if let Ok(_) = tx.send(counter) {
                        tx_runner.record(
                            BenchEventData::ValueSent,
                            vec![("value".into(), counter.into())],
                        );
                        counter += 1;
                    } else {
                        break;
                    }
                    if start.elapsed().as_secs_f32() > 30.0 {
                        break;
                    }
                }
            });
            handles.push(s_h);
        }

        for i in 0..1 {
            let mut rx_runner = runner.spawn_runner(format!("rx_runner_{}", i));
            let rx_thread = rx.clone();
            let r_h = thread::spawn(move || {
                let rx = rx_thread;
                while let Ok(r) = rx.recv() {
                    rx_runner.record(
                        BenchEventData::ValueReceived,
                        vec![("value".into(), r.into())],
                    );
                }
            });
            handles.push(r_h);
        }
    }
    for handle in handles {
        handle.join().unwrap();
    }
}
