# CCE - Claude Config Environment 使用指南（中文）
[English Version](README.md)

CCE 是一个使用 Rust 编写的 Claude 服务提供方配置切换工具，可帮助你在本地安全管理多个 Claude API 账号，并在终端中快速切换相关环境变量。

## ✨ 主要特性

- **极速切换**：一条命令即可在不同 Claude 服务提供方之间切换。
- **多账号配置**：将账号信息保存在本地 `~/.cce/config.toml`，安全可控。
- **即刻生效**：配合 shell 集成，`cce use` / `cce clear` 能立即更新当前终端环境变量。
- **跨平台**：兼容 macOS、Linux（bash/zsh）、Windows PowerShell。

## 🚀 快速入门

```bash
# 方式一：curl 安装（推荐）
curl -sSL https://raw.githubusercontent.com/zhaopengme/cce/master/install.sh | bash

# 方式二：源码构建
git clone https://github.com/zhaopengme/cce.git
cd cce
cargo build --release
cargo install --path .
```

### Windows PowerShell

```powershell
Invoke-WebRequest -Uri "https://raw.githubusercontent.com/zhaopengme/cce/master/install.ps1" -OutFile "install.ps1"
.\install.ps1
```

安装完成后，重新打开一个终端（或执行 `source ~/.zshrc` / `source ~/.bashrc`，PowerShell 下执行 `. $PROFILE`）即可生效。

## 🔧 Shell 集成

### 自动集成（推荐）

```bash
cce install
```

该命令会：
- 自动识别当前 shell（bash 或 zsh）
- 将 CCE 集成片段追加到对应的 profile
- 在每次打开新终端时，根据 `current_provider` 自动加载环境变量
- 包装 `cce` 函数，使 `cce use` / `cce clear` 立即更新当前会话

### 手动集成示例

若使用其它 shell，可手动在配置文件中加入：

```bash
# ~/.zshrc 或 ~/.bashrc
if command -v cce >/dev/null 2>&1; then
  if [[ -f "$HOME/.cce/config.toml" ]]; then
    current_provider=$(awk -F'"' '/^current_provider/ {print $2; exit}' "$HOME/.cce/config.toml")
    if [[ -n "$current_provider" ]]; then
      eval "$(CCE_SHELL_INTEGRATION=1 cce use "$current_provider")"
    fi
  fi
fi

# 可选：加载 cce shell 包装函数
eval "$(cce shellenv)"
```

PowerShell 用户可参考 `install.ps1` 自动生成的片段，它会放置在 `$PROFILE` 中并在会话启动时自动执行。

## 📋 常用命令

| 命令 | 说明 |
|------|------|
| `cce list` | 列出已配置的服务提供方并标记当前使用者 |
| `cce add <name> <api_url> <token> [--model <model>]` | 新增或更新服务提供方 |
| `cce delete <name>` | 删除指定服务提供方 |
| `cce use <name>` | 切换到指定服务提供方 |
| `cce clear` | 清空当前服务提供方，恢复官方客户端 |
| `cce check` | 检查环境变量与配置是否一致 |
| `cce install [--force]` | 安装（或强制重装）shell 集成 |
| `cce shellenv` | 输出 bash/zsh 包装函数定义 |

> 提示：在脚本场景下，可使用 `CCE_SHELL_INTEGRATION=1 cce use <name>` / `CCE_SHELL_INTEGRATION=1 cce clear` 来获取 `export` / `unset` 命令并自动生效。

## 🐛 故障排查

### 安装失败
- 确认 `curl`、`tar` 已安装；
- 检查平台是否受支持（`uname -s && uname -m`）；
- 可改用源码构建或从 Release 页面下载二进制。

### Shell 集成未生效
- 重新执行 `cce install --force`；
- 确认 profile 中存在 “CCE Shell Integration” 区块：
  ```bash
  grep -n "CCE Shell Integration" ~/.zshrc
  ```
- 当前终端临时刷新：
  ```bash
  eval "$(CCE_SHELL_INTEGRATION=1 cce use <provider-name>)"
  ```

### 环境变量未设置
```bash
cce check
echo $ANTHROPIC_AUTH_TOKEN
echo $ANTHROPIC_BASE_URL
```

### PATH 未包含安装目录
```bash
echo 'export PATH="$HOME/.local/bin:$PATH"' >> ~/.zshrc
source ~/.zshrc
```

## 📄 配置文件位置

CCE 的配置文件保存在 `~/.cce/config.toml`，主要字段包括：

```toml
current_provider = "anthropic"

[providers.anthropic]
name = "anthropic"
api_url = "https://api.anthropic.com"
token = "sk-ant-api03-your-token-here"

[providers.custom]
name = "custom"
api_url = "https://custom-claude-api.com"
token = "custom-token-123"
```

可以通过 `cce add` 命令添加多个 provider，并使用 `cce use <name>` 激活其中之一。

## 🤝 反馈与贡献

如需反馈问题或提交改进，欢迎访问 GitHub 项目仓库提交 Issue 或 Pull Request。感谢使用 CCE，祝你切换 Claude 账号畅通无阻！
