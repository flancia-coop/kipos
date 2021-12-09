use std::collections::HashMap;

mod seeder {}

mod node {
    use std::{any::Any, collections::HashMap, path};

    pub struct NodeViews<'a> {
        pub metadata: HashMap<&'a str, Box<dyn Any>>,
        pub exports: HashMap<&'a str, NodeViewType>,
    }

    pub struct File {
        pub path: path::PathBuf,
        pub contents: Vec<u8>,
    }

    pub enum NodeViewType {
        /// Vec<u8> represents a vector of bytes aka
        File(File),
        Other(Box<dyn Any>),
    }

    /// Entry Point for a node. Only this should be called.
    ///
    /// TODO: Make all fields here private, using a new function
    pub enum Node<'a> {
        NodeViews(NodeViews<'a>),
        File(File),
    }

    impl Node<'_> {
        pub fn new() -> Self {
            todo!()
        }
    }
}

mod garden {}

fn main() {}
