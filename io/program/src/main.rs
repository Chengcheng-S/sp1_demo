#![no_main]
sp1_zkvm::entrypoint!(main);

use serde::{Deserialize,Serialize};

#[derive(Debug,PartialEq,Deserialize,Serialize)]
pub struct MyPointUnaligned{
    pub x : usize,
    pub y : usize,
    pub b : bool,
}

pub fn main() {
   let p1 = sp1_zkvm::io::read::<MyPointUnaligned>();
   println!("read point: {:?}", p1);

   let p2 = sp1_zkvm::io::read::<MyPointUnaligned>();
   println!("read point: {:?}", p2);

   let p3:MyPointUnaligned = MyPointUnaligned{
       x : p1.x + p2.x,
       y : p1.y + p2.y,
       b : p1.b || p2.b,    
   };

   sp1_zkvm::io::write(&p3);
}
