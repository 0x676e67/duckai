# duckai

![Crates.io License](https://img.shields.io/crates/l/duckai)
![crates.io](https://img.shields.io/crates/v/duckai.svg)
![Crates.io Total Downloads](https://img.shields.io/crates/d/duckai)

> 🚀 Support my journey to full-time open-source development by [sponsoring me on GitHub](https://github.com/penumbra-x/.github/blob/main/profile/SPONSOR.md)

DuckDuckGo AI to OpenAI API

- API authentication
- Support IP proxy pool
- Built-in Http connection pool
- Streaming/non-streaming API

## Model

Model mapping, unsupported models default to `gpt-4o-mini`

- gpt-4o-mini -> `gpt-4o-mini`
- claude-3-haiku -> `claude-3-haiku-20240307`
- llama-3.1-70b -> `meta-llama/Meta-Llama-3.1-70B-Instruct-Turbo`
- mixtral-8x7b -> `mistralai/Mixtral-8x7B-Instruct-v0.1`

## Chat

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

## Command

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

## Install

<details>

<summary>If you need more detailed installation and usage information, please check here</summary>

1. Install

- cargo

```bash
cargo install duckai
```

- Dokcer

```bash
docker run --rm -it -p 8080:8080 ghcr.io/penumbra-x/duckai:latest run
```

- Compile

```bash
# Required install docker
cargo install cross
cross build --target x86_64-unknown-linux-musl --release
cross build --target aarch64-unknown-linux-musl --release
```

2. Generate config template file

```bash
duckai gt # Generate duckai.yaml file (current directory)
```

```yaml
# Debug mode
debug: false

# Listen address
bind: 0.0.0.0:8080

# Client timeout
timeout: 60

# Client connect timeout
connect_timeout: 10

# Client tcp keepalive
tcp_keepalive: 90

# Maximum tcp connection
concurrent: 100

# Proxy pool
proxies:
- !url http://127.0.0.1:6152
- !url socks5://127.0.0.1:6153
- !cidr 2001:470:e953::/48
- !iface 192.168.1.10

# Enable TLS
tls_cert: null
tls_key: null

# API key
api_key: null
```

3. Proxy pool

`IP` proxy pool type supports three types (priority: `CIDR` > `Proxy` > `Interface`, using round-robin strategy):

- `URL`，protocol supports: `http`/`https`/`socks4`/`socks5`/`socks5h`
- `Interface`，bind local network interface address
- `CIDR`，support `IPv4`/`IPv6` subnet, the premise is that the subnet routes are normally communicable

</details>

## Contribution

If you want to submit contributions, please open [Pull Request](https://github.com/penumbra-x/duckai/pulls)

## Get help

Your questions may have been answered in [issues](https://github.com/penumbra-x/duckai/issues)
