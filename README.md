# asare
Asset allocation rebalancer.

# Backend
run for local dev:
```bash
cargo watch -x 'r --bin backend' -c
```

# Telegram Bot
for logs:
```bash
export RUST_LOG=info
```
See [pretty_env_logger](https://crates.io/crates/pretty_env_logger).

---
run for local dev:
```bash
cargo watch -x 'r --bin telegram_bot' -c
```