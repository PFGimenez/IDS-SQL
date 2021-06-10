use std::io::BufReader;
use std::fs::File;
use std::io::BufRead;
use std::time::SystemTime;
use std::env;
use std::process;
use std::sync::mpsc;
use std::thread;

mod template;
mod ids;
mod tokens;
use ids::{Ids,ReqFate};

const NB_THREADS: usize = 12;

fn main() -> std::io::Result<()> {

    let mut ids = Ids::new();
    let mut threads = Vec::new();

    for _i in 0..NB_THREADS {
        let (tx1, rx1) = mpsc::channel();
        // let (tx2, rx2) = mpsc::channel();
        let mut ids_lock = ids.clone();
        threads.push((tx1,thread::spawn(move || {
                                loop {
                                    let s: Option<String> = rx1.recv().unwrap();
                                    match s {
                                        None => break,
                                        Some(s) => {
                                            let fate = Ids::handle_req(&mut ids_lock, &s, false);
                                            // tx2.send(fate).unwrap();
                                            // match fate {
                                            //     ReqFate::Unknown => unknown_nb += 1,
                                            //     ReqFate::Pass(_) | ReqFate::Trusted => pass_nb += 1,
                                            //     ReqFate::Del(_) | ReqFate::TokenError => del_nb += 1
                                            // };
                                            println!("{}: {}", s, fate);
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
                    Err(e) => println!("Error : {}", e)
                };
            };
            // ids.summarize();
        },
        None => ()
    };

    let f = File::open(query_file)?;
    let reader = BufReader::new(f);
    let before = SystemTime::now();
    let iter = reader.lines();
    // let mut pass_nb = 0;
    // let mut unknown_nb = 0;
    // let mut del_nb = 0;
    for line in iter {
        nb += 1;
        if nb%100 == 0 {
            println!("{}",nb)
        }
        match line {
            Ok(l) => {
                threads[nb%NB_THREADS].0.send(Some(l.clone())).unwrap();
                // let fate = Ids::handle_req(&mut ids, &l,false);
            },
            Err(e) => println!("Error : {}", e)
        }
    }

    for (tx,thr) in threads {
        tx.send(None).unwrap();
        let _ = thr.join();
    }

    let nb = nb as f64;
    let dur = SystemTime::now().duration_since(before.clone()).unwrap().as_millis() as f64;
    println!("Duration: {}ms per query (total: {}s)", dur/nb, dur/1000.);
    // println!("Number of pass: {}", pass_nb);
    // println!("Number of del: {}", del_nb);
    // println!("Number of unknown: {}", unknown_nb);
    Ids::summarize(&ids);
    Ok(())
}


