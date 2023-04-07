
mod tree;
mod traversion_options;
mod list;

use tree::Tree;
//use traversion_options::TraversionOptions::{Thili,Tohi,Sofi};




fn main() {
    let mut t = Tree::<i32>::new(Some(1));
    for i in 2..=3 {
        t = t.insert(i);
    }
    println!("vec: {:?}", t.clone().to_vec(None, None));
    println!("count: {}", t.clone().count());
    println!("h: {}", t.clone().height());
    //println!("eff: {}", t.clone().efficency());
    t = t.balance();
    println!("vec: {:?}", t.clone().to_vec(None, None));
    println!("count: {}", t.clone().count());
    println!("h: {}", t.clone().height());
    //println!("eff: {}", t.clone().efficency());
    
}
