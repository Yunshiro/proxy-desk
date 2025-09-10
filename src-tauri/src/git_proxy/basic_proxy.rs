use std::process::Command;



#[tauri::command]
pub fn set_http_proxy() {
    let output = if cfg!(target_os = "windows") {
        Command::new("cmd")
        .args(["/C", "echo hello"])
        .output()
        .expect("failed to execute") 
    } else {
    Command::new("sh")
            .arg("-c")
            .arg("echo hello")
            .output()
            .expect("failed to execute process")
    };

    let hello = output.stdout;

    let hello_str = String::from_utf8(hello).expect("Invalid UTF-8");

    println!("result: {}", hello_str)
}