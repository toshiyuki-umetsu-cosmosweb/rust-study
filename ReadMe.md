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
cargo new ${package-name}
~~~
package-nameはパッケージ名の制約がある。数字で開始できない。
cargo new で生成したフォルダには.gitと.gitignoreが予め生成されている。git add で追加しようとすると"does not have a commit checked out"メッセージが出て追加できない。


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


   


# 参考にしたURL

Rust公式
https://doc.rust-lang.org/book/

Rust 入門
https://zenn.dev/mebiusbox/books/22d4c1ed9b0003

