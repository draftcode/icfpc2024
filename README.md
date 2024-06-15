# icfpc2024

## Toolchains

RustもPythonもこのToolchainインストーラーでなんとかしましょう。

* https://rustup.rs/
* https://rye.astral.sh/

## Pre-commit

Gitのpre-commit hookでformatterをかけてしましましょう。

```
rye install pre-commit
pre-commit install
```

これでgit commitごとにformatterがかかります。 https://pre-commit.com/hooks.html
にいろいろhookがあるので、ほしいものがあれば.pre-commit-config.yamlを編集してく
ださい。
