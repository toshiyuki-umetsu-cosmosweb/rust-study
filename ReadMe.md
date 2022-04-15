# これは何？

Rustを学習するにあたり、作成したコードとメモをおいておくものです。

# ノート

## 開発環境

rustupを使用する。

~~~
curl https://sh.rustup.rs -sSf | sh
~~~

## rustcを使ってコンパイル

~~~
rustc -o ${output} ${source-file}
~~~

## cargoを使う場合


### プロジェクトテンプレートを作る

~~~
cargo new ${directory-name}
~~~
オプションなしだと、directory-nameはパッケージ名の制約がある。例えば数字で開始できない。
パッケージ名を明示的に指定するならば、 * -name {package-name} * を指定する
cargo new で生成したフォルダには.gitと.gitignoreが予め生成されている。
git add で追加しようとすると"does not have a commit checked out"メッセージが出て追加できない。
.gitignore, .gitを生成したくない場合、cargo new のオプションで--vcs noneを付けて実行する。
つまり、
~~~
cargo new --name ${package-name} --vcs none ${directory-name}
~~~
とする。



### プロジェクトをビルドする

~~~
cargo build
~~~

### 実行する

~~~
cargo run
~~~

### 消去する

~~~
cargo clean
~~~

### ドキュメントを生成する。

~~~
cargo doc
~~~

### 生成したドキュメントを表示する。

~~~
cargo doc open
~~~
   

### デバッグ

~~~
rust-lldb {ouput-binary}
~~~

基本的なコマンド

* p {symbol} - 変数の値を表示。
* b {symbol} - ブレークポイントを設定する。
* n - 次の行実行
* c - 継続(終了するか次のbreakまで実行）

# 参考にしたURL

Rust公式
https://doc.rust-lang.org/book/

Rust 入門
https://zenn.dev/mebiusbox/books/22d4c1ed9b0003

Rust by Example 日本語版
https://doc.rust-jp.rs/rust-by-example-ja/index.html

Qiita Rustのドキュメンテーションコメントについて学ぶ
https://qiita.com/simonritchie/items/87d3743e138763ff3e85
