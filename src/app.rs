use std::io;

use tui_tree_widget::{TreeItem, TreeState};

#[derive(Debug, Clone)]
pub struct Node {
    pub name: String,
    pub children: Vec<Node>,
}

impl Node {
    fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
            children: Vec::<Self>::new(),
        }
    }

    fn find_child(&mut self, name: &str) -> Option<&mut Self> {
        self.children.iter_mut().find(|c| c.name == name)
    }

    fn add_child<T>(&mut self, leaf: T) -> &mut Self
    where
        T: Into<Self>,
    {
        self.children.push(leaf.into());
        self
    }
}

pub struct App {
    pub file_path: String,
    pub root: Node,
    pub tree_state: TreeState<String>,
    pub tree_items: Vec<TreeItem<'static, String>>,
    pub current_widget: CurrentWidget,
}

pub enum CurrentWidget {
    Tree,
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

        let tree_items = App::create_tree(&root);

        Ok(Self {
            file_path: path,
            root,
            tree_state: TreeState::default(),
            tree_items,
            current_widget: CurrentWidget::Tree,
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

    fn create_tree(root: &Node) -> Vec<TreeItem<'static, String>> {
        fn to_tree_item(node: &Node) -> TreeItem<'static, String> {
            let text = node.name.to_owned();
            let identifier = node.name.to_owned();

            if node.children.is_empty() {
                TreeItem::new_leaf(identifier, text)
            } else {
                TreeItem::new(identifier, text, parse_children(node)).unwrap()
            }
        }
        fn parse_children(node: &Node) -> Vec<TreeItem<'static, String>> {
            node.children.iter().map(to_tree_item).collect()
        }

        parse_children(root)
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
