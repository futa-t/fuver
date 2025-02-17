# fuver

[![GitHub license](https://img.shields.io/badge/license-MIT-green.svg)](https://github.com/futa-t/fuver/blob/main/LICENSE)

## プロジェクトバージョン管理ツール
自動でバージョンとかビルド番号をインクリメントしたりするやつ

pre-commitに追加してみたり
```pre-commit
#!/bin/sh
fuver increment version x.x.1
git add fuver.toml
```

各種prebuildツールに`fuver invrement build`追加したり

todo
- Cargo.tomlとかpyproject.tomlとかのバージョンも触れる設定つくる
- れどめ整備
- `x.x.1`みたいな形式だけじゃなくてmajor, minor, patchとかでインクリメントできるようにする
- プレリリースの追加できるようにする
- 上位が更新されたときの下位リセット