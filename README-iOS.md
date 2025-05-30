# iOS 部署指南

本文档介绍如何将 Always Blue 博客应用部署到 iOS 设备上。

## 前置要求

1. **macOS 系统**：iOS 开发需要在 macOS 上进行
2. **Xcode**：从 App Store 安装最新版本的 Xcode
3. **Rust 工具链**：
   ```bash
   # 安装 Rust
   curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
   
   # 安装 iOS 目标平台
   rustup target add aarch64-apple-ios
   rustup target add x86_64-apple-ios
   ```
4. **Dioxus CLI**：
   ```bash
   cargo install dioxus-cli
   ```

## 快速开始

### 1. 使用构建脚本

我们提供了一个便捷的构建脚本：

```bash
./build-ios.sh
```

这个脚本会自动：
- 检查并安装必要的工具
- 安装 iOS 目标平台
- 清理之前的构建
- 构建 iOS 应用

### 2. 手动构建

如果你更喜欢手动操作：

```bash
# 清理之前的构建
cargo clean

# 构建 iOS 应用
dx build --platform ios --release
```

### 3. 运行在模拟器

```bash
# 在 iOS 模拟器中运行
dx serve --platform ios
```

## 配置说明

### Dioxus 配置 (Dioxus.toml)

```toml
[bundle.ios]
development_team = "YOUR_TEAM_ID"  # 替换为你的 Apple Developer Team ID
minimum_version = "11.0"
```

### iOS Info.plist

位于 `ios/Info.plist`，包含了：
- 应用名称和标识符
- 支持的设备方向
- 本地化支持 (中文/英文)
- 自动主题支持

## 移动端优化

### 响应式设计
- 使用 Tailwind CSS 的响应式类名 (`sm:`, `lg:` 等)
- 优化触摸目标大小 (最小 44px)
- 支持横屏和竖屏显示

### iOS 特定优化
- Safe Area 支持
- 触摸反馈动画
- iOS 原生滚动行为
- 自动主题切换

### 移动端导航
- 响应式导航栏
- 移动端折叠菜单
- 触摸友好的按钮尺寸

## 发布到 App Store

### 1. 开发者账号
注册 Apple Developer 账号并获取 Team ID。

### 2. 配置签名
在 Xcode 中配置代码签名：
1. 打开生成的 Xcode 项目
2. 选择项目 -> Signing & Capabilities
3. 选择你的开发团队
4. 配置 Bundle Identifier

### 3. 构建和上传
```bash
# 构建 Release 版本
dx build --platform ios --release

# 在 Xcode 中 Archive 并上传到 App Store Connect
```

## 调试技巧

### 查看日志
```bash
# 查看 iOS 模拟器日志
xcrun simctl spawn booted log stream --predicate 'process == "always-blue"'
```

### 常见问题

1. **构建失败 - 缺少 iOS 工具链**
   ```bash
   rustup target add aarch64-apple-ios x86_64-apple-ios
   ```

2. **Xcode 项目无法打开**
   - 确保 Xcode 已正确安装
   - 检查 Dioxus CLI 版本

3. **应用无法在设备上运行**
   - 检查开发者证书配置
   - 确认设备已添加到开发者账号

## 性能优化

### 二进制大小优化
在 `Cargo.toml` 中已配置：
```toml
[profile.release]
opt-level = 'z'
lto = true
codegen-units = 1
strip = true
```

### 启动时间优化
- 延迟加载非关键组件
- 优化资源加载
- 使用代码分割

## 更多资源

- [Dioxus 官方文档](https://dioxuslabs.com/learn/0.6/)
- [iOS 开发者文档](https://developer.apple.com/documentation/)
- [Rust iOS 开发指南](https://mozilla.github.io/firefox-browser-architecture/experiments/2017-09-21-rust-on-ios.html)
