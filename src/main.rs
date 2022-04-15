use std::io::BufReader;
use std::fs::File;
use std::io::BufRead;
use std::time::SystemTime;
use std::env;
use std::process;
use std::sync::mpsc;
use std::thread;
use log::{error,warn,info};

mod template;
mod ids;
mod tokens;
use ids::{Ids,ReqFate,PredResult};

#[derive(Debug)]
pub enum Class {
    Benign,
    Attack
}

const NB_THREADS: usize = 1;

fn main() -> std::io::Result<()> {
    env_logger::init();

    let mut ids = Ids::new();
    let mut threads = Vec::new();

    for _i in 0..NB_THREADS {
        let (tx, rx) = mpsc::channel();
        let mut ids_lock = ids.clone();
        threads.push((tx,thread::spawn(move || {
                                loop {
                                    let s: Option<(String, Class)> = rx.recv().unwrap();
                                    match s {
                                        None => break,
                                        Some((s,class)) => {
                                            let fate = Ids::handle_req(&mut ids_lock, &s, false);
                                            let pred = match (&fate, class) {
                                                (ReqFate::Unknown | ReqFate::Trusted | ReqFate::Pass(_), Class::Benign) => PredResult::TN,
                                                (ReqFate::Unknown | ReqFate::Trusted | ReqFate::Pass(_), Class::Attack) => {warn!("FN: {} ({})", s, fate); PredResult::FN},
                                                (ReqFate::Del(_) | ReqFate::TokenError, Class::Attack) => PredResult::TP,
                                                (ReqFate::Del(_) | ReqFate::TokenError, Class::Benign) => {error!("FP: {} ({})", s, fate); PredResult::FP},
                                            };
                                            Ids::add_result(&mut ids_lock, pred, s.clone());
                                        }
                                    }
                                }
                            })));
    }

    let args: Vec<String> = env::args().collect();
    let (query_file, trusted_query_file) = match args.len() {
        2 | 3 => (args[1].clone(), args.get(2)),
        _ => {
            println!("Usage: {} queries.txt [trusted_queries.txt]", args[0]);
            process::exit(0);
        }
    };

    let mut nb = 0;

    match trusted_query_file {
        Some(f) => {
            let f = File::open(f)?;
            let reader = BufReader::new(f);
            for line in reader.lines()  {
                match line {
                    Ok(l) => {
                        Ids::handle_req(&mut ids, &l, true);
                    }
                    Err(e) => error!("Error : {}", e)
                };
            };
            Ids::summarize(&ids);
        },
        None => ()
    };

    let f = File::open(query_file)?;
    let reader = BufReader::new(f);
    let before = SystemTime::now();
    let mut iter = reader.lines();
    loop {

        match (iter.next(),iter.next())
        {
            (Some(l), Some(class)) => {
                let class = match class?.as_str()
                    {
                        "0" => Class::Benign,
                        _ => Class::Attack
                    };
                nb += 1;
                threads[nb%NB_THREADS].0.send(Some((l?.clone(),class))).unwrap()
            },
            _ => break
        }
    }

    for (tx,thr) in threads {
        tx.send(None).unwrap();
        let _ = thr.join();
    }

    let nb = nb as f64;
    let dur = SystemTime::now().duration_since(before.clone()).unwrap().as_millis() as f64;
    info!("Duration: {}ms per query (total: {}s)", dur/nb, dur/1000.);
    Ids::summarize(&ids);
    Ids::show_results(&ids);
    Ok(())
}
