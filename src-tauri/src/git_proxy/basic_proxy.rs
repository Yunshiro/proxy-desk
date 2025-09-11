use std::process::Command;

#[tauri::command]
pub fn set_http_proxy(proxy_url: &str, is_https: bool) {
    // select the proxy type, use https or http
    let proxy_type = if is_https {
        "https.proxy"
    } else {
        "http.proxy"
    };
    
    let mut git_command = Command::new("git");

    // git config --global https.proxy "socks5://127.0.0.1:7897"
    git_command.args(
        ["config", "--global", proxy_type, proxy_url]
    ).status().expect("failed to process the command");
}

fn show_proxy_url(is_https: bool) -> String {
    let proxy_type = if is_https {
        "https.proxy"
    } else {
        "http.proxy"
    };
    // git config --global https.proxy
    let output = Command::new("git").args(["config", "--global", proxy_type])
        .output().expect("read system git proxy config error.");

    let result = output.stdout;
    let result_str = std::str::from_utf8(&result).expect("invalid UTF-8");
    
    String::from(result_str)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let proxy_url = "socks5://127.0.0.1:7897";
        let is_https = false;
        set_http_proxy(proxy_url, is_https);
        let now_proxy_url = show_proxy_url(is_https);

        println!("proxy_url: {:?}", proxy_url);
        println!("now_proxy_url: {:?}", now_proxy_url.trim());

        assert!(now_proxy_url.trim() == proxy_url);
    }
}