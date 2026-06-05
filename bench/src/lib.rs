// TODO: potentially instantiate logs with an extremely high capacity
pub mod bench;

use std::{
    collections::HashMap,
    fs::{File, create_dir_all},
    io::Write,
    path::Path,
    sync::{Arc, Mutex},
    time::Instant,
};

use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Debug)]
pub struct MainBenchRunner {
    // this shouldnt slow down any benchmarks since this is only accessed when a
    // test runner completes.
    inner: Arc<Mutex<MainBenchRunnerInner>>,
}

impl MainBenchRunner {
    pub fn new() -> Self {
        Self {
            inner: Arc::new(Mutex::new(MainBenchRunnerInner {
                start: Instant::now(),
                results: vec![],
            })),
        }
    }

    pub fn spawn_runner(&self, id: String) -> BenchRunner {
        BenchRunner {
            main: self.inner.clone(),
            log: BenchEventLog {
                log: vec![BenchEvent {
                    instant: Instant::now(),
                    event: BenchEventData::RunnerStarted,
                    additional: vec![],
                }],
                runner_id: id,
            },
        }
    }

    pub fn write_results_to_file(&self, path: &str) {
        let locked = self.inner.lock().unwrap();
        let transformed: HashMap<String, FinalBenchEventLog> = locked
            .results
            .iter()
            .map(|r| {
                (
                    r.runner_id.clone(),
                    FinalBenchEventLog {
                        log: r
                            .log
                            .iter()
                            .map(|l| FinalBenchEvent {
                                elapsed_s: l.instant.duration_since(locked.start).as_secs_f32(),
                                event: l.event.clone(),
                                additional: l.additional.iter().map(Clone::clone).collect(),
                            })
                            .collect(),
                    },
                )
            })
            .collect();

        create_dir_all(Path::new(path).parent().unwrap()).unwrap();

        let mut file = File::create(path).unwrap();

        file.write_all(&serde_json::to_vec(&transformed).unwrap())
            .unwrap();
    }
}

#[derive(Debug)]
struct MainBenchRunnerInner {
    start: Instant,
    results: Vec<BenchEventLog>,
}

#[derive(Debug)]
pub struct BenchRunner {
    main: Arc<Mutex<MainBenchRunnerInner>>,
    log: BenchEventLog,
}

impl BenchRunner {
    pub fn spawn_runner(&self, id: String) -> Self {
        let id = format!("{}::{}", self.log.runner_id, id);
        Self {
            main: self.main.clone(),
            log: BenchEventLog {
                log: vec![BenchEvent {
                    instant: Instant::now(),
                    event: BenchEventData::RunnerStarted,
                    additional: vec![],
                }],
                runner_id: id,
            },
        }
    }
    pub fn record(&mut self, event: BenchEventData, additional: Vec<(String, Value)>) {
        self.log.log.push({
            BenchEvent {
                instant: Instant::now(),
                event,
                additional,
            }
        })
    }
}

impl Drop for BenchRunner {
    fn drop(&mut self) {
        self.record(BenchEventData::RunnerClosed, vec![]);
        self.main.lock().unwrap().results.push(std::mem::replace(
            &mut self.log,
            BenchEventLog {
                runner_id: String::new(),
                log: vec![],
            },
        ));
    }
}

#[derive(Debug, Default)]
pub struct BenchEventLog {
    runner_id: String,
    log: Vec<BenchEvent>,
}

#[derive(Debug)]
pub struct BenchEvent {
    instant: Instant,
    event: BenchEventData,
    additional: Vec<(String, Value)>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum BenchEventData {
    RunnerStarted,
    RunnerClosed,
    ValueSent,
    ValueReceived,
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct FinalBenchEventLog {
    log: Vec<FinalBenchEvent>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FinalBenchEvent {
    elapsed_s: f32,
    event: BenchEventData,
    additional: HashMap<String, Value>,
}
