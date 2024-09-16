use std::{
    env::{self, current_dir},
    path::PathBuf,
};

use crate::Links;

pub fn link_transform(links: Links) -> Links {
    let transformer = get_link_transformer();
    links.iter().map(transformer).collect()
}

fn get_link_transformer() -> impl Fn(&(String, String)) -> (String, String) {
    let pwd = current_dir().unwrap();
    move |(source, target)| (handle_source(source, &pwd), handle_target(target, &pwd))
}

fn handle_source(source: &str, pwd: &PathBuf) -> String {
    pwd.join(source).to_str().unwrap().to_string()
}

fn handle_target(target: &str, _pwd: &PathBuf) -> String {
    handle_path_placeholder(target)
}

#[inline]
fn handle_path_placeholder(path: &str) -> String {
    if path.starts_with("$") {
        if path.starts_with("$HOME") {
            let mut res = env::var("HOME").unwrap();
            res.push_str(&path[5..]);
            return res;
        }
    }

    path.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_should_cocnat_pwd_in_handle_source() {
        let source = "source";
        let pwd = current_dir().unwrap();
        let expected = pwd.join(source).to_str().unwrap().to_string();
        assert_eq!(expected, handle_source(source, &pwd));
    }

    #[test]
    fn it_should_handle_home_in_handle_path_placeholder() {
        env::set_var("HOME", "/home/user");
        let path = "$HOME/path";
        let expected = "/home/user/path";
        assert_eq!(expected, handle_path_placeholder(path));
    }
}
