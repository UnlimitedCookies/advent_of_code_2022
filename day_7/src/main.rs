#[allow(unused)]
#[derive(Debug)]
struct Node<T>
where T: PartialEq {
    index: usize,
    value: T,
    parent: Option<usize>,
    children: Vec<usize>,
}

impl<T> Node<T>
where T: PartialEq {
    fn new(index: usize, value: T) -> Self {
        Self {
            index,
            value,
            parent: None,
            children: Vec::new(),
        }
    }
}

#[derive(Debug, Default)]
struct ArenaTree<T>
where T: PartialEq {
    arena: Vec<Node<T>>,
}

impl<T> ArenaTree<T>
where T: PartialEq {
    fn node(&mut self, value: T) -> usize {
        let index = self.arena.len();
        self.arena.push(Node::new(index, value));
        index
    }

    fn insert(&mut self, value: T, parent: usize) -> usize {
        let new_node = self.node(value);
        self.arena[parent].children.push(new_node);
        self.arena[new_node].parent = Some(parent); 
        new_node       
    }

    fn init(&mut self, value: T) -> usize {
        let new_node = self.node(value);
        new_node
    }
}

#[derive(Debug, PartialEq)]
enum FileTreeStructure {
    Directory { name: String },
    File { name: String, file_size: u32 },
}

impl Default for FileTreeStructure {
    fn default() -> Self {
        FileTreeStructure::Directory {name: "/".to_string() }
    }
}

fn sum(file_tree: &ArenaTree<FileTreeStructure>, node: &Node<FileTreeStructure>) -> u32 {
    match &node.value {
        FileTreeStructure::File{ name: _, file_size } => *file_size,
        FileTreeStructure::Directory { .. } => {
            node.children.iter().map(|child| sum(file_tree, &file_tree.arena[*child])).sum()
        },
    }
}

fn main() {
    let input = std::fs::read_to_string("input").unwrap();
    let mut file_tree: ArenaTree<FileTreeStructure> = ArenaTree::default();
    let mut pwd_ref = file_tree.init(FileTreeStructure::default());

    // parse the input
    input.lines().skip(1).for_each(|line|{
        if line.starts_with("$ cd ..") {
            pwd_ref = file_tree.arena[pwd_ref].parent.unwrap();
        } else if line.starts_with("$ cd ") {
            pwd_ref = file_tree.insert(FileTreeStructure::Directory{ name: line[5..].to_string() }, pwd_ref);
        } else if !line.starts_with("$ ls") && !line.starts_with("dir") {
            let (file_size, file_name) = line.split_once(" ").unwrap();
            file_tree.insert(FileTreeStructure::File{ name: file_name.to_string(), file_size: file_size.parse().unwrap() }, pwd_ref);
        }
    });

    // first challenge
    let filtered_dir_size_sum: u32 = file_tree.arena.iter().filter_map(|file_tree_element| match &file_tree_element.value {
        FileTreeStructure::Directory { .. } => {
            match sum(&file_tree, file_tree_element) {
                dir_size if dir_size <= 100000 => Some(dir_size),
                _ => None,
            }
        },
        _ => None,
    }).sum();
    println!("Sum of all directories that contain at most 100000: {filtered_dir_size_sum}");

    // second challenge
    let space_needed = sum(&file_tree, &file_tree.arena[0]) - 40000000;
    let smallest_removable_dir = file_tree.arena.iter().filter_map(|file_tree_element| match &file_tree_element.value {
        FileTreeStructure::Directory { name } => {
            match sum(&file_tree, file_tree_element) {
                dir_size if dir_size >= space_needed => Some((name, dir_size)),
                _ => None,
            }
        },
        _ => None,
    }).min_by_key(|dir| dir.1).unwrap();
    println!("Smallest directory that may be deleted for the update is \"{}\" with a size of {}", smallest_removable_dir.0, smallest_removable_dir.1);

}
