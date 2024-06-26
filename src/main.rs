// 一、题意

// - 使用 go(或rust) 语言调用 pcre2 (源码地址：https://github.com/PhilipHazel/pcre2)，必须亲自动手使用 FFI 绑定 C 接口，不得使用现成的三方库
// - 从 **目标文本** 中筛选出符合 **筛选规则** 的 **结果字符串**，
// - 将 **结果字符串** 发送给一个 bash 脚本，
// - 在这个 bash 脚本中接收并打印接收到的结果，
// - 转输过程中要求使用 UDP 协议。

// 二、目标文本

// "a;jhgoqoghqoj0329 u0tyu10hg0h9Y0Y9827342482y(Y0y(G)_)lajf;lqjfgqhgpqjopjqa=)*(^!@#$%^&*())9999999"

// 三、筛选规则

// 1. **结果字符串** 自身不包含数字和任何类型的空白字符（如空格、回车等等），其长度为 3 至 11 个字符
// 2. **结果字符串** 左侧相邻的字符串是4个数字
// 3. **结果字符串** 右侧相邻的字符串不为空
// 4. 正则匹配的次数越少越好，尽可能只使用一个正则表达式

// 注：以上 4 条规则须同时满足。

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
        let data = match_str; //  数据发送
        socket.send_to(data.as_bytes(), target_addr)?;
    }

    Ok(())
}
