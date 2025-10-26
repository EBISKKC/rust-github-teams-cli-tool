# Git Team Stats (gtct)

A fast and beautiful command-line tool for analyzing Git repository statistics across **all branches**, built with Rust.

## Features

- **自動リモート取得**: 実行時に自動的に `git fetch --all` を実行し、最新データを取得
- **全ブランチ対応**: すべてのブランチのコミット履歴を分析
- **Contributor Statistics**: 詳細な貢献者別統計（コミット数、追加/削除行数）
- **Time-based Analysis**: 時間帯別・曜日別のコミットパターン可視化
- **File Change Frequency**: 最も頻繁に変更されるファイルのランキング
- **Comprehensive Reports**: 週次/月次レポートの一括生成
- **Beautiful Output**: 色付きテーブルとチャートで見やすい表示
- **Fast Performance**: Rustによる高速処理
- **.env設定対応**: リポジトリパスやチームフィルターを設定ファイルで管理

## Installation

### グローバルインストール（推奨）

```bash
cargo install --path .
```

これで `gtct` コマンドがどこからでも使えるようになります。

### 開発ビルド

```bash
cargo build --release
```

バイナリは `target/release/gtct` に生成されます。

## Quick Start

```bash
# 月次レポートを表示（最もおすすめ）
gtct report --period monthly

# チームサマリーを表示
gtct summary

# コントリビューター統計（全期間）
gtct contributors --days 0

# ヘルプを表示
gtct --help
```

## Configuration

`.env` ファイルで設定を管理できます。

### 設定ファイルのセットアップ

1. サンプルファイルをコピー:

```bash
cp .env.example .env
```

2. `.env` を編集:

```bash
# リポジトリパス（必須ではない）
GIT_REPO_PATH=/Users/you/Desktop/work/your-project

# チームフィルター（特定のメールドメインのみ表示）
# GIT_TEAMS=company.com,partner.org

# デフォルト分析期間（日数）
DEFAULT_DAYS=30
```

### 設定オプション

#### `GIT_REPO_PATH`
分析対象のGitリポジトリパスを指定。
- 未設定: カレントディレクトリまたは `--repo` 引数を使用
- 例: `GIT_REPO_PATH=/Users/john/projects/myapp`

#### `GIT_TEAMS`
メールドメインやパターンでコントリビューターをフィルタリング（カンマ区切り）。
- 未設定: すべてのコントリビューターを表示
- 例:
  - `GIT_TEAMS=company.com` → @company.comのみ
  - `GIT_TEAMS=team1,team2` → "team1"か"team2"を含むメール
  - `GIT_TEAMS=acme.com,partner.org` → 複数ドメイン

#### `DEFAULT_DAYS`
コマンド実行時のデフォルト分析期間。
- 未設定: コマンド別のデフォルトを使用
- 例:
  - `DEFAULT_DAYS=7` → 直近1週間
  - `DEFAULT_DAYS=30` → 直近1ヶ月
  - `DEFAULT_DAYS=90` → 直近3ヶ月
  - `DEFAULT_DAYS=0` → 全期間

### 優先順位

設定は以下の順序で適用されます（後の設定が優先）:
1. `.env` ファイルの設定
2. コマンドライン引数

例: `.env` に `DEFAULT_DAYS=30` があっても、`gtct summary --days 7` を実行すると7日間が使用されます。

## Usage

### 基本コマンド

#### チームサマリー

```bash
gtct summary
```

デフォルトは30日間。期間を変更するには:

```bash
gtct summary --days 7    # 直近1週間
gtct summary --days 0    # 全期間
```

#### コントリビューター統計

```bash
gtct contributors
```

詳細な貢献者別統計を表示:
- コミット数
- 追加行数・削除行数
- 変更ファイル数
- ネット変更量（追加-削除）

```bash
gtct contributors --days 0  # 全期間のデータ
```

#### 時間帯別分析

```bash
gtct time-analysis
```

以下を可視化:
- 24時間別のコミット分布
- 曜日別のコミット分布

チームの開発パターンを把握するのに便利です。

#### ファイル変更頻度

```bash
gtct files --top 20
```

最も頻繁に変更されたファイルをランキング表示。

```bash
gtct files --top 10 --days 7  # 直近1週間のTop 10
```

#### レポート生成

```bash
# 週次レポート（直近7日間）
gtct report --period weekly

# 月次レポート（直近30日間）
gtct report --period monthly
```

レポートには以下がすべて含まれます:
- チームサマリー
- コントリビューター統計
- 時間帯別分析
- 最も変更されたファイルTop 10

### 応用例

#### 特定のリポジトリを分析

```bash
gtct --repo /path/to/another/repo summary
```

#### 四半期レポート

```bash
gtct contributors --days 90
```

#### チームメンバーのみ表示

`.env` で `GIT_TEAMS` を設定するか:

```bash
# .env
GIT_TEAMS=company.com
```

## Project Structure

```
cl-tool/
├── .env                 # 設定ファイル（.gitignore済み）
├── .env.example         # 設定テンプレート
├── src/
│   ├── main.rs          # エントリーポイント
│   ├── cli.rs           # CLI定義
│   ├── config.rs        # .env設定管理
│   ├── stats/           # 統計解析モジュール
│   │   ├── mod.rs
│   │   ├── contributor.rs
│   │   ├── time.rs
│   │   └── files.rs
│   └── display/         # 表示フォーマット
│       ├── mod.rs
│       └── format.rs
├── Cargo.toml
└── README.md
```

## Development

### 必須環境

- Rust 1.70.0以上
- Gitリポジトリ（テスト用）

### セットアップ

```bash
git init
cargo build
```

### コード品質

このプロジェクトは厳格なコード品質基準に従っています:

#### コードフォーマット

```bash
cargo fmt
```

#### Lint実行

```bash
cargo clippy
```

#### テスト実行

```bash
cargo test
```

### 開発ワークフロー

コード変更時は必ず以下の順序で実行:

1. コード編集
2. フォーマット: `cargo fmt`
3. Lint: `cargo clippy`
4. ビルド: `cargo build`
5. テスト: `cargo test` (該当する場合)
6. 再インストール: `cargo install --path .`

詳細は [.claude/rule.md](.claude/rule.md) を参照。

## 設定ファイル

- **.rustfmt.toml**: コードフォーマットルール
- **clippy.toml**: Linter設定
- **.editorconfig**: エディタ設定

## Dependencies

- **clap**: コマンドライン引数解析
- **git2**: Gitリポジトリ操作
- **colored**: ターミナルカラー出力
- **comfy-table**: 美しいテーブル表示
- **chrono**: 日時処理
- **dotenvy**: .env ファイル読み込み
- **serde**: シリアライゼーション

## 出力例

### コントリビューター統計

```
📊 Contributor Statistics

┌────────────────────────┬─────────┬───────────┬───────────┬───────┬───────┐
│ Contributor            ┆ Commits ┆ Additions ┆ Deletions ┆ Files ┆ Net   │
╞════════════════════════╪═════════╪═══════════╪═══════════╪═══════╪═══════╡
│ John <john@ex.com>     ┆      63 ┆     29894 ┆     10995 ┆   760 ┆+18899 │
│ Alice <alice@ex.com>   ┆      31 ┆      6134 ┆     16314 ┆   371 ┆-10180 │
└────────────────────────┴─────────┴───────────┴───────────┴───────┴───────┘
```

### 時間帯別分析

```
⏰ Time-based Commit Analysis

Commits by Hour:
22:00 │ ████████████████████████████████ (20)
23:00 │ ██████████████████████████████████████████████████ (32)

Commits by Day of Week:
Thu │ ██████████████████████████████████████████████████ (35)
Fri │ ████████████████████████████████ (27)
```

## Use Cases

- **チームレトロスペクティブ**: スプリント期間の活動分析
- **コードレビュー**: 頻繁に変更されるファイルの特定
- **パフォーマンス分析**: チームの生産性が高い時間帯の把握
- **オンボーディング**: 新メンバーへの貢献パターン提示
- **管理レポート**: 週次/月次の統計データを経営陣に報告

## Performance

Rustによる最適化で高速:
- 10,000+コミットのリポジトリを数秒で分析
- 低メモリフットプリント
- LTOとストリッピングによる最適化バイナリ

## Technical Notes

### 全ブランチ対応

このツールは `git log --all` 相当の処理を行い、**すべてのブランチ**のコミットを分析します。
特定のブランチのみを分析したい場合は、そのブランチにチェックアウトしてから `--repo` でそのパスを指定してください。

### 自動リモート取得

**重要**: このツールは実行時に自動的に `git fetch --all` を実行します。

実行されること:
- ✅ すべてのリモートから最新データを自動取得
- ✅ リモートブランチの最新コミットを分析に含める
- ✅ チーム全体の最新の活動を反映

これにより、常に最新のリポジトリ状態で統計を取得できます。

```bash
# 実行例
gtct summary

# 出力:
# 🔄 Fetching latest data from remotes... ✓ Fetched 1 remote(s)
# 📈 Team Summary
# ...
```

**注意**: fetch処理により、初回実行時は若干時間がかかる場合があります（数秒〜数十秒）。

## Future Enhancements

将来的に追加予定の機能:
- JSONやCSVへのエクスポート
- GitHub/GitLab API連携
- より多様な可視化オプション
- カスタム日付範囲指定
- ブランチ間比較
- 時系列トレンド分析

## License

MIT

## Contributing

コントリビューション歓迎！以下を確認してください:
- `cargo fmt` でフォーマット済み
- `cargo clippy` で警告なし
- `cargo build` でビルド成功
- 適切なテストを含む

## Author

Rustでチーム開発ワークフローのために作成。

## Command Reference

```bash
gtct [OPTIONS] <COMMAND>

Commands:
  contributors   コントリビューター統計を表示
  time-analysis  時間帯別のコミット分析を表示
  files          ファイル変更頻度ランキングを表示
  report         包括的なレポートを生成
  summary        チーム全体のサマリーを表示
  help           ヘルプを表示

Options:
  -r, --repo <REPO>  Gitリポジトリのパス [default: .]
  -h, --help         ヘルプを表示
  -V, --version      バージョンを表示
```
