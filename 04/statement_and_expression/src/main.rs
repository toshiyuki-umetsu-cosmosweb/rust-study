//! 式(expression) : 値を返すもの。
//! 文(statement) : 処理をするが、値を返さない。

/// アプリケーションのエントリポイント
fn main() {
    let _x = 5u32;

    // {と}で囲まれたのはコードブロック。
    // コードブロックも式(expression)なので、
    // このように書くと _y には 0 が格納される。
    let _y = {
	0
    };
    println!("{}", _y); // 0が出力される。

    // _zには 2 が格納される。
    let _z = {
	1; // こいつは文。
	2 // こいつは式
	// 3;  <- 式の後に文は書けない。書くとコンパイルでエラーになる。
    };
    println!("{}", _z);

    // _undefined1 は () が格納されている。
    // 参照しようとするならば、"{:?}"としないといけない。
    let _undefined1 = {
	12;
    };
    println!("{:?}", _undefined1);

    if _undefined1 == () {
	println!("_undefined1 is empty.");
    }

    1;
    // 0     <- main()は値を返さないので、式を書くとエラーになる。
}
