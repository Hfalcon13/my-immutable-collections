
mod tree;
mod traversion_options;

use tree::Tree;
use traversion_options::TraversionOptions::{Thili,Tohi,Sofi};




fn main() {
    let mut t = Tree::<i32>::new(Some(1));
    //t = t.insert(Some(Thili), 0);
    t = t.insert(Some(Tohi), 2);
    //t = t.insert(Some(Sofi), 3);
    println!("vec: {:?}", t.clone().to_vec(None, None));
    println!("count: {}", t.clone().count());
    println!("h: {}", t.clone().height());
    println!("eff: {}", t.clone().efficency());
}
