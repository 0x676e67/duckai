# duckai

![Crates.io License](https://img.shields.io/crates/l/duckai)
![crates.io](https://img.shields.io/crates/v/duckai.svg)
![Crates.io Total Downloads](https://img.shields.io/crates/d/duckai)

DuckDuckGo AI to OpenAI

- `API`身份验证
- 支持`IP`代理池
- 流式/非流式`API`

## 模型

模型映射，不支持的模型默认为`gpt-4o-mini`

- gpt-4o-mini -> `(gpt-4o-mini)`
- claude-3-haiku -> `(claude-3-haiku-20240307)`
- llama-3.1-70b -> `(meta-llama/Meta-Llama-3.1-70B-Instruct-Turbo)`
- mixtral-8x7b -> `(mistralai/Mixtral-8x7B-Instruct-v0.1)`

## 对话

```bash
curl --request POST 'http://127.0.0.1:8080/v1/chat/completions' \
  --header 'Content-Type: application/json' \
  --data '{
    "messages": [
      {
        "role": "user",
        "content": "Rust example."
      }
    ],
    "model": "gpt-4o-mini",
    "stream": true
  }'
```

## 命令

```bash
$ duckai -h
DuckDuckGo AI to OpenAI

Usage: duckai
       duckai <COMMAND>

Commands:
  run      Run server
  start    Start server daemon
  restart  Restart server daemon
  stop     Stop server daemon
  log      Show the server daemon log
  ps       Show the server daemon process
  gt       Generate config template file (yaml format file)
  help     Print this message or the help of the given subcommand(s)

Options:
  -h, --help     Print help
  -V, --version  Print version

$ duckai run -h
Run server

Usage: duckai run [CONFIG_PATH]

Arguments:
  [CONFIG_PATH]  Configuration filepath [default: duckai.yaml]

Options:
  -h, --help  Print help
```

## 安装

<details>

<summary>如果您需要更详细的安装和使用信息，请查看此处</summary>

1. 安装

- cargo

```bash
cargo install vproxy
```

- Dokcer

```bash
docker run --rm -it -p 8080:8080 ghcr.io/penumbra-x/duckai:latest run
```

2. 使用

- 生成配置模版

```bash
duckai gt # 生成duckai.yaml文件（当前目录）
```

```yaml
# 调试模式
debug: false

# 监听地址
bind: 0.0.0.0:8080

# 客户端超时
timeout: 60

# 客户端连接超时
connect_timeout: 10

# 客户端 tcp keepalive
tcp_keepalive: 90

# 最大 tcp 连接
concurrent: 100

# 代理池
proxies:
- !url http://127.0.0.1:6152
- !url socks5://127.0.0.1:6153
- !cidr 2001:470:e953::/48
- !iface 192.168.1.10

# 启用 TLS
tls_cert: null
tls_key: null

# 验证 api 密钥
api_key: null
```

3. 代理池

`IP`代理池类型支持三种类型（优先级：`CIDR` > `Proxy` > `Interface`，使用轮训策略）:

- `URL`，协议支持：`http`/`https`/`socks4`/`socks5`/`socks5h`
- `Interface`，即绑定本地网络接口地址
- `CIDR`，支持`IPv4`/`IPv6`子网，前提是子网路由正常通信

</details>

## 贡献

如果您想提交贡献，请打开 [Pull Request](https://github.com/penumbra-x/duckai/pulls)

## 获取帮助

您的问题可能已在 [issues](https://github.com/penumbra-x/duckai/issues) 中得到解答

## 赞助商

如果您觉得这个项目有帮助，请考虑赞助我以支持持续开发：

**USDT-TRC20**: TCwD8HfHnJ7236Hdj3HF5uZKR2keeWeqZe
