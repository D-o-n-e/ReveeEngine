
pub use bevy::prelude::*;
use pyo3::prelude::*;
use pyo3::wrap_pyfunction;
use globals::*;
use std::collections::HashMap;


#[pyclass]
#[derive(Default, Debug, Clone)]
pub struct Tree{
    pub nodes: Vec<Node>
}

#[pyclass]
#[derive(Debug, Clone)]
pub enum Nodes{
    Texture
}

#[pyclass]
#[derive(Debug, Clone)]
pub struct Node{
    node: Nodes,
    values: HashMap<String, String>,
    position: Vec2
}

#[pyclass]
#[derive(Debug, Clone)]
pub struct Vec2{
    pub x: f32,
    pub y: f32
}
#[pyclass]
pub struct Engine{
    pub tree: Tree,
}

#[pymethods]
impl Vec2{
    #[new]
    fn new(a: f32, b: f32) -> Self{
        Vec2{x: a, y: b}
    }
}

#[pymethods]
impl Tree{
    #[new]
    fn new(a: Vec<Node>) -> Self{
        Tree{nodes: a}
    }
}

#[pymethods]
impl Node{
    #[new]
    fn new(a: Nodes, b: HashMap<String, String>, c: Vec2) -> Self{
        Node{node: a, values: b, position: c}
    }
}

pub fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    for node in &globals::get::<Tree>().nodes{
        let (path, position) = 
            match node.node{
                Nodes::Texture => {
                    (node.values.get("path").unwrap(), &node.position)
                }
        };
        commands.spawn_bundle(OrthographicCameraBundle::new_2d());
        commands.spawn_bundle(SpriteBundle {
            texture: asset_server.load(path),            
            transform: Transform::from_xyz(position.x, position.y, 0.),
            ..default()
        });
    }
}

#[pymethods]
impl Engine{
    #[new]
    fn new(a: Tree) -> Self{
        Engine{tree: a}
    }

    pub fn start(&mut self) {
        self.tree = Tree{nodes: vec!()};
        App::new()
            .add_plugins(DefaultPlugins)
            .add_startup_system(setup)
            .run();
    }
    pub fn create_node(&mut self, node: Nodes, position: Vec2, values: HashMap<String, String>) -> Node{
        Node{node: node, position: position, values: values}
        //match node{
            //Nodes::Texture{path, position} => {
                //globals::get::<Node>().node = Nodes::Texture{path: path, position: position};
            //}
            //_ => {}
        //}      
    }
    pub fn add_node(&mut self, node: Node){
        self.tree.nodes.append(&mut vec!(node));
        globals::get::<Tree>().nodes = (&*self.tree.nodes).to_vec();       
    }

}

#[pymodule]
#[pyo3(name = "ReveeEngine")]
fn revee_engine(py: Python, m: &PyModule) -> PyResult<()> {
    m.add_class::<Node>()?;
    m.add_class::<Nodes>()?;
    m.add_class::<Engine>()?;
    m.add_class::<Tree>()?;
    m.add_class::<Vec2>()?;
    //m.add_class::<Nodes>()?;
    //m.add_class::<Engine>()?;
    //m.add_class::<Node>()?;
    //m.add_class::<Tree>()?;

    Ok(())
}