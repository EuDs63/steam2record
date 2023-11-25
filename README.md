# STEAM2RECORD

## 使用
1. 使用[Steam library filters](https://www.lorenzostanco.com/lab/steam/)获取csv文件
2. 重命名为`steam-library.csv`
3. 修改`config.toml.example`为对应的配置，并重名为`config.toml`
4. `neodb_token`可参考[NeoDB 获取 Access Token-大大的小蜗牛](https://eallion.com/neodb_token)
5. 运行

## 说明
本项目使用搜索功能，默认选择搜索中的第一条结果。所以会出现不匹配的情况，运行完毕后，请对结果进行进一步的检查。


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

## todo
- [ ] 重构代码：每次循环不应都要重新判断
- [ ] 支持同步到Bangumi
- [ ] 对搜索结果进行判断，降低不匹配率

## 参考链接
- [Why rust is failing to build command for openssl-sys v0.9.60 even after local installation? - Stack Overflow](https://stackoverflow.com/questions/65553557/why-rust-is-failing-to-build-command-for-openssl-sys-v0-9-60-even-after-local-in)
- [(2) Error - reqwest json response to get : rust --- (2) 错误 - reqwest json 响应获取：rust](https://www.reddit.com/r/rust/comments/gg98m0/error_reqwest_json_response_to_get/)