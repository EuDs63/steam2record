# STEAM2RECORD

## 踩坑
- `failed to run custom build command for openssl-sys v0.9.60`,
    - 解决，执行以下命令:
    ```bash
    sudo apt install pkg-config
    sudo apt-get install libudev-dev
    ```
- wsl可用内存过少，导致频繁崩溃
    - 解决:
    修改`.wslconfig`,示例
    ```
    [wsl2]
    memory=4GB
    swap=4GB
    localhostForwarding=true
    ```
- no method named `json` found for `reqwest` in the current scope
    - 解决:
    修改`Cargo.toml`对应内容为：
    `reqwest = { version = "0.11", features = ["json"] }`

## 参考链接
- [Why rust is failing to build command for openssl-sys v0.9.60 even after local installation? - Stack Overflow](https://stackoverflow.com/questions/65553557/why-rust-is-failing-to-build-command-for-openssl-sys-v0-9-60-even-after-local-in)
- [(2) Error - reqwest json response to get : rust --- (2) 错误 - reqwest json 响应获取：rust](https://www.reddit.com/r/rust/comments/gg98m0/error_reqwest_json_response_to_get/)