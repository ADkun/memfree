use std::io;
use std::process::{Command, Stdio};

/// 执行一个外部命令
pub fn run(exe: &str, args: &[&str]) -> Result<String, io::Error> {
    let output = Command::new(exe)
        .args(args)
        .stdin(Stdio::null())
        .stdout(Stdio::piped())
        .output()?;
    if !output.status.success() {
        let fail_reason = String::from_utf8_lossy(&output.stderr).to_string();
        return Err(io::Error::new(io::ErrorKind::Other, fail_reason));
    }
    Ok(String::from_utf8_lossy(&output.stdout).to_string())
}

/// 获取当前进程的父目录
pub fn get_exe_parent_dir() -> String {
    let mut path = std::env::current_exe().unwrap();
    path.pop();
    path.to_str().unwrap().to_string()
}
