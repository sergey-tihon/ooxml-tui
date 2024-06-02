use std::io;

use tui_tree_widget::TreeState;

#[derive(Debug, Clone)]
pub struct Node {
    pub name: String,
    pub children: Vec<Box<Node>>,
}

impl Node {
    fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
            children: Vec::<Box<Self>>::new(),
        }
    }

    fn find_child(&mut self, name: &str) -> Option<&mut Self> {
        for c in self.children.iter_mut() {
            if c.name == name {
                return Some(c);
            }
        }
        None
    }

    fn add_child<T>(&mut self, leaf: T) -> &mut Self
    where
        T: Into<Self>,
    {
        self.children.push(Box::new(leaf.into()));
        self
    }
}

pub struct App {
    pub file_path: String,
    pub tree_state: TreeState<String>,
    pub root: Node,
}

impl App {
    pub fn from_file(path: String) -> io::Result<Self> {
        let file = std::fs::File::open(path.clone())?;
        let archive = zip::ZipArchive::new(file)?;

        let mut root = Node::new("root");

        for file in archive.file_names() {
            let path = file.split('/').collect::<Vec<&str>>();
            App::build_tree(&mut root, &path, 0);
        }

        Ok(Self {
            file_path: path,
            tree_state: TreeState::default(),
            root,
        })
    }

    fn build_tree(node: &mut Node, parts: &Vec<&str>, depth: usize) {
        if depth < parts.len() {
            let item = &parts[depth];

            let dir = match node.find_child(item) {
                Some(d) => d,
                None => {
                    let d = Node::new(item);
                    node.add_child(d);
                    match node.find_child(item) {
                        Some(d2) => d2,
                        None => panic!("Got here!"),
                    }
                }
            };
            App::build_tree(dir, parts, depth + 1);
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::App;
    use std::io;

    #[test]
    fn load_pptx() -> io::Result<()> {
        let app = App::from_file("data/sample.pptx".to_string())?;
        app.root.children.iter().for_each(|node| {
            println!("{}", node.name);
        });

        Ok(())
    }
}
