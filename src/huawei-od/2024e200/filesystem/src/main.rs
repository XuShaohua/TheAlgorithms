// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by General Public License that can be found
// in the LICENSE file.

use std::collections::{HashMap, HashSet};
use std::io::{stdin, BufRead};

/// 目录节点
pub struct FolderEntry {
    /// 当前目录的绝对路径
    path: String,
    /// 指向父节点
    parent: String,
    /// 子节点
    children: HashSet<String>,
}

pub type FolderMap = HashMap<String, FolderEntry>;

impl FolderEntry {
    #[must_use]
    #[inline]
    pub fn new(path: String, parent: String) -> Self {
        Self {
            path,
            parent,
            children: HashSet::new(),
        }
    }

    fn validate_folder_name(folder_name: &str) -> bool {
        for chr in folder_name.chars() {
            if !chr.is_ascii_lowercase() {
                return false;
            }
        }
        true
    }

    pub fn mkdir(&mut self, folder_name: String) -> Result<Self, String> {
        if !Self::validate_folder_name(&folder_name) {
            return Err(folder_name);
        }
        if self.children.contains(&folder_name) {
            return Err(folder_name);
        }

        let path = Self::to_path(&self.path, &folder_name);
        self.children.insert(folder_name);
        Ok(Self::new(path, self.path.clone()))
    }

    fn to_path(path: &str, folder_name: &str) -> String {
        format!("{path}{folder_name}/")
    }

    pub fn cd(&self, folder_name: &str) -> Option<String> {
        if folder_name == ".." {
            return Some(self.parent.clone());
        }
        if !Self::validate_folder_name(folder_name) {
            return None;
        }
        if self.children.contains(folder_name) {
            let path = Self::to_path(&self.path, &folder_name);
            Some(path)
        } else {
            None
        }
    }
}

fn solution() {
    let root = FolderEntry::new("/".to_owned(), "/".to_owned());
    let mut map = HashMap::new();
    let mut cwd = root.path.clone();
    map.insert(root.path.clone(), root);

    for line in stdin().lock().lines() {
        let line = line.unwrap();
        let line = line.trim_ascii();
        if line.is_empty() {
            continue;
        }
        let mut parts = line.split_ascii_whitespace();
        match parts.next() {
            Some("pwd") => {
                if parts.next().is_none() {
                    println!("pwd: {}", cwd);
                }
            }
            Some("cd") => {
                // 切换工作目录
                if let Some(folder_name) = parts.next() {
                    if !parts.next().is_none() {
                        continue;
                    }
                    if let Some(cwd_entry) = map.get_mut(&cwd) {
                        if let Some(new_folder_name) = cwd_entry.cd(folder_name) {
                            cwd = new_folder_name;
                        }
                    }
                }
            }
            Some("mkdir") => {
                // 创建子目录
                if let Some(folder_name) = parts.next() {
                    if !parts.next().is_none() {
                        continue;
                    }
                    if let Some(cwd_entry) = map.get_mut(&cwd) {
                        if let Ok(new_folder) = cwd_entry.mkdir(folder_name.to_owned()) {
                            map.insert(new_folder.path.clone(), new_folder);
                        }
                    }
                }
            }
            Some(_) | None => {}
        }
    }
}

fn main() {
    solution()
}
