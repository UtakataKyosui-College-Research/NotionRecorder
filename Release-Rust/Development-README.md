## clap

### アトリビュートについて
- `#[derive]`や`#[clap]`を対で記述していく
- `struct`のフィールドに、`#[clap(subcommand)]`でサブコマンドであることを明示する
- サブコマンドとして使う`struct`には`#[derive(Subcommand)]`でサブコマンドとしての実装をするように明記

### `#[derive(Parser)]`
Clapでパーサを作成するための大元
`~::parse()`でオブジェクトを作成したらオプションにあたる`struct`を呼び出すことが可能

### `#[clap(name = "",version = "",author = "" ,about = "" ,arg_required_else_help = bool)]`

helpに作成者やCLIのバージョンなどを表示してくれる

### `env!(環境変数名)`
helpに表示させたい内容を環境変数に格納しているときに使用

### `#[clap(short = 's', long = "server", value_name = "URL", default_value = "localhost:3000")]`
- short：コマンドのオプションのショートハンド
- long：正式名称の指定
- value_name：どういう値を入れるかを示す文
- default_value = デフォルト値の設定

### chrono

[Rustで日時を扱う](https://qiita.com/fujitayy/items/ae6175118cbed7134594)
```rust
use chrono::{Utc,Local,DateTime,Date};

fn main(){
    // UTCで現在時刻取得
    let utc_datetime: DateTime<Utc> = Utc::now();
    // UTCで今日の日付取得
    let utc_date: Date<Utc> = Utc::today();

    // 日本時間の現在時刻取得
    let local_datetime: DateTime<Local> = Local::now();
    // 日本時間の今日の日付取得
    let local_date: Date<Local> = Local::today();

}
```