use std::collections::HashSet;

pub fn part1(input: String) {
    let ans = process(&input)
        .get_all_children()
        .iter()
        .filter(|x| x.is_dir && x.size <= 100000)
        .fold(0, |acc, node| acc + node.size)
        .to_string();

    println!("{}", ans);
}

pub fn part2(input: String) {
    let root_node = process(&input);
    let target_size = 30000000 - (70000000 - root_node.size);
    let nodes = root_node.get_all_children();

    let node = nodes.iter().fold(None, |acc: Option<&FileTreeNode>, node| {
        if !node.is_dir {
            return acc;
        }
        if let Some(acc) = acc {
            if node.size >= target_size && node.size < acc.size {
                Some(node)
            } else {
                Some(acc)
            }
        } else if node.size >= target_size {
            Some(node)
        } else {
            None
        }
    });

    println!("{:?}", node.unwrap().size);
}

fn process(input: &str) -> FileTreeNode {
    let mut root_node = FileTreeNode::new("/", 0);
    let mut history = Vec::new();

    for line in input.lines() {
        let words = line.split_whitespace().collect::<Vec<_>>();
        if words[..2] == ["$", "cd"] {
            match words[2] {
                "/" => continue,
                ".." => {
                    history.pop().unwrap();
                    continue;
                }
                _ => {}
            }
            let parent = root_node.walk(&history);
            history.push(words[2].to_owned());
            if parent.children.iter().any(|x| x.name == words[2]) {
                continue;
            }
            let mut dir = FileTreeNode::new(words[2], 0);
            dir.is_dir = true;
            parent.children.push(dir);
        } else if words[0] == "dir" {
            let parent_node = root_node.walk(&history);
            let mut child = FileTreeNode::new(words[1], 0);
            child.is_dir = true;
            parent_node.children.push(child);
        } else if let Ok(size) = words[0].parse::<usize>() {
            let child = FileTreeNode::new(words[1], size);
            root_node.walk(&history).children.push(child);
        }
    }
    root_node.propagate_size();

    root_node
}

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
struct FileTreeNode {
    name: String,
    size: usize,
    children: Vec<FileTreeNode>,
    is_dir: bool,
}

impl FileTreeNode {
    fn new(name: &str, size: usize) -> Self {
        Self {
            name: name.to_string(),
            size,
            children: Vec::new(),
            is_dir: false,
        }
    }

    fn walk(&mut self, history: &[String]) -> &mut Self {
        let mut current_node = self;
        for node_name in history {
            current_node = current_node
                .children
                .iter_mut()
                .find(|f| f.name == *node_name)
                .unwrap();
        }

        current_node
    }

    fn propagate_size(&mut self) -> usize {
        for i in &mut self.children {
            self.size += i.propagate_size();
        }

        self.size
    }

    fn get_all_children(&self) -> HashSet<Self> {
        let mut out = HashSet::new();

        for i in &self.children {
            out.insert(i.clone());
            out.extend(i.get_all_children());
        }

        out
    }
}
