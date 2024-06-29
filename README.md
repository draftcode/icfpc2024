# icfpc2024

## Toolchains

Rust も Python もこの Toolchain インストーラーでなんとかしましょう。

- https://rustup.rs/
- https://rye.astral.sh/

## Pre-commit

Git の pre-commit hook で formatter をかけてしましましょう。

```bash
rye install pre-commit
pre-commit install
```

これで git commit ごとに formatter がかかります。 https://pre-commit.com/hooks.html
にいろいろ hook があるので、ほしいものがあれば.pre-commit-config.yaml を編集してく
ださい。
