mod template;
mod ids;
use ids::Ids;

fn main() {
    let mut ids = Ids::new();
    let status = ids.handle_req("SELECT * FROM taable WHERE A=1");
    println!("{:?}", status);
}


