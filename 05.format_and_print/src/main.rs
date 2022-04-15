//! フォーマット指定して出力するサンプル
//! 書式の詳細はstd:fmtのドキュメントを読むのが良い。

/// アプリケーションのエントリポイント
fn main() {
    // ところで、{や}を出力したい場合、2つ重ねればいいようだ
    println!("{{  }}");
    {
	// {}はコンパイラが自動的に置き換えてくれる。
	println!("{}", 12345678);
	println!("{}", 1234.5678);
	println!("{}", "string");
    }
    {
	// 特にコメントに書いてなかったけど、
	// ローカルスコープの変数はアンダースコア(_)で始めないと
	// ワーニングが出る。優しい！
	let _a = "arg1";
	let _b = "arg2";
	println!("{{0}}と指定すると{0}, {{1}}と指定すると{1}", _a, _b);
	println!("{{1}}と指定すると{1}, {{0}}と指定すると{0}", _a, _b);
	//println!("{{2}}と指定するとコンパイルエラー", _a, _b); // コンパイルエラー
    }
    {
	// 名前で指定できるってどういうこと？
	println!("{arg1} {arg2} {arg3}", arg1="引数1", arg2="引数2", arg3=12);
	let _name = "パラメータ名";
	let _value = 12345; 
	println!("{name} {value}", name = _name, value = _value);

    }

    { // : の後に書式を指定することができる。
	let _x = -1234;
	println!("No format : {}", _x);
	println!("{{0:b}}: {0:b}", _x);
	println!("{{0:e}}: {0:e}", _x);
	println!("{{0:x}}: {0:x}", _x);
	println!("{{0:o}}: {0:o}", _x);
	println!("{{0:p}}: {0:p}", &_x);
    }

    { // :の後に".{number}"で桁数
	let pi = 3.141592;
	println!("{0:.2}", pi);
    }
    {
	// 桁指定と0埋め
	println!("{0:08}", 24);
	// 
	println!("{0:8}", 24);
    }


}
