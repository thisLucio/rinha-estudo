#![allow(unused)]
use std::fs;

use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct File{
    name: String,
    expression: Term,
    
}
#[derive(Debug, Deserialize)]
pub struct Int{
    value: i32,
}
#[derive(Debug, Deserialize)]
pub struct Str{
    value: String,
}

#[derive(Debug, Deserialize)]
pub struct Print{
    value: Box<Term>,
}

#[derive(Debug, Deserialize)]
#[serde(tag = "kind")]
pub enum Term{
    Int(i32),
    Str(Str),
    Print(Print),
}

#[derive(Debug)]
pub enum Val{
    Void,
    Int(i32),
    Bool(bool),
    Str(String),
}

fn eval(term: Term) -> Val {
    todo!()
}
fn main() {
   let program  = fs::read_to_string("./examples/hello.json").unwrap();
   let program = serde_json::from_str::<File>(&program).unwrap();

   println!("{program:?}");
}
