use std::net::UdpSocket;

use safe_ffi::Pcre2Regex;

mod ffi;
mod safe_ffi;

fn main() -> std::io::Result<()> {
    let socket = UdpSocket::bind("0.0.0.0:0")?; // 绑定到任意可用地址和端口
    let target_addr = "127.0.0.1:34254"; // 目标地址和端口，这里以34254为例

    let target_text = "a;jhgoqoghqoj0329 u0tyu10hg0h9Y0Y9827342482y(Y0y(G)_)lajf;lqjfgqhgpqjopjqa=)*(^!@#$%^&*())9999999";
    let regex_pattern = r"(?<=\d{4})[^\s\d]{3,11}(?=[^\s])";

    let regex = Pcre2Regex::new(regex_pattern).expect("Failed to compile regex");
    let matches = regex.find_all_matches(target_text);

    println!("Found matches:");
    for match_str in matches {
        println!("{}", match_str);
        let data = match_str; // 这里是你想发送的数据
        socket.send_to(data.as_bytes(), target_addr)?;
    }

    Ok(())
}
