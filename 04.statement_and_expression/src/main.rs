//! 式(expression) : 値を返すもの。
//! 文(statement) : 処理をするが、値を返さない。

/// アプリケーションのエントリポイント
fn main() {
    // これは文。mainの返値ではない。というか、mainは値を返さない。
    let _x = 5u32;

    // {と}で囲まれたのはコードブロック。
    // コードブロックも式(expression)なので、
    // このように書くと _y には 0 が格納される。
    let _y = {
	0
    }; // let _y = {}; という文
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
    };

    let _undefined2 = {
	{
	    // コードブロック
	};
	{
	    // コードブロック
	};
	// _undefined2には式がないので()になる。
    };

    let _undefined3 = {
	println!("println!を式にして、結果を返してみる")
    };
    println!("{:?}", _undefined3); // ()が返ってる。

    1; // これは文。意味は無い。

    // 0     <- main()は値を返さないので、式を書くとエラーになる。
}
