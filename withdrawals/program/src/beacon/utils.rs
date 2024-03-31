use alloy_primitive::u256;
use sha2::{Digest, Sha256};
use ssz_rs::prelude::Node;

// Verify ssz proof
pub fn is_vaild_merkel_big_branch<'a>(
    leaf : &Node,
    mut branch : impl Iterator<Item = &'a Node>,
    depth : usize,
    index : u256,
    root : &Node,
)->bool{
    let mut value: [u8;32] = leaf.as_ref().try_into().unwrap();

    let mut hasher  = Sha256::new();
    for i in 0..depth{
        let next_node  = match branch.next(){
            Some(node) => node,
            None => return false,
        };
        if index.bit(i){
            hasher.update(next_node.as_ref());
            hasher.update(value.as_ref());
        }else{
            hasher.update(value.as_ref());
            hasher.update(next_node.as_ref());
        }
    }
    let root:[u8;32] = root.as_ref().try_into().unwrap();
    root == value 
}

pub fn branch_form_bytes(s : &[u8;32])-> Vec<Node>{
    s.iter()
    .map(|hex| node_from_bytes(*hex))
    .collect::<Vec<Node>>()
}

pub fn node_from_bytes(s: [u8;32])->Node{
    Node.try_from(&s[..]).unwrap()
}