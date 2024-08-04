use std::path::PathBuf;

mod tool;

const EXE_NAME: &str = "EmptyStandbyList.exe";
const ARG_LIST: [&str; 4] = [
    "workingsets",
    "modifiedpagelist",
    "standbylist",
    "priority0standbylist",
];

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let parent_dir = tool::get_exe_parent_dir();
    let mut exe = PathBuf::from(parent_dir);
    exe.push(EXE_NAME);

    match tool::run(exe.to_str().unwrap(), &ARG_LIST[..]) {
        Ok(_) => println!("Your memory has been freed!"),
        Err(e) => {
            eprintln!("Error: {}", e);
            wait_input();
        }
    }

    Ok(())
}

fn wait_input() {
    println!("Press any key to exit...");
    std::io::stdin().read_line(&mut String::new()).unwrap();
}
