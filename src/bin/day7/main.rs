use lib::{io_utils::read_input_for_day, tree::Tree};

fn main() {
    println!("Part One: {}", part_one());
    println!("Part Two: {}", part_two());
}

fn part_one() -> usize {
    let input = read_input_for_day(7);
    let commands = parse_input(input);

    let fs = create_fs(commands);

    fs.0.into_iter()
        .filter(|dir| dir.size <= 100_000)
        .map(|dir| dir.size)
        .sum()
}

const FILE_SYSTEM_SIZE: usize = 70000000;
const MIN_SIZE_TO_FREE: usize = 30000000;

fn part_two() -> usize {
    let input = read_input_for_day(7);
    let commands = parse_input(input);

    let fs = create_fs(commands);

    let root_dir_size = fs.0.into_iter().map(|dir| dir.size).max().unwrap();
    let difference = MIN_SIZE_TO_FREE - (FILE_SYSTEM_SIZE - root_dir_size); // this will overflow if there is already that space available

    fs.0.into_iter()
        .map(|dir| dir.size)
        .filter(|size| size >= &difference)
        .min()
        .unwrap()
}

#[derive(Debug)]
enum Command {
    ChangeDir(String),
    MoveToRootDir,
    MoveUp,
    List(Vec<SingleListOutput>),
}

#[derive(Debug)]
enum SingleListOutput {
    File { size: usize },
    Dir(String),
}

#[derive(Debug)]
struct FileSystem(Tree<Directory>);

#[derive(Debug)]
struct Directory {
    name: String,
    size: usize,
    files: Vec<File>,
}

impl Directory {
    fn new(name: String) -> Self {
        Self {
            files: vec![],
            name,
            size: 0,
        }
    }

    fn add_file(&mut self, file: File) {
        self.size += file.size;
        self.files.push(file);
    }
}

#[derive(Debug)]
struct File {
    size: usize,
}

impl File {
    fn new(size: usize) -> Self {
        Self { size }
    }
}

fn create_fs(commands: Vec<Command>) -> FileSystem {
    let mut t = Tree::default();
    let root_dir = t.add_node(None, Directory::new("/".to_string()));
    let mut curr_dir = root_dir;

    for cmd in commands {
        match cmd {
            Command::ChangeDir(dir_name) => {
                for child in t.get_node(&curr_dir).unwrap().children.iter() {
                    if t.get_node(child).unwrap().data.name == dir_name {
                        curr_dir = *child;
                        break;
                    }
                }
            }
            Command::MoveToRootDir => {
                curr_dir = root_dir;
            }
            Command::MoveUp => {
                let curr_node = t.get_node(&curr_dir).unwrap();
                let parent_id = curr_node.parent.unwrap();
                let child_size = curr_node.data.size;

                // don't update parent if it is the root since
                // we update the root at the end
                if parent_id != root_dir {
                    t.update_node(parent_id, &|parent| {
                        parent.data.size += child_size;
                    });
                }

                curr_dir = parent_id;
            }
            Command::List(outputs) => {
                for output in outputs {
                    match output {
                        SingleListOutput::File { size } => {
                            t.update_node(curr_dir, &|node| {
                                node.data.add_file(File::new(size));
                            });
                        }
                        SingleListOutput::Dir(dir_name) => {
                            t.add_node(Some(curr_dir), Directory::new(dir_name));
                        }
                    }
                }
            }
        }
    }

    let root_children_sizes: Vec<usize> = t
        .get_node(&root_dir)
        .unwrap()
        .children
        .iter()
        .map(|c| t.get_node(c).unwrap().data.size)
        .collect();

    // update the root
    t.update_node(root_dir, &|root| {
        root_children_sizes.iter().for_each(|size| {
            root.data.size += size;
        });
    });
    FileSystem(t)
}

fn parse_input(input: String) -> Vec<Command> {
    let mut input = input.lines();
    let mut res = vec![];

    while let Some(line) = input.next() {
        let mut cmd_iter = line.split_ascii_whitespace();
        match cmd_iter.nth(1).unwrap() {
            "cd" => match cmd_iter.next().unwrap() {
                "/" => res.push(Command::MoveToRootDir),
                ".." => res.push(Command::MoveUp),
                dir_name => res.push(Command::ChangeDir(dir_name.to_string())),
            },
            "ls" => {
                let ls_output_iter = input.to_owned().take_while(|l| !l.starts_with('$'));
                let mut outputs = vec![];
                for out in ls_output_iter {
                    let mut out_iter = out.split_ascii_whitespace();
                    let first_token = out_iter.next().unwrap();
                    match first_token {
                        "dir" => outputs
                            .push(SingleListOutput::Dir(out_iter.next().unwrap().to_string())),
                        size => outputs.push(SingleListOutput::File {
                            size: size.parse().unwrap(),
                        }),
                    };
                    input.next();
                }
                res.push(Command::List(outputs));
            }
            sth_else => panic!("Undexpected token {}", sth_else),
        }
    }

    res
}

#[cfg(test)]
mod tests {
    use crate::{part_one, part_two};

    #[test]
    fn test_part_one() {
        assert_eq!(part_one(), 2031851);
    }

    #[test]
    fn test_part_two() {
        assert_eq!(part_two(), 2568781);
    }
}
