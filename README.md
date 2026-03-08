## mcproto
[官方文档](https://theopenteam.github.io/doc/#/mcproto)    

### 使用
#### 安装
在你的`项目根目录`中使用以下命令：  
```bash
cargo add mcproto
```
或者在`Cargo.toml`中手动添加：
```toml
[dependencies]
mcproto = "0.1"
```
#### 示例
```rust
use mcproto::utils::{VarInt};
pub fn main() {
    let mut buffer = Vec::new();
    VarInt::write_to(&mut buffer, 114514);
    println!("{:?}", &buffer[..]); // 输出结果
}
```

### 鸣谢与其他
本项目使用[RustRover](https://www.jetbrains.com/zh-cn/rust/)开发，感谢[JetBrains](https://jb.gg/OpenSourceSupport)的支持。  
本项目欢迎来自任何形式的贡献。可以通过[Pull Request](https://github.com/TheOpenTeam/mcproto-rs/pulls)或[Issue](https://github.com/TheOpenTeam/mcproto-rs/issues)进行贡献，也可以为官方文档纠错或加以补充。  
注意，本项目**遵循MIT协议**，对于使用本项目用于商业、开源产品的，请署名。

