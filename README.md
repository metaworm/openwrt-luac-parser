
这是针对 openwrt 的一个 luac 修改版定制的parser，可以编译成WASM给[metaworm's luadec][luadec]加载使用来反编译openwrt的lua脚本

如果需要修改字节码的定义顺序，只需要修改[src/custom.rs](./src/custom.rs)文件里的`OPCODE_LIST`的排列顺序即可

## 构建

1. [安装Rust及其工具链](https://www.rust-lang.org/tools/install) `curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh`
2. 切换到 nightly 版本 `rustup default nightly`
3. 添加 wasm32 的编译工具 `rustup target add wasm32-unknown-unknown`
4. 安装 trunk 构建工具 `cargo install trunk`
5. 安装 wasm-bindgen `cargo install wasm-bindgen-cli`
6. 安装 wasm-opt `cargo install wasm-opt`
7. 构建 luac parser `trunk build --release`

构建成功以后会在 dist 目录下生成 `openwrt-luac-parser_bg.wasm`

在[metaworm's luadec][luadec]页面中点击`<Custom luac parser>`，选择此 `openwrt-luac-parser_bg.wasm` 即可使用这个parser来反编译 openwrt 的luac

[luadec]: http://luadec.metaworm.site/