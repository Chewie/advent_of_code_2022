use std::cell::RefCell;
use std::collections::BTreeMap;
use std::rc::{Rc, Weak};

pub struct FileSystem {
    root: Rc<RefCell<DirEntry>>,
}

struct DirEntry {
    parent: Weak<RefCell<DirEntry>>,
    subdirs: BTreeMap<String, Rc<RefCell<DirEntry>>>,
    files: BTreeMap<String, usize>,
}

impl FileSystem {
    pub fn from_string(input: &str) -> Self {
        let root = Rc::new(RefCell::new(DirEntry {
            parent: Weak::new(),
            subdirs: BTreeMap::new(),
            files: BTreeMap::new(),
        }));
        {
            let mut current = Rc::clone(&root);

            // First token is always empty, second is always "cd /"
            for token in input.split("$ ").skip(2) {
                match &token[..2] {
                    "ls" => Self::handle_ls(Rc::clone(&current), token),
                    "cd" => current = Self::get_new_dir(current, token),
                    _ => unreachable!("unknown command"),
                };
            }
        }
        FileSystem { root }
    }

    pub fn total_size_under_100k(&self) -> usize {
        let mut dirs = Vec::new();
        self.root
            .borrow()
            .all_dirs_sizes_under_threshold(&mut dirs, 100000);
        dirs.iter().sum()
    }

    pub fn smallest_dir_big_enough(&self) -> usize {
        let mut dirs = Vec::new();
        self.root
            .borrow()
            .all_dirs_sizes_under_threshold(&mut dirs, 70000000);

        // By recursion, the outer dir is the last appended
        let current_occupation = dirs.last().unwrap();

        dirs.iter()
            .copied()
            .filter(|size| current_occupation - size < 40000000)
            .min()
            .unwrap()
    }

    fn handle_ls(current_dir: Rc<RefCell<DirEntry>>, token: &str) {
        // First token is the "ls" command, we skip to the output
        for line in token.lines().skip(1) {
            let (left, right) = line.split_once(' ').unwrap();
            if left == "dir" {
                current_dir.borrow_mut().subdirs.insert(
                    right.to_string(),
                    Rc::new(RefCell::new(DirEntry {
                        parent: Rc::downgrade(&current_dir),
                        subdirs: BTreeMap::new(),
                        files: BTreeMap::new(),
                    })),
                );
            } else {
                current_dir
                    .borrow_mut()
                    .files
                    .insert(right.to_string(), left.parse().unwrap());
            }
        }
    }

    fn get_new_dir(current_dir: Rc<RefCell<DirEntry>>, token: &str) -> Rc<RefCell<DirEntry>> {
        let (_, arg) = token.trim().split_once(' ').unwrap();
        if arg == ".." {
            return current_dir.borrow().parent.upgrade().unwrap();
        } else {
            return Rc::clone(&current_dir.borrow().subdirs[arg]);
        }
    }
}

impl DirEntry {
    fn all_dirs_sizes_under_threshold(
        &self,
        good_dirs: &mut Vec<usize>,
        threshold: usize,
    ) -> usize {
        let files_size: usize = self.files.values().sum();
        let dirs_size: usize = self
            .subdirs
            .values()
            .map(|subdir| {
                subdir
                    .borrow()
                    .all_dirs_sizes_under_threshold(good_dirs, threshold)
            })
            .sum();

        let total_size = files_size + dirs_size;
        if total_size <= threshold {
            good_dirs.push(total_size);
        }
        total_size
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use indoc::indoc;

    impl FileSystem {
        fn dump(&self, writer: &mut impl std::fmt::Write) {
            self.root.borrow().dump(writer, "/", 0);
        }
    }

    impl DirEntry {
        fn dump(&self, writer: &mut impl std::fmt::Write, dir_name: &str, indent_size: usize) {
            let indent = " ".repeat(indent_size);
            writeln!(writer, "{indent}- {dir_name} (dir)").unwrap();
            for (name, size) in &self.files {
                writeln!(writer, "{indent}  - {name} (file, size={size})").unwrap();
            }
            for (child_name, child_entry) in &self.subdirs {
                child_entry
                    .borrow()
                    .dump(writer, &child_name, indent_size + 2);
            }
        }
    }

    #[test]
    fn filesystem_from_string_empty() {
        // GIVEN
        let input = indoc! {"
            $ cd /
            $ ls
            "};

        // WHEN
        let fs = FileSystem::from_string(input);

        // THEN
        let mut output = String::new();
        fs.dump(&mut output);
        assert_eq!(
            indoc! {"
                - / (dir)
            "},
            output
        );
    }

    #[test]
    fn filesystem_from_string_one_dir() {
        // GIVEN
        let input = indoc! {"
            $ cd /
            $ ls
            dir a
            "};

        // WHEN
        let fs = FileSystem::from_string(input);

        // THEN
        let mut output = String::new();
        fs.dump(&mut output);
        assert_eq!(
            indoc! {"
                - / (dir)
                  - a (dir)
            "},
            output
        );
    }

    #[test]
    fn filesystem_from_string_one_file() {
        // GIVEN
        let input = indoc! {"
            $ cd /
            $ ls
            42 toto.txt
            "};

        // WHEN
        let fs = FileSystem::from_string(input);

        // THEN
        let mut output = String::new();
        fs.dump(&mut output);
        assert_eq!(
            indoc! {"
                - / (dir)
                  - toto.txt (file, size=42)
            "},
            output
        );
    }

    #[test]
    fn filesystem_from_string_one_file_inside_subdir() {
        // GIVEN
        let input = indoc! {"
            $ cd /
            $ ls
            dir a
            $ cd a
            $ ls
            42 toto.txt
            "};

        // WHEN
        let fs = FileSystem::from_string(input);

        // THEN
        let mut output = String::new();
        fs.dump(&mut output);
        assert_eq!(
            indoc! {"
                - / (dir)
                  - a (dir)
                    - toto.txt (file, size=42)
            "},
            output
        );
    }

    #[test]
    fn filesystem_from_string_going_up() {
        // GIVEN
        let input = indoc! {"
            $ cd /
            $ ls
            dir a
            dir b
            $ cd a
            $ ls
            42 toto.txt
            $ cd ..
            $ cd b
            $ ls
            51 tata.txt
            "};

        // WHEN
        let fs = FileSystem::from_string(input);

        // THEN
        let mut output = String::new();
        fs.dump(&mut output);
        assert_eq!(
            indoc! {"
                - / (dir)
                  - a (dir)
                    - toto.txt (file, size=42)
                  - b (dir)
                    - tata.txt (file, size=51)
            "},
            output
        );
    }

    #[test]
    fn filesystem_from_string_complex() {
        // GIVEN
        let input = indoc! {"
            $ cd /
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
            7214296 k
            "};

        // WHEN
        let fs = FileSystem::from_string(input);

        // THEN
        let mut output = String::new();
        fs.dump(&mut output);
        assert_eq!(
            indoc! {"
            - / (dir)
              - b.txt (file, size=14848514)
              - c.dat (file, size=8504156)
              - a (dir)
                - f (file, size=29116)
                - g (file, size=2557)
                - h.lst (file, size=62596)
                - e (dir)
                  - i (file, size=584)
              - d (dir)
                - d.ext (file, size=5626152)
                - d.log (file, size=8033020)
                - j (file, size=4060174)
                - k (file, size=7214296)
            "},
            output
        );
    }

    #[test]
    fn filesystem_total_size_under_100k() {
        // GIVEN
        let input = indoc! {"
            $ cd /
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
            7214296 k
            "};
        let fs = FileSystem::from_string(input);

        // WHEN
        let total_size = fs.total_size_under_100k();

        // THEN
        assert_eq!(95437, total_size);
    }

    #[test]
    fn filesystem_smallest_dir_big_enough() {
        // GIVEN
        let input = indoc! {"
            $ cd /
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
            7214296 k
            "};
        let fs = FileSystem::from_string(input);

        // WHEN
        let total_size = fs.smallest_dir_big_enough();

        // THEN
        assert_eq!(24933642, total_size);
    }
}
