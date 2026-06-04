use std::thread;

use bench::{BenchEventData, BenchRunner, MainBenchRunner};
use log::info;
use mpac_rs::{BlockingReceive, BlockingSend, ChannelMaker};

/// ## Bench:
///
/// - 1 core reserved for gathering metrics
///
/// ### Metrics (Mean, P{50, 90, 99, 999}):
///
/// - Send/Receive Throughput
/// - Sender/Receiver Latency
///
/// #### Other
///
/// - Metrics' scaling with # of Sender/Receiver threads
///
/// ### Scenarios
///
/// - Pure value channel
///     - 1-1, 7-1, 1-7, 4-4, 7-7 sender-receiver threads
///     - T sizes: 4 bytes, 64 bytes, 8 kB, 64 kB
///
/// - One request and one response channel
///     - 1-1, 4-4, 6-1, 1-6 sender-receiver threads for each channel
/// - Sending sequenced data which has to be re-constructed and ordered by receivers
///     - 1 unique series per sender
///     - all receivers need to cooperate for each series and maintain a collection of sequenced values
///     - (1-1, 7-1, 1-7, 4-4, 7-7)
fn main() {
    env_logger::builder()
        .target(env_logger::Target::Stdout)
        .filter_level(log::LevelFilter::Debug)
        .init();
    info!("Starting benchmark");
    let makers = vec![("v1_naive", Box::new(mpac_rs::v1::V1Maker))];
    let runner = MainBenchRunner::new();

    for (desc_id, version) in makers {
        let runner = runner.spawn_runner(format!("version_{}", desc_id));
        run_bench_1(&runner, version.as_ref());
    }

    info!("Benchmarks completed. Writing results");

    runner.write_results_to_file("results/benchmark_results.json");
}

fn run_bench_1<Maker>(runner: &BenchRunner, maker: &Maker)
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
                let mut counter = 0u64;
                let tx = tx_thread;
                for _ in 0..100_000 {
                    if let Err(_) = tx.send(counter) {
                        break;
                    } else {
                        tx_runner.record(
                            BenchEventData::ValueSent,
                            vec![("value".into(), counter.into())],
                        );
                        counter += 1;
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
