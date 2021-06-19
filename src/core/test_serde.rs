use serde::{Serialize,Deserialize};

#[derive(Serialize,Deserialize,Debug)]
struct Point{
    x:i32,
    y:i32,
}

#[test]
fn serde_test(){
    let point=Point{x:1,y:2};
    let serialized=serde_json::to_string(&point).unwrap();
    println!("{}",serialized)
}