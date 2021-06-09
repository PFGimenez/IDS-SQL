use std::io::BufReader;
use std::fs::File;
use std::io::BufRead;

mod template;
mod ids;
mod tokens;
use ids::Ids;

fn main() -> std::io::Result<()> {
    let mut ids = Ids::new();

    let f = File::open("queries2.txt")?;
    let reader = BufReader::new(f);
    for line in reader.lines() {
        let l = line.unwrap();
        // println!("{}", l); 
        let fate = ids.handle_req(&l);
        println!("  {}", fate);
    }

    Ok(())
}


