use lib::{io_utils::read_input_for_day, tree::Tree};

fn main() {
    let input = read_input_for_day(7);
    let commands = parse_input(input);

    let fs = create_fs(commands);

    println!("fs: {:?}", fs);
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
    File { size: usize, name: String },
    Dir(String),
}

#[derive(Debug)]
struct FileSystem(Tree<Directory>);

#[derive(Debug)]
struct Directory {
    name: String,
    files: Vec<File>,
}

impl Directory {
    fn new(name: String) -> Self {
        Self {
            files: vec![],
            name,
        }
    }

    fn add_file(&mut self, file: File) {
        self.files.push(file);
    }
}

#[derive(Debug)]
struct File {
    name: String,
    size: usize,
}

impl File {
    fn new(name: String, size: usize) -> Self {
        Self { name, size }
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
                curr_dir = t.get_node(&curr_dir).unwrap().parent.unwrap();
            }
            Command::List(outputs) => {
                for output in outputs {
                    match output {
                        SingleListOutput::File { size, name } => {
                            t.update_node(curr_dir, &|node| {
                                node.data.add_file(File::new(name.clone(), size));
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
                let ls_output_iter = input.to_owned().take_while(|l| !l.starts_with("$"));
                let mut outputs = vec![];
                for out in ls_output_iter {
                    let mut out_iter = out.split_ascii_whitespace();
                    let first_token = out_iter.next().unwrap();
                    match first_token {
                        "dir" => outputs
                            .push(SingleListOutput::Dir(out_iter.next().unwrap().to_string())),
                        size => outputs.push(SingleListOutput::File {
                            size: size.parse().unwrap(),
                            name: out_iter.next().unwrap().to_string(),
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
