/*
When parsing, do I want each node to parse its own data? Or do I want a single function
that simply manages the state in a for loop? That might be easiest.
*/

#[derive(Debug, PartialEq, Eq)]
enum Entry {
    Dir {
        full_path: String,
        children: Vec<Entry>,
    },
    File {
        full_path: String,
        size: usize,
    },
}

impl Entry {
    // fn parse(input_str: &str) {
    //     let lines: Vec<&str> = input_str.lines().collect();
    //     let mut fs = Entry::Dir {
    //         full_path: "/".to_string(),
    //         children: vec![],
    //     };

    //     // Use a vec as a stack to keep track of which Dir we are currently on
    //     let mut dir_stack: Vec<Entry> = vec![fs];

    //     // Skip the first line bc we've already created the root entry
    //     for line in lines.iter().skip(1) {
    //         if let Entry::Dir {
    //             full_path,
    //             children,
    //         } = dir_stack.last_mut().expect("No last directory")
    //         {}

    //         let mut parts = line.split_ascii_whitespace();
    //         let first_word = parts.next().expect("Could not get first item in line");
    //         /*
    //         The cases are as follows:
    //         - "$" means a command.
    //             - "cd" means move to the next word
    //             - "ls" means basically get ready to add things to the current Dir
    //         - "(\d+)" means this is a file, and the next word is the name of the file.
    //                 Add this file to the current Dir
    //         - "dir" means create a new empty Dir
    //         */
    //         if first_word
    //             .chars()
    //             .next()
    //             .expect("No first letter")
    //             .is_ascii_digit()
    //         {
    //             match dir_stack.last_mut().expect("No last directory") {
    //                 Entry::Dir {
    //                     full_path,
    //                     children,
    //                 } => {
    //                     let filename = parts.next().expect("No filename");
    //                     let mut fp = full_path.clone();
    //                     fp.push('/');
    //                     fp.push_str(filename);
    //                     children.push(Entry::File {
    //                         full_path: fp,
    //                         size: first_word.parse().expect("Could not parse into usize"),
    //                     })
    //                 }
    //                 Entry::File {
    //                     full_path: _,
    //                     size: _,
    //                 } => unreachable!("Should never be trying to add a file to a file"),
    //             }
    //         } else if first_word.eq("dir") {
    //             let mut new_dir = parts.next().expect("No directory name").to_string();
    //             new_dir.push(ch)
    //         }
    //     }
    // }

    /// Calculates the size of this entry.
    /// If this is a File, then simply returns the size
    /// If this is a Dir, then recursively calculates the size of all children
    fn get_size(&self) -> usize {
        match self {
            Entry::Dir {
                full_path: _,
                children,
            } => children.iter().map(Entry::get_size).sum(),
            Entry::File { full_path: _, size } => *size,
        }
    }
}

fn main() {
    println!("hello");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse() {
        let input_str = "$ cd /
$ ls
dir a
14848514 b.txt
8504156 c.dat
dir d
$ cd a
$ ls
dir e
29116 f
2557 g
62596 h.lst
$ cd e
$ ls
584 i
$ cd ..
$ cd ..
$ cd d
$ ls
4060174 j
8033020 d.log
5626152 d.ext
7214296 k";
        let want = Entry::Dir {
            full_path: "/".to_string(),
            children: vec![
                Entry::Dir {
                    full_path: "/a".to_string(),
                    children: vec![
                        Entry::Dir {
                            full_path: "/a/e".to_string(),
                            children: vec![Entry::File {
                                full_path: "/a/e/i".to_string(),
                                size: 584,
                            }],
                        },
                        Entry::File {
                            full_path: "/a/f".to_string(),
                            size: 29116,
                        },
                        Entry::File {
                            full_path: "/a/g".to_string(),
                            size: 2557,
                        },
                        Entry::File {
                            full_path: "/a/h.lst".to_string(),
                            size: 62596,
                        },
                    ],
                },
                Entry::File {
                    full_path: "/b.txt".to_string(),
                    size: 14848514,
                },
                Entry::File {
                    full_path: "/c.dat".to_string(),
                    size: 8504156,
                },
                Entry::Dir {
                    full_path: "/d".to_string(),
                    children: vec![
                        Entry::File {
                            full_path: "/d/j".to_string(),
                            size: 4060174,
                        },
                        Entry::File {
                            full_path: "/d/d.log".to_string(),
                            size: 8033020,
                        },
                        Entry::File {
                            full_path: "/d/d.ext".to_string(),
                            size: 5626152,
                        },
                        Entry::File {
                            full_path: "/d/k".to_string(),
                            size: 7214296,
                        },
                    ],
                },
            ],
        };
        // let got = Entry::parse(input_str);
        // assert_eq!(want, got);
    }
}
