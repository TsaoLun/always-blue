# Always Blue

一个使用 Rust 和 Slint 构建的个人主页，其中包含一个隐藏的跨平台记忆配对游戏（彩蛋），支持桌面和 WebAssembly (WASM) 部署。

## 🎮 简介

Always Blue 是一个个人展示站点，包含一个作为彩蛋的海洋主题记忆配对游戏。项目包含：

- **8 种可爱的海洋生物**：鱼、章鱼、螃蟹、水母、海星、海龟、鲸鱼、海马
- **跨平台支持**：桌面应用和网页版本
- **音频系统**：背景音乐和音效
- **响应式界面**：适配不同屏幕尺寸

## 🚀 快速开始

### 桌面版本

```bash
# 克隆项目
git clone <repository-url>
cd always-blue

# 构建并运行（默认使用desktop特性，可手动修改为wasm)
cargo run
# 或显式指定特性
cargo run --features desktop
```

### Web 版本

```bash
# 构建 WASM（使用wasm特性）
./build.sh

# 启动本地服务器
deno task start
# 或使用其他静态文件服务器
```

然后在浏览器中访问 `http://localhost:8000`

## 🛠️ 技术栈

- **Rust** - 系统编程语言
- **Slint** - 声明式 UI 框架
- **WASM** - WebAssembly 支持
- **Rodio** - 桌面音频库
- **Web Audio API** - 网页音频

## 📁 项目结构

```
always-blue/
├── src/
│   ├── main.rs          # 桌面版本入口点
│   ├── lib.rs           # WASM 版本入口点
│   ├── audio.rs         # 跨平台音频管理
│   └── game/            # 游戏逻辑
│       ├── mod.rs       # 游戏初始化
│       └── tiles.rs     # 卡片生成
├── ui/
│   ├── main-window.slint # 主窗口定义
│   ├── app-window.slint  # 游戏窗口定义
│   └── icons/           # 游戏图标
├── raw/                 # 音频资源
├── pkg/                 # WASM 构建输出
└── build.sh             # 构建脚本
```

## 🔧 构建说明

### 开发构建

```bash
# 桌面版本
cargo build --features desktop

# WASM版本（本地检查）
cargo build --features wasm --no-default-features
```

### 发布构建

```bash
# 桌面版本
cargo build --release --features desktop

# WASM版本（使用build.sh脚本）
./build.sh
```

### WASM 构建

```bash
# 完整构建（包含优化和压缩）
./build.sh

# 或手动构建
wasm-pack build --target web --out-dir pkg --release -- --features wasm --no-default-features
```

## 🌐 部署到 Web

构建完成后，`pkg` 目录包含：

- `always_blue_wasm.js` - JavaScript 胶水代码
- `always_blue_wasm_bg.wasm` - WASM 二进制文件
- `always_blue_wasm.js.br` / `always_blue_wasm_bg.wasm.br` - Brotli 压缩版本

将 `pkg` 目录、`index.html` 和 `raw` 目录一起部署到静态文件服务器。

## 🎵 音频功能

游戏包含完整的音频系统：

- **背景音乐**：游戏运行时播放
- **匹配音效**：成功配对时播放
- **跨平台支持**：
  - 桌面：使用 Rodio 库
  - 网页：使用 Web Audio API

## 🎨 UI 特性

- **响应式设计**：适配不同屏幕尺寸
- **平滑动画**：卡片翻转和状态切换
- **主题系统**：蓝色海洋主题
- **交互反馈**：悬停效果和点击反馈

## 📱 平台支持

- **桌面**：Windows, macOS, Linux
- **网页**：现代浏览器（Chrome, Firefox, Safari, Edge）
- **移动端**：通过浏览器支持

## 🔍 开发指南

### 添加新卡片

1. 在 `ui/icons/` 目录添加新的 PNG 图标
2. 在 `src/game/tiles.rs` 中添加新的 `TileData` 条目
3. 确保图标路径正确

### 修改音频

1. 将音频文件放入 `raw/` 目录
2. 在 `src/audio.rs` 中更新文件路径
3. 支持格式：MP3, WAV, OGG

### 自定义主题

编辑 `ui/` 目录中的 `.slint` 文件来修改：
- 颜色方案
- 布局
- 字体
- 动画

## 🐛 故障排除

### 常见问题

1. **WASM 构建失败**
   - 确保已安装 `wasm-pack`
   - 检查 Rust 工具链：`rustup target add wasm32-unknown-unknown`

2. **音频无法播放**
   - 检查音频文件路径
   - 网页版本需要 HTTPS 或 localhost 才能播放音频

3. **图标不显示**
   - 检查图标文件是否存在
   - 确认文件路径大小写正确

### 调试

```bash
# 详细构建输出
cargo build --verbose --features desktop

# WASM 调试构建
wasm-pack build --target web --out-dir pkg --dev -- --features wasm --no-default-features

# 检查WASM代码（不实际构建WASM）
cargo check --features wasm --no-default-features
```

## 📄 许可证

本项目基于 MIT 许可证开源。详见 [LICENSE](LICENSE) 文件。

## 🤝 贡献

欢迎提交 Issue 和 Pull Request！

### 开发指南

项目使用特性（features）来区分不同平台：
- `desktop`：桌面版本特性（默认启用）
- `wasm`：WebAssembly版本特性

开发时请注意：
1. 使用 `#[cfg(feature = "desktop")]` 和 `#[cfg(feature = "wasm")]` 进行条件编译
2. 桌面和WASM版本共享大部分游戏逻辑
3. 平台特定的代码（如音频、文件访问）需要分别实现

### 贡献步骤
1. Fork 项目
2. 创建功能分支
3. 提交更改
4. 推送到分支
5. 创建 Pull Request

## 🙏 致谢

- [Slint UI](https://slint.rs/) - 优秀的 Rust UI 框架
- [Rust 社区](https://www.rust-lang.org/) - 强大的工具和库
- 所有贡献者和用户

---

**海洋世界等你来探索！** 🌊🐠