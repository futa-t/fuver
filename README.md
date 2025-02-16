# fuver
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