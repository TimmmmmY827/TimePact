# 时契 TimePact

> 本地优先的 Windows 待办与专注计时器 —— 把时间承诺记录下来，到期提醒，专注投入，延期与完成都进入统计。

时契是一款桌面工作台，帮你管理带时间约束的待办（Deadline 或倒计时）与番茄式专注会话。所有数据只存在你的电脑上，不需要账号、云同步或联网。技术栈为 Tauri 2、Vue 3、Rust 与 SQLite。

## 为什么

多数待办工具要么把一切塞进日历，要么只做轻量清单，时间承诺被淹没在视图里。时契反其道而行：首页只有一列按紧迫度排序的进行中待办，没有看板、没有仪表盘装饰。带计时的待办到期会用独立置顶窗口集中提醒，专注时屏幕边缘浮现一圈呼吸光晕，让你自然回到任务。

## 功能

**待办与提醒**
- 字段：标题（必填）、备注、优先级、标签，以及可选的 Deadline 或倒计时（两者互斥）。
- Deadline 一经设置，原始截止时间不可修改；到期后的推延以事件记录，不覆盖原始截止时间。
- 倒计时启动后锁定初始时长；暂停、休眠或系统退出期间不继续流逝。
- 到期使用独立置顶提醒窗口，关闭它不会改变待办状态。
- 支持提前完成、暂停/继续、10/30 分钟或自定义时长推延；多个计时器可并行。

**专注**
- 默认专注 25 分钟、休息 5 分钟，可自定义。
- 可关联一个待办，也可自由专注。
- 每轮结束后停在确认状态，由你决定开始休息、继续下一轮或停止。
- 支持暂停、提前结束、跳过休息；同一时间仅运行一个专注会话。
- 运行专注时屏幕边缘出现绿色呼吸光晕 overlay。

**统计与数据**
- 统计区间支持周、月、季度和自定义日期，可导出 CSV。
- 完成率、延期次数、累计延期、生命周期耗时和专注投入均进入归档与统计。
- 支持 JSON 原子备份/恢复、每日自动快照（保留最近 7 天）。
- 默认开机启动并驻留系统托盘，设置中可关闭。

## 技术栈

- **前端**：Vue 3 + Pinia + Vue Router（hash 模式），Vite 构建。
- **后端**：Rust + Tauri 2，SQLite（rusqlite bundled）持久化，原生窗口/托盘/提醒。
- **平台**：Windows 10 22H2 / Windows 11 x64（WebView2）。架构上保留迁移 macOS 的空间。
- **隐私**：不包含账号、云同步或遥测。数据库、快照与日志只写入本机应用数据目录。

## 下载安装

在仓库右侧 **Releases** 打开最新版，下载：

```text
TimePact_<版本号>_x64-setup.exe
```

安装包为当前用户模式，默认安装到 `%LOCALAPPDATA%\TimePact`，不需要管理员权限。TimePact 未购买代码签名证书，其他电脑首次运行可能看到 SmartScreen 提示；请只从自己的 GitHub Releases 页面下载。

数据保存在 `%APPDATA%\com.timepact.desktop`（数据库 `timepact.db`、`snapshots/`、`logs/`）。

## 开发

需要 Node.js 20+、Rust stable (MSVC)、Visual Studio 2022 C++ Build Tools 和 WebView2。

```powershell
npm install
npm run test:run      # 单元测试（vitest）
npm run build         # 类型检查 + 前端构建
npm run tauri dev     # 启动开发版
```

生成安装包：

```powershell
npm run tauri build   # NSIS 包输出到 src-tauri/target/release/bundle/nsis/
```

发布新版本：同步修改 `package.json`、`src-tauri/Cargo.toml`、`src-tauri/tauri.conf.json` 的版本号后提交，推送同名标签（如应用版本 `0.1.5` → 标签 `v0.1.5`），GitHub Actions 会自动构建并创建 Release。

## 更多

- 产品约束与功能细节：[PRODUCT.md](PRODUCT.md)
- 设计规范与视觉系统：[DESIGN.md](DESIGN.md)
