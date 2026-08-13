# 时契 TimePact

本地优先的 Windows 待办与专注计时器，使用 Tauri 2、Vue 3、Rust 和 SQLite 构建。

## 下载与安装

发布到 GitHub 后，在仓库右侧打开 **Releases**，进入最新版并下载：

```text
TimePact_<版本号>_x64-setup.exe
```

安装包使用当前用户模式，默认安装到英文目录 `%LOCALAPPDATA%\TimePact`，不需要管理员权限。TimePact 当前没有购买 Windows 代码签名证书，因此其他电脑第一次运行时可能看到 SmartScreen 提示；请只从自己的 GitHub Releases 页面下载。

## 开发

需要 Node.js 20+、Rust stable MSVC、Visual Studio 2022 C++ Build Tools 和 WebView2。

```powershell
npm install
npm run test:run
npm run build
npm run tauri dev
```

## 生成安装包

```powershell
npm run tauri build
```

NSIS 安装包会输出到 `src-tauri/target/release/bundle/nsis/`。首版使用当前用户安装模式，不需要管理员权限。

## 发布到 GitHub

项目包含 `.github/workflows/release.yml`。首次发布前，登录 GitHub CLI，并用 GitHub 的隐私邮箱记录提交，避免把当前电脑配置的公司邮箱公开到仓库历史中：

```powershell
gh auth login
$githubLogin = gh api user --jq .login
$githubId = gh api user --jq .id
git config user.name $githubLogin
git config user.email "$githubId+$githubLogin@users.noreply.github.com"
git add .
git commit -m "feat: publish TimePact"
gh repo create TimePact --public --source . --remote origin --push
```

如果只想让登录了自己 GitHub 账号的电脑下载，可把 `--public` 改为 `--private`；希望任何电脑直接打开链接下载，就保留公开仓库。

后续发布新版本时，先同步修改 `package.json`、`src-tauri/Cargo.toml` 和 `src-tauri/tauri.conf.json` 中的版本号并提交，然后推送同版本标签：

```powershell
git tag v0.1.5
git push origin main
git push origin v0.1.5
```

GitHub Actions 会在 Windows 构建机上运行前端与 Rust 测试，生成 NSIS 安装包，并自动创建对应的 GitHub Release。标签版本必须与应用配置版本一致；例如应用版本 `0.1.5` 使用标签 `v0.1.5`。

## 数据

SQLite 数据库、7 日自动快照和 7 日本地日志保存在系统应用数据目录的 `com.timepact.desktop` 文件夹。应用不包含账号、云同步或遥测。

产品约束见 [PRODUCT.md](PRODUCT.md)，实现计划见 [docs/superpowers/plans/2026-07-30-timepact-desktop.md](docs/superpowers/plans/2026-07-30-timepact-desktop.md)。
