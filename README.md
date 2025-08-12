# Solana DBotX Helius Bot

一个基于 Rust 的原型交易机器人，用于演示如何从 [DBotX](https://dbotx.com/) API 获取 Solana 链的 1 分钟 K 线数据，检测成交量突增（"volume dump"）信号，并通过 [Helius](https://www.helius.xyz/) RPC 执行交易。

> **注意**：项目仍处于早期阶段，尚未覆盖全部计划功能，仅供学习和二次开发参考。

## 功能概览

- 调用 DBotX K 线接口，实时拉取市场数据。
- 根据成交量突增判断交易信号。
- 提供两种执行模式：
  - **DRY**：仅打印拟执行的订单，不发送到链上；
  - **LIVE**：通过 Jupiter 执行真实交易（需正确配置 RPC 和密钥）。
- 支持可选的 RSI 和布林带过滤（可通过命令行或环境变量启用）。

## 环境准备

1. 安装 [Rust 开发环境](https://www.rust-lang.org/)，推荐使用 `rustup`：
   ```bash
   curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
   source $HOME/.cargo/env
   ```
2. 克隆项目代码：
   ```bash
   git clone https://github.com/your/repo.git
   cd solana_dbotx_helius_bot
   ```

## 配置环境变量

项目使用 `.env` 文件管理配置。先复制示例文件：

```bash
cp .env.example .env
```

编辑 `.env` 并填入实际的 API Key、交易对地址等信息：

```env
DBOTX_API_KEY=...
DBOTX_KLINE_URL="https://api-data-v1.dbotx.com/kline/chart?chain={chain}&pair={pair}&interval={interval}&end={end}"
DBOTX_CHAIN=solana
DBOTX_PAIR=PAIR_ADDRESS
RPC_URL=https://mainnet.helius-rpc.com/?api-key=...
PUBLIC_KEY=...
SECURE_CODE=...
BASE=TOKEN
QUOTE=SOL
ORDER_SIZE_SOL=0.5
```

更多可选参数：

- `EXEC_MODE`：执行模式（`dry`/`live`/`backtest`），默认为 `dry`。
- `RSI_FILTER`：是否启用 RSI 过滤，默认为 `true`。
- `BB_FILTER`：是否启用布林带过滤，默认为 `false`。

## 编译与运行

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

## 更多文档

- 详细的中文部署指南请参阅 [`docs/deploy.zh.md`](docs/deploy.zh.md)。

## 免责声明

本项目仅用于学习和示例，**不构成投资建议**。使用者需自行承担使用本项目产生的任何风险。

