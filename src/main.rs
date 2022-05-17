mod lib;
use std::collections::HashMap;

fn main(){
    let mut node_values = HashMap::new();
    node_values.insert("path".to_string(), "icon.png".to_string());
    let mut engine = lib::Engine{tree: lib::Tree{nodes: vec!()}};
    let sprite: lib::Node = engine.create_node(lib::Nodes::Texture, lib::Vec2{x: 0.,y: 0.}, node_values);
    //let sprite2: lib::Node = engine.create_node(lib::Nodes::Texture, values: ,lib::Vec3::new(50.,0.,0.));
    engine.add_node(sprite);
    //engine.add_node(sprite2);
    engine.start();
}

//
//
//
