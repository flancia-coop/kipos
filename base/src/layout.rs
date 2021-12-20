//! This is just a outline so I know how to do shit.
//!
//! Don't copy and paste code from here, it's kinda a final project structure if you will

fn main() {
    unimplemented!()
}

pub mod config {
    use super::plugin::PluginType;
    use std::{any::Any, collections::HashMap};

    /// This is a private instance of configuration we use. It contains instances of [GardenConfig].
    ///
    /// At any given moment, there should be one owned `Config` Instance.
    /// It contains certain features relevent to the site like plugins.
    ///
    /// Do note the docs of [GardenConfig]

    struct Config<'a> {
        plugins: HashMap<String, PluginType>,
        flags: HashMap<&'a str, Box<dyn Any>>,
        /// {garden name, [GardenConfig]}
        children: Option<HashMap<&'a String, GardenConfig<'a>>>
    }

    pub struct GardenConfig<'a> {
        /// there
        level: u8,
        flags: HashMap<&'a str, Box<dyn Any>>,
        children: Option<Vec<GardenConfig<'a>>>

    }
}

mod plugin {
    use super::node::Node;


    pub struct Seeder {
    }

    trait Plugin {
        fn fetch(&self) -> Vec<Node>;

    }

    pub enum PluginType {
        Seeder(Seeder),
    }
}

/// A node
pub mod node {
    use std::{any::Any, collections::HashMap, path};

    pub struct NodeView<'a> {
        pub metadata: HashMap<&'a str, Box<dyn Any>>,
        pub exports: HashMap<&'a str, NodeViewType<'a>>,
    }

    /// A file
    pub struct File<'a> {
        /// The path the file should be placed at. `./` is the garden root.
        pub path: path::PathBuf,
        /// Vec<u8> represents a vector of bytes aka the actual file content
        pub contents: Vec<u8>,
        /// File metadata
        ///
        /// If called via [NodeView], this will be merged with it.
        pub metadata: Option<HashMap<&'a str, Box<dyn Any>>>,
        /// Where the file came from
        ///
        /// In rare circumstances this can't be provided.
        pub og_path: Option<path::PathBuf>,
    }
    pub enum NodeViewType<'a> {
        File(File<'a>),
        Other(Box<dyn Any>),
    }

    /// Entry Point for a node. Only this should be called.
    ///
    /// TODO: Make all fields here private, using a new function
    pub enum Node<'a> {
        NodeViews(NodeView<'a>),
        File(File<'a>),
    }

    impl Node<'_> {
        pub fn new() -> Self {
            todo!()
        }
    }
}

pub mod garden {
    use std::path::PathBuf;

    use super::{config::GardenConfig, node::Node};

    pub struct Garden<'a> {
        name: String,
        path: PathBuf,
        nodes: Vec<Node<'a>>,
        config: Option<GardenConfig<'a>>,
    }
}
