## 使脚本可执行:

```bash 
chmod +x receive_and_print.sh
```
## 启动脚本:

运行bash脚本开始监听传入的UDP数据。将listen_udp.sh替换为你脚本的文件名。

```bash 
./receive_and_print.sh
```
Bash脚本现在应该正在运行，并在指定的UDP端口上等待数据。

## 运行Rust程序

```bash
cargo run
```
Rust程序将使用pcre2库执行正则表达式匹配，并将结果发送到bash脚本正在监听的UDP端口。然后，bash脚本会打印接收到的数据。
