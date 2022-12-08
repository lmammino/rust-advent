use std::{
    collections::HashMap,
    ops::{Deref, DerefMut},
    path::{Path, PathBuf},
};

use crate::parser::{Cmd, Line, Out};

#[derive(Debug, PartialEq, Clone)]
pub(crate) struct File {
    pub name: String,
    pub size: usize,
}

#[derive(Debug, PartialEq, Clone)]
pub(crate) struct Fs(HashMap<PathBuf, Vec<File>>);

impl Deref for Fs {
    type Target = HashMap<PathBuf, Vec<File>>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for Fs {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl Fs {
    fn new() -> Self {
        Fs(HashMap::new())
    }

    fn update_file_size(&mut self, path: &Path) {
        let mut current_path = PathBuf::from(path);
        while let Some(parent_path) = current_path.parent() {
            let folder_name = current_path.file_name().unwrap().to_str().unwrap();
            let files_size = self
                .get(&current_path)
                .unwrap()
                .iter()
                .map(|f| f.size)
                .sum();
            let mut file_in_parent = self
                .get_mut(parent_path)
                .unwrap()
                .iter_mut()
                .find(|f| f.name == folder_name)
                .unwrap();
            file_in_parent.size = files_size;
            current_path.pop();
        }
    }

    pub fn get_folders_size(&self) -> HashMap<PathBuf, usize> {
        let mut folders_size: HashMap<PathBuf, usize> = HashMap::new();
        for (path, files) in self.iter() {
            folders_size.insert(path.clone(), files.iter().map(|f| f.size).sum());
        }

        folders_size
    }

    pub fn used_space(&self) -> usize {
        self.get(Path::new("/"))
            .unwrap()
            .iter()
            .map(|f| f.size)
            .sum()
    }
}

impl<'a> FromIterator<Line<'a>> for Fs {
    fn from_iter<T: IntoIterator<Item = Line<'a>>>(iter: T) -> Self {
        let mut fs = Fs::new();
        let mut curr_path = PathBuf::from("/");
        for cmd in iter {
            match cmd {
                Line::CmdLine(Cmd::Cd(dir)) => {
                    match dir {
                        "/" => {}
                        ".." => {
                            curr_path.pop();
                        }
                        folder_name => curr_path.push(folder_name),
                    };
                }
                Line::CmdLine(Cmd::Ls) => {
                    // we can ignore ls commands and just process the output lines
                }
                Line::OutLine(Out::Dir(name)) => {
                    // A foldered is considered like a file of size 0 (size will be calculated later)
                    let files = fs.entry(curr_path.clone()).or_default();
                    files.push(File {
                        name: name.to_string(),
                        size: 0,
                    });
                }
                Line::OutLine(Out::File(size, name)) => {
                    let files = fs.entry(curr_path.clone()).or_default();
                    files.push(File {
                        name: name.to_string(),
                        size,
                    });
                }
            }
        }

        let mut summed = fs.clone();
        for (path, _files) in fs.iter_mut() {
            if path.to_str().unwrap() != "/" {
                summed.update_file_size(path);
            }
        }

        summed
    }
}
