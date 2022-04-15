//! ドキュメンテーションコメント練習用。
//! 先頭にある//!で始まるところがソースファイルのコメントにつくようだ。  
//!   
//! コメントではマークダウンが使える。  
//! 生成するドキュメントで改行を入れたい場合、行末にスペースを2ついれる。  
//! アスタリスクはバックスラッシュが必要。つまり、\\\*とする。  
//! *斜体は"\*"で囲む。  
//! **太字は"\*\*"で囲む  
//!   
//! * リスト項目は"* "で開始する。
//! * リスト項目は"* "で開始する。
//!
//! - リスト項目は"- "で開始する。
//! - リスト項目は"- "で開始する。
//!   
//! リンク  
//! [Rust公式](https://doc.rust-lang.org/book/)
//! 
//! # 見出し1は"# "で開始する。 
//!  
//! ## 見出し2は"## "で開始する。
//!   
//! ### 見出し3は"### "で開始する。
//!   
//! ~~取り消し線は~~ "~~"で囲む。  
//!  
//! > 引用は"> "で開始する。
//!   
//!
//! 水平線は"---"で表記する。
//! ---
//! 
//! インラインコードは"`"で囲む。： `println!("Hello!");"`
//! 
//! コードブロックは"```"で囲む。  
//! ```
//! println!("Hello world!");
//! ```
//!  
//! データテーブル表記もできる。  
//! テーブルの見出し行の前には空行を入れないとだめっぽい。  
//! 
//! |カラム1|カラム2|
//! |---|---|
//! |日本白色種|ネザーランドドワーフ|
//! |ミニレッキス|ポーランドロップ|

//
/*
 * ブロックコメント
 */

 // 関数コメントサンプル。 
 /// 関数コメントは"///"で開始する。
 /// このプロジェクトのエントリーポイント。
 /// Hello, world!を出力します。
fn main() {
    // ラインコメント
    println!("Hello, world!");
}
/*
//! ソースの途中にあるドキュメンテーションコメントはエラーになる！！
//! さすがrust,やさしい。
*/


pub mod sample_module {
    //! サンプルモジュール

    /// 2を返す。
    pub fn get_two() -> i32 {
	2
    }

    /// 3を返す。
    pub fn get_three() -> i32 {
	3
    }


    /// 2つの数値を加算した結果を返す。
    /// 
    /// * `x` - 数値
    /// * `y` - 数値。
    ///
    /// Note : 32bit整数で処理しているので使う際にはオーバーフローに注意すること。
    pub fn sum_two_values(x: i32, y: i32) -> i32 {
	x + y
    }
}