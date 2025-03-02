# fuver

[![GitHub license](https://img.shields.io/badge/license-MIT-green.svg)](https://github.com/futa-t/fuver/blob/main/LICENSE)

## プロジェクトバージョン管理ツール
自動でバージョンとかビルド番号をインクリメントしたりするやつ

pre-commitに追加してみたり
```pre-commit
#!/bin/sh
fuver increment patch 
NEW_VERSION=$(fuver show version)
if [ -n "$NEW_VERSION" ]; then
    sed -i "s/^version = \".*\"/version = \"$NEW_VERSION\"/" Cargo.toml
    cargo update
    git add Cargo.toml Cargo.lock
fi
```

各種prebuildツールに`fuver invrement build`追加したり

todo
- ~~Cargo.tomlとかpyproject.tomlとかのバージョンも触れる設定つくる~~  
導入先のhookで対応したほうが安全そうなのでやめた

- れどめ整備