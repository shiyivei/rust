**测试某个crate**

```
cargo test -p pallet-difttt
```

**运行某个crate**

```
cargo test -p adder
```

# 1 关于Rust的版本问题

```
rustc --version //查看版本
rustup update //更新所有版本
rustup toolchain list //查看所有已经安装的版本
rustup install nightly  //安装nightly版本
rustup default nightly/stable 更新当前版本为不同的版本
```

# 2 rust analyzer 启动耗时久问题

```
rm -rf ~/.cargo/.package-cache
```

