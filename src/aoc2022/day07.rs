use std::collections::HashMap;

use crate::utils::advent;

use scan_fmt::scan_fmt;

pub struct Solver;

impl Solver {
    fn size(pwd: &str, dirs: &HashMap<String, (Vec<String>, HashMap<String, i32>)>) -> i32 {
        let (subdirs, files) = dirs.get(pwd).unwrap();
        let files_size = files.values().sum();
        if subdirs.is_empty() {
            return files_size;
        }
        let subdirs_size = subdirs
            .into_iter()
            .map(|subdir| Solver::size(&subdir, dirs))
            .sum::<i32>();
        subdirs_size + files_size
    }

    fn dirs(input: &str) -> HashMap<String, (Vec<String>, HashMap<String, i32>)> {
        let mut path = Vec::new();
        let mut dirs = HashMap::new();
        dirs.insert("/".to_string(), (Vec::new(), HashMap::new()));

        for cmd in input.trim().lines() {
            if cmd.starts_with("$ cd ..") {
                path.pop();
            } else if cmd.starts_with("$ cd") {
                let dir = scan_fmt!(cmd, "$ cd {}", String).unwrap();
                path.push(dir);
                let pwd = path.join("/");
                assert!(dirs.contains_key(&pwd), "{:?} {:?}", pwd, path);
            } else if cmd.starts_with("$ ls") {
                continue;
            } else if cmd.starts_with("dir") {
                let pwd = path.join("/");
                let dir = scan_fmt!(cmd, "dir {}", String).unwrap();
                let dir = format!("{}/{}", pwd, dir);
                dirs.insert(dir.clone(), (Vec::new(), HashMap::new()));
                let (subdirs, _) = dirs.get_mut(&pwd).unwrap();
                subdirs.push(dir);
            } else {
                let pwd = path.join("/");
                let (size, name) = scan_fmt!(cmd, "{d} {}", i32, String).unwrap();
                let (_, files) = dirs.get_mut(&pwd).unwrap();
                files.insert(name, size);
            }
        }
        dirs
    }
}

impl advent::Solver<2022, 7> for Solver {
    type Part1 = i32;
    type Part2 = i32;

    fn solve_part_one(&self, input: &str) -> Self::Part1 {
        let dirs = Solver::dirs(input);
        dirs.keys()
            .map(|dir| Solver::size(dir, &dirs)) // perf: memoize this
            .filter(|sz| *sz < 100000)
            .sum()
    }

    fn solve_part_two(&self, input: &str) -> Self::Part2 {
        let dirs = Solver::dirs(input);
        let unused = 70000000 - Solver::size("/", &dirs);
        let min = 30000000 - unused;

        dirs.keys()
            .map(|dir| Solver::size(dir, &dirs))
            .filter(|sz| *sz > min)
            .min()
            .unwrap()
    }
}