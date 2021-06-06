mod template;
mod ids;
mod tokens;
use ids::Ids;

fn main() {
    let mut ids = Ids::new();
    println!("{}", ids.handle_req("SELECT * FROM table ; SELECT * FROM users WHERE A=\'1\'"));
    println!("{}", ids.handle_req("SELECT * FROM table1 WHERE A=\'1\'"));
    println!("{}", ids.handle_req("SELECT * FROM table2 WHERE A=\'1\'"));
    println!("{}", ids.handle_req("SELECT * FROM table5 WHERE B=\'1\'"));
    println!("{}", ids.handle_req("SELECT * FROM table5 WHERE B=\'2\'"));
    println!("{}", ids.handle_req("SELECT * FROM table3 WHERE A=\'1\'"));
    println!("{}", ids.handle_req("SELECT * FROM table4 WHERE A=\'1\'"));
    println!("{}", ids.handle_req("SELECT * FROM table5 WHERE A=\'1\'"));
    println!("{}", ids.handle_req("SELECT * FROM table ; SELECT * FROM users WHERE A=\'1\'"));
}


