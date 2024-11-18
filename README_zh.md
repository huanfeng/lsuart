# lsuart

[English](README.md) | [简体中文](README_zh.md)

## 项目简介

lsuart 是一个基于 Rust 的串口获取程序。该程序使用了 `serialport` 和 `clap` 两个主要依赖库，提供了简单易用的命令行接口来获取系统中的串口。

## 技术框架

该项目使用了以下技术框架和库：

- **Rust**: 作为主要编程语言，提供了高性能和内存安全的保证。
- **serialport**: 用于串口通信的库，版本为 4.5.0。
- **clap**: 用于解析命令行参数的库，版本为 4.5.17，并启用了 `derive` 特性。

## 安装

1. 确保已安装 Rust 开发环境
2. 克隆项目并编译：

```bash
git clone https://github.com/huanfeng/lsuart.git
cd lsuart
cargo build --release
```

## 使用方法

### 基本命令

`lsuart [OPTIONS]`

### 可用选项

- `--sort <SORT>`: 排序模式 [默认值: 1]
  - 0: 默认排序
  - 1: 按端口号排序

- `-u, --usb-only`: 仅显示 USB 串口设备
- `--pci-only`: 仅显示 PCI 串口设备
- `--bt-only`: 仅显示蓝牙串口设备
- `-v, --verbose`: 显示详细信息
- `-h, --help`: 显示帮助信息
- `-V, --version`: 显示版本信息

### 示例

1. 列出所有串口设备：
```bash
lsuart
```

2. 仅显示 USB 设备并按端口号排序：
```bash
lsuart --usb-only --sort 1
```

3. 显示详细的设备信息：
```bash
lsuart --verbose
```

## 输出说明

程序会显示找到的串口设备列表，包含以下信息：

- 端口名称
- 设备类型（USB/PCI/蓝牙）
- 详细模式下的额外信息（仅 USB 设备）：
  - 供应商 ID (VID)
  - 产品 ID (PID)
  - 序列号
  - 制造商
  - 产品名称

## 贡献

欢迎提交 Issue 和 Pull Request！

## 许可证

MIT License
