use home::home_dir;
use std::{
    env::{current_dir, var},
    path::Path,
};

fn get_args_path(args: &Vec<String>) -> Option<String> {
    let user_path = args.get(1);
    if user_path.is_none() {
        return None;
    }
    let user_path = user_path.unwrap();
    if !Path::new(user_path).exists() {
        eprintln!("Defined path \"{}\" does not exist", user_path);
        std::process::exit(1);
    }
    Some(String::from(user_path))
}

fn get_system_path() -> String {
    String::from("/etc/localbang.csv")
}

fn get_config_dir_path() -> Option<String> {
    match var("XDG_CONFIG_HOME") {
        Ok(xdg_config_home) => Some(String::from(format!("{}/localbang.csv", xdg_config_home))),
        Err(_) => None,
    }
}

fn get_home_dir_path() -> Option<String> {
    match home_dir() {
        Some(home_dir) => Some(format!("{}/.localbang.csv", home_dir.display())),
        None => None,
    }
}

fn get_current_dir_path() -> Option<String> {
    match current_dir() {
        Ok(current_dir) => Some(String::from(format!(
            "{}/localbang.csv",
            current_dir.display()
        ))),
        Err(_) => None,
    }
}

pub fn find_shortcuts_path(args: &Vec<String>) -> Option<String> {
    let possible_paths: [Option<String>; 5] = [
        get_args_path(&args),
        Some(get_system_path()),
        get_config_dir_path(),
        get_home_dir_path(),
        get_current_dir_path(),
    ];

    for path in possible_paths.iter() {
        match path {
            Some(path) => {
                if Path::new(&path).exists() {
                    return Some(String::from(path));
                }
            }
            None => continue,
        };
    }

    None
}
