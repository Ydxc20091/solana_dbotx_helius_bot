# Solana DBotX Helius Bot 部署保姆级教程

本文档将带你一步一步在本地部署并运行 Solana DBotX Helius 交易机器人。

## 1. 环境准备

1. 安装 [Rust](https://www.rust-lang.org/)。推荐使用 rustup：
   ```bash
   curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
   source $HOME/.cargo/env
   ```
2. 安装构建所需的系统依赖（如已安装可忽略）：
   ```bash
   sudo apt-get update && sudo apt-get install -y pkg-config libssl-dev
   ```
3. 克隆项目代码：
   ```bash
   git clone https://github.com/your/repo.git
   cd solana_dbotx_helius_bot
   ```

## 2. 配置环境变量

项目使用 `.env` 文件管理配置。先复制示例文件：

```bash
cp .env.example .env
```

编辑 `.env` 并填入实际的 API Key、交易对地址等信息：

```env
DBOTX_API_KEY=真实的DBotX密钥
DBOTX_CHAIN=solana
DBOTX_PAIR=交易对地址
RPC_URL=https://mainnet.helius-rpc.com/?api-key=你的Helius密钥
# 其他变量同理填写
```

## 3. 运行机器人

1. 下载依赖并编译：
   ```bash
   cargo build --release
   ```
2. 运行默认的 DRY 模式：
   ```bash
   cargo run --release
   ```
3. 如需在主网下单，将 `EXEC_MODE` 设置为 `LIVE`：
   ```bash
   EXEC_MODE=LIVE cargo run --release
   ```

## 4. 常见问题

- 若程序提示缺少环境变量，请检查 `.env` 文件是否正确填写。
- 如需调试，请加上 `RUST_LOG=debug` 查看更详细日志。

至此，机器人即可在本地运行。如需在服务器部署，只需在服务器重复上述步骤，并确保网络和密钥安全。
