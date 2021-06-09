use std::io::BufReader;
use std::fs::File;
use std::io::BufRead;
use std::time::SystemTime;
use std::env;
use std::process;

mod template;
mod ids;
mod tokens;
use ids::{Ids,ReqFate};

fn main() -> std::io::Result<()> {
    let args: Vec<String> = env::args().collect();
    let (query_file, trusted_query_file) = match args.len() {
        2 | 3 => (args[1].clone(), args.get(2)),
        _ => {
            println!("Usage: {} queries.txt [trusted_queries.txt]", args[0]);
            process::exit(0);
        }
    };

    let mut ids = Ids::new();
    let mut nb = 0;

    match trusted_query_file {
        Some(f) => {
            let f = File::open(f)?;
            let reader = BufReader::new(f);
            for line in reader.lines()  {
                match line {
                    Ok(l) => { 
                        ids.handle_req(&l, true).ok();
                    }
                    Err(e) => println!("Error : {}", e)
                };
            };
            ids.summarize();
        },
        None => ()
    };

    let f = File::open(query_file)?;
    let reader = BufReader::new(f);
    let before = SystemTime::now();
    let iter = reader.lines();
    let mut pass_nb = 0;
    let mut unknown_nb = 0;
    let mut del_nb = 0;
    for line in iter {
        nb += 1;
        if nb%100 == 0 {
            println!("{}",nb)
        }
        match line {
            Ok(l) => {
                let fate = ids.handle_req(&l,false);
                match fate {
                    Ok(f) => {
                        match f {
                            ReqFate::Unknown => unknown_nb += 1,
                            ReqFate::Pass(_) => pass_nb += 1,
                            ReqFate::Del(_) => del_nb += 1,
                            ReqFate::Trusted => (),
                        }
                        println!("{}: {}", l, f)
                    },
                    Err(e) => println!("Error: {:?}", e)
                }
            },
            Err(e) => println!("Error : {}", e)
        }
    }
    let nb = nb as f64;
    let dur = SystemTime::now().duration_since(before.clone()).unwrap().as_millis() as f64;
    println!("Duration: {}ms per query (total: {}ms)", dur/nb, dur);
    println!("Number of pass: {}", pass_nb);
    println!("Number of del: {}", del_nb);
    println!("Number of unknown: {}", unknown_nb);
    ids.summarize();
    Ok(())
}


