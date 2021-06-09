use std::io::BufReader;
use std::fs::File;
use std::io::BufRead;
use std::time::SystemTime;

mod template;
mod ids;
mod tokens;
use ids::Ids;

fn main() -> std::io::Result<()> {
    let mut ids = Ids::new();
    let mut nb = 0;
    // let f = File::open("queries.txt")?;
    let f = File::open("../../log_v3.txt")?;
    let reader = BufReader::new(f);
    let before = SystemTime::now();
    let iter = reader.lines();
    let len = iter.size_hint().1;
    for line in iter {
        nb += 1;
        // if nb%100 == 0 {
        //     match len {
        //         Some(upper) => println!("{}/{}",nb,upper),
        //         None => println!("{}",nb)
        //     }
        // }
        match line {
            Ok(l) => {
                let fate = ids.handle_req(&l,false);
                match fate {
                    Ok(f) => println!("{}: {}", l, f),
                    Err(e) => println!("Error: {:?}", e)
                }
            },
            Err(e) => println!("Error : {}", e)
        }
    }
    let nb = nb as f64;
    let dur = SystemTime::now().duration_since(before.clone()).unwrap().as_millis() as f64;
    println!("Duration: {}ms per query (total: {}ms)", dur/nb, dur);
    ids.summarize();
    Ok(())
}


