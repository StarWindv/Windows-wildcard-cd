# Windows 通配符目录切换工具

`rcd` 是一个用于 Windows 系统的通配符目录切换工具，允许你使用通配符来匹配并切换到指定的目录。

## 功能特性
- 支持使用通配符（如 `*` 和 `?`）来匹配目录名。
- 可以在命令行中快速切换到匹配的目录。
- 支持 PowerShell 和 CMD 环境。

## 安装步骤

### 1. 编译 Rust 项目
首先，确保你已经安装了 Rust 开发环境。然后，在项目根目录下运行以下命令来编译项目：
```bash
cargo build --release
```
这将在 `target/release` 目录下生成 `rcd.exe` 可执行文件。

### 2. 优化可执行文件
编译完成后，你可以使用 `strip` 命令来优化可执行文件的大小：
```bash
strip .\target\release\rcd.exe
```

### 3. 迁移可执行文件
将编译并优化后的 `rcd.exe` 文件迁移到系统环境变量 `PATH` 可以访问的位置，例如 `C:\Windows\System32` 或自定义的 `bin` 目录。

### 4. 迁移批处理文件
将 `cmd/rscd.bat` 文件迁移到系统环境变量 `PATH` 可以访问的位置，确保在 CMD 中也能使用该工具。

### 5. 配置 PowerShell 环境
运行 `install_ps1.py` 脚本来配置 PowerShell 环境：
```bash
python install_ps1.py
```
该脚本会自动更新 PowerShell 的配置文件，使其支持 `rcd` 命令。

## 使用方法

### 在 CMD 中使用
在 CMD 中，你可以直接使用 `rscd` 命令来切换目录：
```bash
rscd <目录通配符模式>
```
例如，如果你想切换到名称以 `test` 开头的目录，可以使用以下命令：
```bash
rscd test*
```

### 在 PowerShell 中使用
在 PowerShell 中，同样可以使用 `rscd` 命令来切换目录：
```powershell
rscd <目录通配符模式>
```
例如：
```powershell
rscd test*
```

## 通配符说明
- `*`：匹配任意数量的任意字符。
- `?`：匹配单个任意字符。

例如，`rscd test*` 可以匹配 `test`、`test123`、`test_folder` 等目录；`rscd test?` 可以匹配 `test1`、`testA` 等目录，但不能匹配 `test` 或 `test123`。

## 错误处理
如果指定的目录不存在或无法访问，工具会输出错误信息并退出：
```
错误: 目录不存在或无法访问
```

## 依赖项
该项目依赖于以下 Rust 库：
- `clap`：用于解析命令行参数。
- `walkdir`：用于遍历目录树。
- `regex`：用于处理通配符匹配。

这些依赖项会在 `cargo build` 时自动下载和安装。
